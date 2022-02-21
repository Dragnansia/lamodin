//! Blocking version of api

use reqwest::{blocking::Client, header::USER_AGENT};

use super::{Version, PROTONGE_API_URL};
use crate::error::Error;

/// Return a vec of all available ProtonGE version on github
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
    println!("{}", versions.unwrap().len())
}
