use std::vec::Vec;
use std::result::Result;
use std::fs::File;
use std::io::BufReader;
use std::str::*;

extern crate xml;
use self::xml::reader::{EventReader, XmlEvent};
use self::xml::name::OwnedName;
use self::xml::attribute::OwnedAttribute;

use ::types::*;
use ::tile_set::TileSet;

pub struct Map {
//    version: Version,
    orientation: Option<Orientation>,
    render_order: Option<RenderOrder>,
//
    tile_count: Vector2u,
    tile_size: Vector2u,
//
    hex_side_length: f32,
    stagger_axis: Option<StaggerAxis>,
    stagger_index: Option<StaggerIndex>,
//
    background_color: Option<Color>,
//
    working_directory: String,
//
    tile_sets: Vec<TileSet>,
//    layers: std::vector<Layer::Ptr>,
//    properties: std::vector<Property>,

}


impl Map {
    fn process_map(&mut self, attributes: &Vec<OwnedAttribute>) {
        for ref attr in attributes {
            match &attr.name.local_name as &str {
                "orientation" => {
                    self.orientation = Some(Orientation::from_str(attr.value.as_ref()).unwrap());
                }
                "renderorder" => {
                    self.render_order = Some(RenderOrder::from_str(attr.value.as_ref()).unwrap());
                }
                "width" => {
                    self.tile_count.x = attr.value.parse().unwrap();
                }
                "height" => {
                    self.tile_count.y = attr.value.parse().unwrap();
                }
                "tilewidth" => {
                    self.tile_size.x = attr.value.parse().unwrap();
                }
                "tileheight" => {
                    self.tile_size.y = attr.value.parse().unwrap();
                }
                "hexSideLength" => {
                    self.hex_side_length = attr.value.parse().unwrap();
                }
                "staggeraxis" => {
                    self.stagger_axis = Some(StaggerAxis::from_str(attr.value.as_ref()).unwrap());
                }
                "staggerindex" => {
                    self.stagger_index = Some(StaggerIndex::from_str(attr.value.as_ref()).unwrap());
                }
                "backgroundcolor" => {
                    self.background_color = Some(Color::from_str(&attr.value).unwrap());
                }
                &_ => {}
            }
        }

        let is_staggered_or_hex = match self.orientation {
            Some(Orientation::Staggered) => true,
            Some(Orientation::Hexagonal) => true,
            _ => false,
        };

        if is_staggered_or_hex && (self.stagger_axis.is_some() || self.stagger_index.is_some()) {
            panic!();
        }
    }

    fn default() -> Self {
        Map {
            orientation: None,
            render_order: None,
            tile_count: Default::default(),
            tile_size: Default::default(),
            hex_side_length: 0.0,
            stagger_axis: None,
            stagger_index: None,
            background_color: None,
            working_directory: String::new(),
            tile_sets: Vec::new(),
        }
    }

    pub fn new(path: &str) -> Result<Map, ::std::io::Error> {
        let file = File::open(path)?;
        let file = BufReader::new(file);

        let mut parser = EventReader::new(file);

        let mut map: Map = Map::default();

        while let Ok(ref e) = parser.next() {
            match e {
                &XmlEvent::StartElement {ref name, ref attributes, ref namespace} => {
                    println!("StartElement name={:?}, attr={:?}, namespace=((({:?})))", name, attributes, namespace);
                    let ref name = name.local_name;
                    match &name as &str{
                        "map" => {
                            map.process_map(&attributes);
                        }
                        "tileset" => {
                            map.tile_sets.push(TileSet::new(attributes, &mut parser));
                        }
                        "layer" => {

                        }
                        "objectgroup" => {

                        }
                        "imagelayer" => {

                        }
                        "properties" => {

                        }
                        _ => {}
                    }
                }
                &XmlEvent::EndElement {ref name} => {
                    println!("end = {}", name);
                }
                &XmlEvent::StartDocument{ref version, ref encoding, ref standalone} => {}
                &XmlEvent::EndDocument => {}
                &XmlEvent::ProcessingInstruction{ref name, ref data} => {}
                &XmlEvent::CData(ref ss) => {}
                &XmlEvent::Comment(_) => {}
                &XmlEvent::Characters(_) => {}
                &XmlEvent::Whitespace(_) => {}
            }
        }

        Ok(map)
    }


}