use std::fmt::Debug;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Reaction {
    pub url: String,
    pub total_count: i32,
    #[serde(alias = "+1")]
    pub p1: i32,
    #[serde(alias = "-1")]
    pub l1: i32,
    pub laugh: i32,
    pub hooray: i32,
    pub confused: i32,
    pub heart: i32,
    pub rocket: i32,
    pub eyes: i32,
}
