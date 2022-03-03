use super::Modifier;
use crate::{
    archive::install,
    downloader::{file, Download},
    error::Error,
    launcher::lutris::Lutris,
};
use async_trait::async_trait;
use reqwest::{header::USER_AGENT, Client};
use serde::Deserialize;

const API_RELEASE: &str =
    "https://api.github.com/repos/GloriousEggroll/wine-ge-custom/releases?per_page=100";

#[derive(Debug, Deserialize)]
pub struct WAsset {
    pub id: i64,
    pub name: String,
    pub size: u64,
    pub browser_download_url: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct WVersion {
    pub tag_name: String,
    pub name: String,
    pub prerelease: bool,
    pub assets: Vec<WAsset>,
}

#[async_trait]
impl Modifier<WVersion, WAsset> for Lutris {
    async fn install<D>(&self, data: &WAsset, mut downloader: D) -> Result<(), Error>
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

    async fn versions(&self) -> Result<Vec<WVersion>, Error> {
        let client = Client::new();
        let response = client
            .get(API_RELEASE)
            .header(USER_AGENT, "lamodin")
            .send()
            .await?;
        let text = response.text().await.unwrap_or_default();
        let releases: Vec<WVersion> = serde_json::from_str(&text)?;

        Ok(releases)
    }

    async fn remove(&self, _: WVersion) -> Result<(), Error> {
        todo!()
    }
}
