extern crate reqwest;
extern crate url;

use futures::future::join_all;
use std::env;
use std::io::{self, Read};
use url::Url;
use wappalyzer::scan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut urls = vec![];
    if args.len() == 1 {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        urls.extend(strings_to_urls(buffer));
    } else {
        urls.push(Url::parse(&String::from(&args[1]))?);
    }

    let futures = urls
        .into_iter()
        .map(|url| async move { scan(&url) })
        .collect::<Vec<_>>();
    let results = join_all(futures).await;
    for res in results {
        if let Ok(output) = serde_json::to_string(&res) {
            println!("{}", output);
        }
        // if let Ok(output) = match res {
        //     Ok(info) => serde_json::to_string(&info),
        //     Err(err) => serde_json::to_string(&err),
        // } {
        //     println!("{}", output);
        // }
    }
    Ok(())
}

fn strings_to_urls(domains: String) -> Vec<Url> {
    domains
        .split_terminator('\n')
        .map(|s| Url::parse(s))
        .filter_map(Result::ok)
        .collect()
}
