use std::vec::Vec;
use ::types::*;


use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;
use std::io::BufReader;
use std::fs::File;
use ::tile_error::TMXError;

pub struct Tile {
    id: u32,
    flip_flags: u8,
}

impl Tile {
    fn new(gid: u32) -> Self {
        Tile{
           id: gid,
            flip_flags: 0,
        }
    }
}

pub struct Layer {
    pub name: String,
    pub opacity: f32,
    pub visible: bool,
    pub offset: Vector2i,
    pub properties: Vec<Property>,
}


pub struct TileLayer {
    pub layer: Layer,
    pub tiles: Vec<Tile>,
    pub tile_count: usize,
}

impl TileLayer {
    pub fn new(size: usize, attrs: &Vec<OwnedAttribute>, mut parser: &mut EventReader<BufReader<File>>) -> Result<Self, TMXError> {
        let tiles: Vec<Tile>;

        let mut name = String::new();
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut opacity: f32 = 1.0;
        let mut visible: bool = true;

        for attr in attrs {
            match &attr.name.local_name as &str {
                "name"    => {name     = attr.value.clone();}
                "opacity" => {opacity  = attr.value.parse()?;}
                "visible" => {visible  = attr.value.parse()?;}
                "offsetx" => {x        = attr.value.parse()?;}
                "offsety" => {y        = attr.value.parse()?;}
                "width" => {}
                "height" => {}
                name => { panic!("unexpected element = ({})", name); }
            }
        }

        let layer = Layer {
            name,
            opacity,
            visible,
            offset: Vector2i {
                x,
                y,
            },
            properties: Vec::new(),
        };

        'outer: loop {
            match parser.next() {
                Ok(XmlEvent::StartElement {ref name, ref attributes, ..}) => {
                    if name.local_name != "data" {
                        panic!("error");
                    }

                    for attr in attributes {
                        match &attr.name.local_name as &str {
                            "base64" => {
                                tiles = TileLayer::parse_base64(&mut parser);
                                break 'outer;
                            }
                            "csv" => {
                                tiles = TileLayer::parse_csv(&mut parser);
                                break 'outer;
                            }
                            &_ => {}
                        }
                    }

                    tiles = TileLayer::parse_xml(&mut parser);
                    break 'outer;
                }
                Ok(XmlEvent::Whitespace(_)) => {}
                a => panic!("unrecognized layer data = ({:?})", a),
            }
        }

        Ok(TileLayer {
            layer,
            tiles,
            tile_count: size,
        })
    }

    fn parse_csv(_parser: &mut EventReader<BufReader<File>>) -> Vec<Tile>{
        unimplemented!("base64 not implemented"); //todo
    }

    fn parse_base64(_parser: &mut EventReader<BufReader<File>>) -> Vec<Tile>{
        unimplemented!("base64 not implemented"); //todo
    }

    fn parse_xml(parser: &mut EventReader<BufReader<File>>) -> Vec<Tile>{
        let mut vec: Vec<Tile> = Vec::new();
        'outer: loop {
            match parser.next() {
                Ok(XmlEvent::StartElement {ref name, ref attributes, ..}) => {
                    match &name.local_name as &str {
                        "tile" => {
                            let gid = attributes.get(0).expect("grid element error");
                            assert_eq!(gid.name.local_name, "gid");
                            vec.push(Tile::new(gid.value.parse().unwrap()));
                        }
                        &_ => {}
                    }
                }
                Ok(XmlEvent::EndElement {ref name}) => {
                    if name.local_name == "data" {
                        break 'outer;
                    }
                },
                Ok(XmlEvent::Whitespace(_)) => {}
                _ => {panic!("parse_xml error")},
            }
        }

        return vec;
    }

}