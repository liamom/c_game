extern crate tmx_parser;

use tmx_parser::map::Map;
use std::env;
use tmx_parser::TileLayer;

#[test]
fn test_map() {
    let mut tmx_path = String::new();
    if let Some(path) = env::args().next() {
        if let Some(index) = path.rfind("target") {
            tmx_path = path[0..index].to_string() + "assets/tmx/untitled.tmx";
        }
    }

    assert!(!tmx_path.is_empty());

    println!("running with {}", tmx_path);

    let map = Map::new(tmx_path.as_ref()).unwrap();

    let ref sets = map.tile_sets;
    assert_eq!(sets.len(), 3);
    assert_eq!(sets[0].name, "terrain1");
    assert_eq!(sets[1].name, "");
    assert_eq!(sets[1].source, "modern_cities.tsx");
    assert_eq!(sets[2].name, "");
    assert_eq!(sets[2].source, "medieval_cities.tsx");


    let ref layer1 = map.layers[0];
    assert_eq!(layer1.layer.name, "Tile Layer 1");
}