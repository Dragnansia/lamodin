use super::Modifier;
use crate::{
    archive::install,
    downloader::{file, Download},
    error::Error,
    launcher::steam::Steam,
};
use async_trait::async_trait;
use reqwest::{blocking::Client, header::USER_AGENT};
use serde::Deserialize;

const API_RELEASE: &str =
    "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases?per_page=100";

#[derive(Debug, Deserialize)]
pub struct PAsset {
    pub id: i64,
    pub name: String,
    pub size: u64,
    pub browser_download_url: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct ProtonGE {
    pub tag_name: String,
    pub name: String,
    pub prerelease: bool,
    pub assets: Vec<PAsset>,
}

#[async_trait]
impl Modifier<ProtonGE, PAsset> for Steam {
    async fn install<D>(&self, data: &PAsset, mut downloader: D) -> Result<(), Error>
    where
        D: Download + Send,
    {
        let cache_path = dirs::cache_dir()
            .ok_or("No cache dir")?
            .to_str()
            .ok_or("No cache dir format")?
            .to_string();
        let temp_file = format!("{}/{}", cache_path, data.name);

        file(&data.browser_download_url, &temp_file, &mut downloader).await?;
        install(&temp_file, &self.modifier_path)?;

        Ok(())
    }

    fn versions(&self) -> Result<Vec<ProtonGE>, Error> {
        let client = Client::new();
        let response = client
            .get(API_RELEASE)
            .header(USER_AGENT, "lamodin")
            .send()?;
        let text = response.text().unwrap_or_default();
        let releases: Vec<ProtonGE> = serde_json::from_str(&text)?;

        Ok(releases)
    }

    fn remove(&self, _: ProtonGE) -> Result<(), Error> {
        todo!()
    }
}

#[test]
fn proton_versions() {
    let steam = Steam::new().unwrap();
    let versions = steam.versions();

    assert!(versions.is_ok());

    println!("Versions ProtonGE Count: {}", versions.unwrap().len());
}
