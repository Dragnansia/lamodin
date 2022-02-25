pub mod proton;

use crate::{downloader::Download, error::Error};
use async_trait::async_trait;

#[async_trait]
pub trait Modifier<T, A> {
    /// Install current T version on launcher
    async fn install<D>(&self, data: A, download: D) -> Result<(), Error>
    where
        D: Download + Send;

    /// Return a list of all
    fn versions(&self) -> Result<Vec<T>, Error>;

    /// Remove a specifique version of T
    fn remove(&self, data: T) -> Result<(), Error>;
}
