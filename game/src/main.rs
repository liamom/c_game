extern crate tmx_parser;
extern crate rand;

use tmx_parser::Map;

use self::rand::{Rng, ChaChaRng};
mod name_generator;
use name_generator::*;

use std::env;

fn rec(i: i32)->i32 {
    match i {
        0 => {1}
        _ => {i+rec(i-1)}
    }
}

fn main() {
    let mut rng = rand::ChaChaRng::new_unseeded();

    println!("male name={:?}, female name={:?}", Name::male(&mut rng), Name::female(&mut rng));

    let value = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = value + "/../tmx_parser/examples/demo.tmx";

    println!("running with {}", path);

    let map = Map::new(path.as_ref()).unwrap();

    println!("rec={}", rec(5));
    println!("Hello, world!");
}
