use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("apps.json.rs");
    let mut f = File::create(&dest_path).unwrap();

    let apps_json_str = include_str!("./apps.json");
    f.write_all(format!("const APPS_JSON_TEXT : &str = r##\"{}\"##;", apps_json_str).as_bytes())
        .unwrap();
}
