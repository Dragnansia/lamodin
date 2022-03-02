pub mod proton;
pub mod wine;

use crate::{downloader::Download, error::Error};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GAsset {
    pub id: i64,
    pub name: String,
    pub size: u64,
    pub browser_download_url: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct GVersion {
    pub tag_name: String,
    pub name: String,
    pub prerelease: bool,
    pub assets: Vec<GAsset>,
}

#[async_trait]
pub trait Modifier<T, A> {
    /// Install current T version on launcher
    async fn install<D>(&self, data: &A, download: D) -> Result<(), Error>
    where
        D: Download + Send;

    /// Return a list of all
    async fn versions(&self) -> Result<Vec<T>, Error>;

    /// Remove a specifique version of T
    async fn remove(&self, data: T) -> Result<(), Error>;
}
