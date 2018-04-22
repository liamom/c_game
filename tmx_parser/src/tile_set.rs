use std::vec::Vec;
use std::string::String;
use ::types::*;
use std::default::Default;

extern crate xml;
use self::xml::reader::{EventReader, XmlEvent};
use self::xml::name::OwnedName;
use self::xml::attribute::OwnedAttribute;
use self::xml::namespace::Namespace;
use std::io::BufReader;
use std::fs::File;

pub enum Property {
    Boolean(String, bool),
    Float(String, f32),
    Int(String, i32),
    String(String, String),
    Colour(String, Color),
//    File(String, File),
    Undef
}

struct Terrain {
    name: String,
    tile_id: u32,
    properties: Vec<Property>,
}

impl Terrain {
    fn new() ->Self{
        Terrain{
            name: String::new(),
            tile_id: u32::max_value(),
            properties: Vec::new(),
        }
    }
}

/*
\brief A frame within an animation
*/
struct Frame {
    tile_id: u32,
    duration: u32,
}

impl Frame {
    fn new() -> Self {Frame{ tile_id: 0, duration: 0 }}
}

/*
\brief a group of frames which make up an animation
*/
struct Animation {
    frames: Vec<Frame>,
}

struct ObjectGroup {

}

struct Tile {
    id :u32,
    terrain_indices: [i32; 4],
    probability: u32,
    properties: Vec<Property>,
    object_group: ObjectGroup,
    image_path: String,
    image_size: Vector2u,
    image_position: Vector2u,
    tile_type: String,
}

impl Tile {
    fn new() -> Self {
        Tile{
            id: 0,
            terrain_indices: [0;4],
            probability: 0,
            properties: Vec::new(),
            object_group: ObjectGroup{},
            image_path: String::new(),
            image_size: Default::default(),
            image_position: Default::default(),
            tile_type: String::new(),
        }
    }
}

pub struct TileSet {
    working_dir: String,
    first_gid: u32,
    source: String,
    name: String,
    tile_size: Vector2u,
    spacing: u32,
    margin: u32,
    tile_count: u32,
    column_count: u32,
    tile_offset: Vector2u,
    properties: Vec<Property>,
    image_path: String,
    transparency_colour: Color,
    has_transparency: bool,
    terrain_types: Vec<Terrain>,
    tiles: Vec<Tile>,
}

impl TileSet {
    pub fn new(attr: &Vec<OwnedAttribute>, parser: &mut EventReader<BufReader<File>>) -> Self{

        loop {
            let e = parser.next();
            match e {
                Ok(XmlEvent::StartElement {ref name, ..}) => {
                    match &name.local_name as &str {
                        "image" => {

                        }
                        _ =>{panic!("unexpected element in tileset")}
                    }
                },
                Ok(XmlEvent::EndElement {ref name}) => {
                    if name.local_name == "tileset" {break;}
                },
                Ok(_) =>{},
                Err(_) => {},
            };
        }
//        let tile_set = xml.get("tileset");
//        match xml {
//
//
//        }
//        xml.
//        match xml {
//
//        }


        TileSet{
            working_dir: String::new(),
            first_gid: 0,
            source: String::new(),
            name: String::new(),
            tile_size: Vector2u::default(),
            spacing: 0,
            margin: 0,
            tile_count: 0,
            column_count: 0,
            tile_offset: Vector2u::default(),
            properties: Vec::new(),
            image_path: String::new(),
            transparency_colour: Color::default(),
            has_transparency: false,
            terrain_types: Vec::new(),
            tiles: Vec::new(),
        }
    }
}