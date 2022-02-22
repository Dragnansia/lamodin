//! Blocking version of api

use super::{Version, PROTONGE_API_URL};
use crate::error::Error;
use reqwest::{blocking::Client, header::USER_AGENT};

/// Return a vec of all available ProtonGE version on github
///
/// # Exemple
///
/// ```
/// use purs::{api::blocking::version_list, error::Error};
///
/// async fn func() -> Result<(), Error> {
///     let versions = version_list()?;
///     println!("{:?}", versions);
///     Ok(())
/// }
/// ```
pub fn version_list() -> Result<Vec<Version>, Error> {
    let client = Client::new();
    let response = client
        .get(PROTONGE_API_URL)
        .header(USER_AGENT, "purs")
        .send()?;
    let text = response.text().unwrap_or_default();
    let releases: Vec<Version> = serde_json::from_str(&text)?;

    Ok(releases)
}

#[test]
fn list() {
    let versions = version_list();
    assert!(versions.is_ok(), "{}", versions.unwrap_err());
}
