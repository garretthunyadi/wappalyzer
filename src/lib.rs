#[macro_use]
extern crate lazy_static;

pub mod wapp;

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use url::Url;
use wapp::{RawData, Tech};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Analysis {
    pub url: String,
    pub result: Result<Vec<Tech>, String>,
}

/// Possible Errors in the domain_info lib
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WappError {
    Fetch(String),
    Analyze(String),
    Other(String),
}

impl fmt::Display for WappError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WappError::Fetch(err) => format!("Fetch/{}", err),
                WappError::Analyze(err) => format!("Analyze/{}", err),
                WappError::Other(err) => format!("Other/{}", err),
            }
        )
    }
}

impl std::convert::From<std::io::Error> for WappError {
    fn from(err: std::io::Error) -> Self {
        WappError::Other(err.to_string())
    }
}
impl From<&dyn std::error::Error> for WappError {
    fn from(err: &dyn std::error::Error) -> Self {
        WappError::Other(err.to_string())
    }
}
// the trait `std::convert::From<page::reqwest::Error>` is not implemented for `WappError`
impl From<reqwest::Error> for WappError {
    fn from(err: reqwest::Error) -> Self {
        WappError::Other(err.to_string())
    }
}
// the trait `std::convert::From<std::str::Utf8Error>` is not implemented for `WappError`
impl From<std::str::Utf8Error> for WappError {
    fn from(err: std::str::Utf8Error) -> Self {
        WappError::Other(err.to_string())
    }
}

async fn fetch(url: Url) -> Result<Arc<wapp::RawData>, WappError> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await.unwrap();
    let mut cookies = vec![];
    {
        let cs: std::vec::Vec<reqwest::cookie::Cookie<'_>> = res.cookies().collect::<Vec<_>>();
        for c in cs {
            cookies.push(wapp::Cookie {
                name: String::from(c.name()),
                value: String::from(c.value()),
            });
        }
    }

    let status_code = res.status().to_string();
    if !res.status().is_success() {
        return Err(WappError::Fetch(format!(
            "Non-200 status code: {}",
            status_code
        )));
    }
    let headers = res.headers().clone();
    let html_string = res.text().await?;
    let parsed_html = Html::parse_fragment(&html_string);
    let selector = Selector::parse("meta").unwrap();
    let mut script_tags = vec![];
    for js in parsed_html.select(&Selector::parse("script").unwrap()) {
        script_tags.push(js.html());
    }

    // Note: using a hashmap will not support two meta tags with the same name and different values,
    // though I'm not sure if that's legal html.
    let mut meta_tags = HashMap::new();
    for meta in parsed_html.select(&selector) {
        if let (Some(name), Some(content)) =
            (meta.value().attr("name"), meta.value().attr("content"))
        {
            // eprintln!("META {} -> {}", name, content);
            meta_tags.insert(String::from(name), String::from(content));
        }
    }
    let raw_data = Arc::new(RawData {
        headers,
        cookies,
        meta_tags,
        script_tags,
        html: html_string,
    });

    Ok(raw_data)
}

pub async fn scan(url: Url) -> Analysis {
    let url_str = String::from(url.as_str());
    match fetch(url).await {
        Ok(raw_data) => {
            let analysis = wapp::check(raw_data).await;
            Analysis {
                url: url_str,
                result: Ok(analysis),
            }
        }
        Err(err) => Analysis {
            url: url_str,
            result: Err(err.to_string()),
        },
    }
}
