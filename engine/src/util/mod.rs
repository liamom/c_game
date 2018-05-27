use std::env;
use std::path::Path;
use std::path::PathBuf;

pub fn get_asset(path: &str) -> PathBuf {
    let base = get_assets_path();
    [&base, "assets", path].iter().collect()
}

pub fn get_assets_path() -> String {
    if let Some(path) = env::args().next()  {
        if let Some(index) = path.rfind("target") {
            return path[0..index].to_string();
        }
    }

    panic!("Could not get assets path");
}