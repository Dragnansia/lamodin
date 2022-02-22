pub mod asset;
pub mod author;
pub mod blocking;
pub mod reaction;
pub mod version;

use self::version::Version;
use crate::error::Error;
use reqwest::{header::USER_AGENT, Client};

pub const PROTONGE_API_URL: &str =
    "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases";

/// Return a list of all available ProtonGE version on github
///
/// # Exemple
///
/// ```
/// use purs::{api::version_list, error::Error};
///
/// async fn func() -> Result<(), Error> {
///     let versions = version_list().await?;
///     println!("{:?}", versions);
///     Ok(())
/// }
/// ```
pub async fn version_list<'a>() -> Result<Vec<Version>, Error> {
    let client = Client::new();
    let response = client
        .get(format!("{}?per_page=100", PROTONGE_API_URL))
        .header(USER_AGENT, "purs")
        .send()
        .await?;
    let text = response.text().await?;
    let releases: Vec<Version> = serde_json::from_str(&text)?;

    Ok(releases)
}

#[test]
fn list() {
    let versions = tokio_test::block_on(version_list());
    assert!(versions.is_ok(), "{}", versions.unwrap_err());
}
