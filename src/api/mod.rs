pub mod asset;
pub mod author;
pub mod blocking;
pub mod reaction;
pub mod version;

use reqwest::{header::USER_AGENT, Client};

use self::version::Version;
use crate::error::Error;

pub const PROTONGE_API_URL: &str =
    "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases";

/// Return a vec of all available ProtonGE version on github
pub async fn version_list<'a>() -> Result<Vec<Version>, Error> {
    let client = Client::new();
    let response = client
        .get(format!("{}?per_page=100", PROTONGE_API_URL))
        .header(USER_AGENT, "purs")
        .send()
        .await?;
    let text = response.text().await.unwrap_or_default();
    let releases: Vec<Version> = serde_json::from_str(&text)?;

    Ok(releases)
}
