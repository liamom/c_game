extern crate tmx_parser;
use tmx_parser::Map;

use std::env;

fn rec(i: i32)->i32 {
    match i {
        0 => {1}
        _ => {i+rec(i-1)}
    }
}

fn main() {
    let value = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = value + "/../tmx_parser/examples/demo.tmx";

    println!("running with {}", path);

    let map = Map::new(path.as_ref()).unwrap();

    println!("rec={}", rec(5));
    println!("Hello, world!");
}
