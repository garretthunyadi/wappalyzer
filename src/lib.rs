#[macro_use]
extern crate lazy_static;

pub mod wapp;

use serde::{Deserialize, Serialize};
use url::Url;
use wapp::Tech;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Analysis {
    pub url: String,
    pub result: Result<Vec<Tech>, String>,
}

pub fn scan(url: &Url) -> Analysis {
    Analysis {
        url: String::from(url.as_str()),
        result: Ok(vec![]),
    }
}
