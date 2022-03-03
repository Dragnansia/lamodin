pub mod proton;
pub mod wine;

use crate::{downloader::Download, error::Error};
use async_trait::async_trait;
use serde::Deserialize;

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
