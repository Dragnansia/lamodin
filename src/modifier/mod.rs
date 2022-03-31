pub mod proton;
pub mod wine;

use crate::{downloader::Download, error::Error, launcher::Launcher};
use async_trait::async_trait;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Modifier {
    pub name: String,
    pub path: String,
}

#[async_trait]
pub trait ModifierImpl<T, A>
where
    Self: Launcher,
{
    /// Install current T version on launcher
    async fn install<D>(&self, data: &A, download: D) -> Result<(), Error>
    where
        D: Download + Send;

    /// Return a list of all
    async fn versions() -> Result<Vec<T>, Error>;

    /// Remove a specifique version of T
    async fn remove(&self, data: T) -> Result<(), Error>;
}
