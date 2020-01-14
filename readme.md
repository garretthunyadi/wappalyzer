
# wappalyzer

This lib will fetch a url and identify the techs using the definitions from the [Wappalyzer](https://github.com/AliasIO/Wappalyzer/) project.

The Rust version is somewhat limited in comparison to the JS (main) library as we are not running the rules from within a headless browser, but only against the initially-returned html from the main page.  It's more like the [Go port](https://github.com/rverton/webanalyze).

The tradeoff is speed and efficiency as this can run much more efficiently against large sets of webpages.

```rust
let url = Url::parse(&String::from("http://google.com"))?;
let res = wappalyzer::scan(url).await;
println!("{:?}", res);

// Analysis { url: "http://google.com/", result: Ok([Tech { category: "Web Servers", 
// name: "Google Web Server" }, Tech { category: "JavaScript Frameworks", name: "ExtJS" }
//, Tech { category: "JavaScript Libraries", name: "List.js" }]) }
```

Or from the executable
```bash
> cargo run cargo run http://google.com/ | jq
{
  "url": "http://google.com/",
  "result": {
    "Ok": [
      {
        "category": "Web Servers",
        "name": "Google Web Server"
      },
      {
        "category": "JavaScript Libraries",
        "name": "List.js"
      },
      {
        "category": "JavaScript Frameworks",
        "name": "ExtJS"
      }
    ]
  }
}```

or given a list of domains in a file:
```bash
> cat urls.list
http://google.com/
http://bbc.com/
...
http://cnn.com/

> cat urls.list | cargo run
{"url":"http://google.com/","result":{"Ok":[{"category":"JavaScript Frameworks","name":"ExtJS"},{"category":"Web Servers","name":"Google Web Server"},{"category":"JavaScript Libraries","name":"List.js"}]}}
{"url":"http://bbc.com/","result":{"Ok":[{"category":"Tag Managers","name":"Google Tag Manager"},{"category":"Analytics","name":"Chartbeat"},{"category":"JavaScript Frameworks","name":"React"},{"category":"Cache Tools","name":"Varnish"},{"category":"Web Servers","name":"Apache"},{"category":"Issue Trackers","name":"Atlassian Jira"},{"category":"Analytics","name":"GrowingIO"},{"category":"JavaScript Libraries","name":"List.js"},{"category":"JavaScript Graphics","name":"Chart.js"},{"category":"Analytics","name":"Optimizely"},{"category":"Analytics","name":"Segment"}]}}
{"url":"http://cnn.com/","result":{"Ok":[{"category":"JavaScript Frameworks","name":"ExtJS"},{"category":"JavaScript Frameworks","name":"Twitter Flight"},{"category":"JavaScript Frameworks","name":"Riot"},{"category":"Advertising Networks","name":"Criteo"},{"category":"Analytics","name":"Chartbeat"},{"category":"Analytics","name":"GoSquared"},{"category":"JavaScript Libraries","name":"Moment.js"},{"category":"Ecommerce","name":"Magento"},{"category":"JavaScript Frameworks","name":"React"},{"category":"Cache Tools","name":"Varnish"},{"category":"Analytics","name":"GrowingIO"},{"category":"JavaScript Libraries","name":"List.js"},{"category":"JavaScript Graphics","name":"Chart.js"},{"category":"Comment Systems","name":"Livefyre"},{"category":"Analytics","name":"Optimizely"},{"category":"Analytics","name":"Segment"}]}}
```

## Status
In development.

## TODO

## Notes
