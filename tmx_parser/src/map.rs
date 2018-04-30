use std::vec::Vec;
use std::result::Result;
use std::fs::File;
use std::io::BufReader;
use std::str::*;

use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

use ::types::*;
use ::tile_set::TileSet;

use ::tile_layer::*;

pub struct Map {
//    version: Version,
    orientation: Option<Orientation>,
    render_order: Option<RenderOrder>,
    tile_count: Vector2u,
    tile_size: Vector2u,
    hex_side_length: f32,
    stagger_axis: Option<StaggerAxis>,
    stagger_index: Option<StaggerIndex>,
    background_color: Option<Color>,
//    working_directory: String,

    pub tile_sets: Vec<TileSet>,
    pub layers: Vec<TileLayer>,
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
//            working_directory: String::new(),
            tile_sets: Vec::new(),
            layers: Vec::new(),
        }
    }

    pub fn new(path: &str) -> Result<Map, ::std::io::Error> {
        let file = File::open(path)?;
        let file = BufReader::new(file);

        let mut parser = EventReader::new(file);

        let mut map: Map = Map::default();


        while let Ok(e) = parser.next() {
            match e {
                XmlEvent::StartElement {ref name, ref attributes, ref namespace} => {
                    println!("StartElement name={:?}, attr={:?}, namespace=((({:?})))", name, attributes, namespace);
                    let ref name = name.local_name;
                    match &name as &str{
                        "map" => {
                            map.process_map(&attributes);
                        }
                        "tileset" => {
                            map.tile_sets.push(TileSet::new(&attributes, &mut parser));
                        }
                        "layer" => {
                            let size = map.tile_count.x * map.tile_count.y;
                            map.layers.push(TileLayer::new(size as usize, &attributes, &mut parser).unwrap());
                        }
                        "objectgroup" => {
                            panic!("no implemented yet"); //todo
                        }
                        "imagelayer" => {
                            panic!("no implemented yet"); //todo
                        }
                        "properties" => {
                            panic!("no implemented yet"); //todo
                        }
                        _ => {}
                    }
                }
                XmlEvent::EndElement {ref name} => {
                    println!("end = {}", name);
                }
                XmlEvent::StartDocument{..} => {}
                XmlEvent::EndDocument => {
                    println!("end doc");
                    break;
                }
                XmlEvent::ProcessingInstruction{..} => {}
                XmlEvent::CData(..) => {}
                XmlEvent::Comment(_) => {}
                XmlEvent::Characters(_) => {}
                XmlEvent::Whitespace(_) => {}
            }
        }

        Ok(map)
    }


}