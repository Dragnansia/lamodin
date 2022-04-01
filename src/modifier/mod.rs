pub mod proton;
pub mod wine;

use crate::{downloader::Download, error::Error, launcher::Launcher};
use async_trait::async_trait;

/// This a basic modifier with juste name and path,
/// to be store on struct with `Launcher` trait impl
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Modifier {
    pub name: String,
    pub path: String,
}

/// Used to add `Modifier` type for a `Launcher`
///
/// # Exemple
/// ```
/// use lamodin::{
///     error::Error,
///     modifier::{ModifierImpl, Modifier},
///     downloader::Download,
///     launcher::Launcher,
/// };
/// use async_trait::async_trait;
///
/// // Version information of modifier
/// struct WineVersion;
/// // Asset info to download and install
/// // modifier
/// struct WineAsset;
///
/// struct WineLauncher;
/// impl Launcher for WineLauncher {
///     fn containt_version(&self, name: &str) -> bool {
///         // Check if found modifier by name
///         true
///     }
///
///     fn modifiers(&self) -> Vec<Modifier> {
///         // Return all modifier
///         vec![]
///     }
/// }
///
/// #[async_trait]
/// impl ModifierImpl<WineVersion, WineAsset> for WineLauncher {
///     async fn install<D>(&self, _data: &WineAsset, mut _downloader: D) -> Result<(), Error>
///     where
///         D: Download + Send
///     {
///         // Code to install modifier
///         Ok(())
///     }
///
///     async fn versions() -> Result<Vec<WineVersion>, Error> {
///         // Return all `T` version
///         Ok(vec![])
///     }
///
///     async fn remove(&self, _version: WineVersion) -> Result<(), Error> {
///         // Remove specifique version
///         Ok(())
///     }
/// }
/// ```
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
