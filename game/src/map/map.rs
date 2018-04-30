use std::env;
use std::io;
use error::*;
use ::gfx::draw::*;
use util::*;
use piston_window::*;
use tiled;
use std::fs::File;
use std::path::Path;
use log::*;
use std::path::PathBuf;

struct TileSet {
    image: G2dTexture,
    tile_set: tiled::Tileset,
}

impl TileSet {
    fn new(window: &mut PistonWindow, ts: &tiled::Tileset) -> Self {
        let ref image = ts.images[0];
        let full_path: PathBuf =
            [&get_assets_path(), "assets", "tmx", &image.source].iter().collect();

        debug!("creating with tileset {:?}", full_path);


        let tex = Texture::from_path(
            &mut window.factory,
            full_path,
            Flip::None,
            &TextureSettings::new()).unwrap();

        for tile in &ts.tiles {
            println!("tile: {:?}", tile);
        }

        TileSet {
            image: tex,
            tile_set: ts.clone(),
        }
    }

    fn get_source_rect(&self, id: u32) -> [f64; 4] {
        let (w,h) = (self.tile_set.tile_width, self.tile_set.tile_height);

        let image = &self.tile_set.images[0];
        let col_num = (image.width as u32) / w - 1;
        let local_id = id - self.tile_set.first_gid;
        let col = local_id % col_num;
        let row = local_id / col_num;

        let (x,y) = (col * w, row * h);
        return [ x as f64, y as f64, (x + w) as f64, (y + h) as f64];
    }
}

struct TilesetManager{
    tile_sets: Vec<TileSet>,

}

impl TilesetManager {
    fn get_by_id<'a>(&'a self, id: usize) -> &'a TileSet {
        let set = self.tile_sets
            .iter().rev().find(|t| id as u32 >= t.tile_set.first_gid);

        if let Some(t) = set {
            return t;
        } else {
            panic!("value {} not found", id)
        }
    }

    fn draw(&self, id: usize, x: usize, y: usize, c: Context, g: &mut G2d) {
        if id == 0 {
            return;
        }

        let tile_set = self.get_by_id(id);

        let tf = c.transform.trans(
            x as f64 * tile_set.tile_set.tile_width as f64 / 2.0,
            y as f64 * tile_set.tile_set.tile_height as f64 / 2.0,
        );

        let source_rect = tile_set.get_source_rect(id as u32);

        let image = Image::new().src_rect(source_rect).draw(
            &tile_set.image,
            &DrawState::default(),
            tf,
            g
        );
    }
}

pub struct Map{
//    map: TMXMap,
    image: Image,
    texture: G2dTexture,
    map: tiled::Map,
    manager: TilesetManager,
}

impl Map {
    pub fn new(mut window: &mut PistonWindow) -> Result<Self, GameError> {
//        let path = get_asset("tmx/untitled.tmx");
        let img = get_asset("smiley.png");

        let texture = Texture::from_path(
            &mut window.factory,
            img,
            Flip::None,
            &TextureSettings::new()).unwrap();

//        let map_file = File::open(get_asset("tmx/untitled.tmx"))?;
        let path_str = get_asset("tmx/untitled.tmx");
        let path = Path::new(&path_str);
        let map = tiled::parse_file(path)?;

        //tile sets
        let mut tile_sets = Vec::new();
        for tile_set in &map.tilesets {
            tile_sets.push(TileSet::new(&mut window, &tile_set));
        }

        let map = Map {
            image: Image::new(),
            texture,
            map,
            manager: TilesetManager{
                tile_sets: tile_sets,
            },
        };

        return Ok(map);
    }
}

impl Drawable for Map {
    fn draw(&mut self, event: &Event, mut window: &mut PistonWindow) {
        window.draw_2d(event, |c, g| {
            let mut tf = c.transform
                .trans(0.2, 0.2)
                .scale( 0.2, 0.2);

            let (w,h) = self.texture.get_size();
            let rec = [0.0, 0.0, w as f64, h as f64] ;
            self.image.src_rect(rec).draw(
                &self.texture,
                &DrawState::default(),
                tf,
                g
            );

            for layer in &self.map.layers {
                for x in 0..layer.tiles.len() {
                    for y in 0..layer.tiles[x].len() {
                        let tile = layer.tiles[x][y];
                        self.manager.draw(tile as usize, x, y, c, g);
                    }
                }
            }
        });
    }
}