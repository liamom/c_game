use std::vec::Vec;
use std::string::String;
use ::types::*;
use std::default::Default;

use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;
use std::io::BufReader;
use std::fs::File;

pub struct Terrain {
    pub name: String,
    pub tile_id: u32,
    pub properties: Vec<Property>,
}

impl Terrain {
    fn new() -> Self{
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

pub struct ObjectGroup {

}

pub struct Tile {
    pub id :u32,
    pub terrain_indices: [i32; 4],
    pub probability: u32,
    pub properties: Vec<Property>,
    pub object_group: ObjectGroup,
    pub image_path: String,
    pub image_size: Vector2u,
    pub image_position: Vector2u,
    pub tile_type: String,
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
    pub working_dir: String,
    pub first_gid: u32,
    pub source: String,
    pub name: String,
    pub tile_size: Vector2u,
    pub spacing: u32,
    pub margin: u32,
    pub tile_count: u32,
    pub column_count: u32,
    pub tile_offset: Vector2u,
    pub properties: Vec<Property>,
    pub image_path: String,
    pub transparency_colour: Color,
    pub has_transparency: bool,
    pub terrain_types: Vec<Terrain>,
    pub tiles: Vec<Tile>,
}

impl TileSet {
    fn empty() -> Self {
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

    pub fn new(attrs: &Vec<OwnedAttribute>, parser: &mut EventReader<BufReader<File>>) -> Self{
        let mut tile_set = TileSet::empty();

        for attr in attrs {
            match &attr.name.local_name as &str {
                "tileset"    => continue,
                "firstgid"   => { tile_set.first_gid    = attr.value.parse().unwrap(); }
                "source"     => { tile_set.source       = attr.value.clone(); }
                "name"       => { tile_set.name         = attr.value.clone(); }
                "tilewidth"  => { tile_set.tile_size.x  = attr.value.parse().unwrap(); }
                "tileheight" => { tile_set.tile_size.y  = attr.value.parse().unwrap(); }
                "tilecount"  => { tile_set.tile_count   = attr.value.parse().unwrap(); }
                "columns"    => { tile_set.column_count = attr.value.parse().unwrap(); }

                &_ => {}
            }
        }

        loop {
            let e = parser.next();
            match e {
                Ok(XmlEvent::StartElement {ref name, ref attributes, ..}) => {
                    match &name.local_name as &str{
                        "image" => {
                            for ref attr in attributes {
                                match &attr.name.local_name as &str {
                                    "source" => tile_set.source = attr.value.clone(),
                                    "width" => tile_set.tile_size.x = attr.value.parse().unwrap(),
                                    "height" => tile_set.tile_size.y = attr.value.parse().unwrap(),
                                    &_ => panic!("unknown attribute on image tag"),
                                }
                            }
                        }
                        "grid" => {
                            println!("grid not handled"); //todo
                        }
                        "tileoffset"   => { panic!("unimplemented behavoir"); } //todo implement
                        "properties"   => { panic!("unimplemented behavoir"); } //todo implement
                        "terraintypes" => { panic!("unimplemented behavoir"); } //todo implement
                        "tile"         => { panic!("unimplemented behavoir"); } //todo implement
                        s              => { panic!("unknown tile set element \"{}\"", s);}
                    }
                },
                Ok(XmlEvent::EndElement {ref name}) => {
                    if name.local_name == "tileset" {break;}
                },
                Ok(_) =>{},
                Err(_) => {},
            };
        }

        if tile_set.tiles.len() != tile_set.tile_count as usize {
            for id in 0..tile_set.tile_count {
                // First, we check if the tile does not yet exist
                for tile in &tile_set.tiles {
                    if tile.id == id {
                        break;
                    }
                }

                let row_index   = id % tile_set.column_count;
                let column_index= id / tile_set.column_count;
                let ref tile_size = tile_set.tile_size;
                let image_pos_x = row_index * tile_size.x;
                let image_pos_y = column_index * tile_size.y;


                let tile = Tile {
                    id: id,
                    terrain_indices: [0;4],
                    probability: 0,
                    properties: Vec::new(),
                    object_group: ObjectGroup{},
                    image_path: tile_set.image_path.clone(),
                    image_size: tile_set.tile_size.clone(),
                    image_position: Vector2u{ x: image_pos_x, y: image_pos_y },
                    tile_type: String::new(),
                };


                tile_set.tiles.push(tile);

            }
        }


        return tile_set;
    }
}