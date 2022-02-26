use super::{GAsset, GVersion, Modifier};
use crate::{
    archive::install,
    downloader::{file, Download},
    error::Error,
    launcher::lutris::Lutris,
};
use async_trait::async_trait;
use reqwest::{blocking::Client, header::USER_AGENT};

const API_RELEASE: &str =
    "https://api.github.com/repos/GloriousEggroll/wine-ge-custom/releases?per_page=100";

#[async_trait]
impl Modifier<GVersion, GAsset> for Lutris {
    async fn install<D>(&self, data: &GAsset, mut downloader: D) -> Result<(), Error>
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

    fn versions(&self) -> Result<Vec<GVersion>, Error> {
        let client = Client::new();
        let response = client
            .get(API_RELEASE)
            .header(USER_AGENT, "lamodin")
            .send()?;
        let text = response.text().unwrap_or_default();
        let releases: Vec<GVersion> = serde_json::from_str(&text)?;

        Ok(releases)
    }

    fn remove(&self, _: GVersion) -> Result<(), Error> {
        todo!()
    }
}
