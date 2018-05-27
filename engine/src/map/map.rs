use std::env;
use std::io;
use error::*;
use util::*;
use tiled;
use std::fs::File;
use std::path::Path;
use log::*;
use std::path::PathBuf;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::image::*;
use sdl2::render::Canvas;
use draw::Drawable;
use sdl2::render::RenderTarget;
use sdl2::rect::Rect;

struct TileSet {
    texture: Texture,
    tile_set: tiled::Tileset,
}

impl TileSet {
    fn new<W>(tc: &TextureCreator<W>, ts: &tiled::Tileset) -> Self {
        let ref image = ts.images[0];
        let full_path: PathBuf =
            [&get_assets_path(), "assets", "tmx", &image.source].iter().collect();

        debug!("creating with tileset {:?}", full_path);

        let tex = tc.load_texture(full_path).unwrap();

//        let tex = Texture::from_path(
//            &mut window.factory,
//            full_path,
//            Flip::None,
//            &TextureSettings::new()).unwrap();

        for tile in &ts.tiles {
            info!("tile: {:?}", tile);
        }

        TileSet {
            texture: tex,
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

struct TilesetManager {
    tile_sets: Vec<TileSet>,

}

impl TilesetManager {
    fn get_by_id<'b>(&'b self, id: usize) -> &'b TileSet {
        let set = self.tile_sets
            .iter().rev().find(|t| id as u32 >= t.tile_set.first_gid);

        if let Some(t) = set {
            return t;
        } else {
            panic!("value {} not found", id)
        }
    }

    fn draw<T: RenderTarget>(&self, id: usize, x: usize, y: usize, c: &mut Canvas<T>) {
        if id == 0 {
            return;
        }

        let tile_set = self.get_by_id(id);

//        let tf = c.transform.trans(
//            ((x as i32 - y as i32) * tile_set.tile_set.tile_width  as i32) as f64 / 2.0,
//            ((x as i32 + y as i32) * tile_set.tile_set.tile_height as i32) as f64 / 2.0,
//        );

        let (x,y) = (((x as i32 - y as i32) * tile_set.tile_set.tile_width  as i32) as f64 / 2.0,
                              ((x as i32 + y as i32) * tile_set.tile_set.tile_height as i32) as f64 / 2.0);

        let source_rect1 = tile_set.get_source_rect(id as u32);
        let source_rect = Rect::new(source_rect1[0] as i32, source_rect1[1] as i32,
                                    source_rect1[2] as u32, source_rect1[3] as u32);
        let dest_rect = Rect::new(x as i32, y as i32, tile_set.tile_set.tile_width, tile_set.tile_set.tile_height);

        c.copy(&tile_set.texture, source_rect, dest_rect).expect("Render failed");
//        c.copy(&tile_set.texture, None, None).expect("Render failed");
    }
}

pub struct Map {
//    map: TMXMap,
//    image: Image,
    texture: Texture,
//    texture_creator: TextureCreator<T>,
    map: tiled::Map,
    manager: TilesetManager,
}

impl Map {
    pub fn new<T>(mut tc: &mut TextureCreator<T>) -> Result<Self, GameError> {
        let texture = tc.load_texture(get_asset("smiley.png")).unwrap();


//        let path = get_asset("tmx/untitled.tmx");
//        let img = get_asset("smiley.png");
//
//        let texture = Texture::from_path(
//            &mut window.factory,
//            img,
//            Flip::None,
//            &TextureSettings::new()).unwrap();

//        let map_file = File::open(get_asset("tmx/untitled.tmx"))?;
        let path_str = get_asset("tmx/untitled.tmx");
        let path = Path::new(&path_str);
        let map = tiled::parse_file(path)?;

//        let reef = tc;

        //tile sets
        let mut tile_sets: Vec<TileSet> = Vec::new();
        for tile_set in &map.tilesets {
            let temp: TileSet = TileSet::new(&mut tc, &tile_set);
            tile_sets.push(temp);
        }

        let map = Map {
//            image: Image::new(),
            texture,
//            texture_creator: tc,
            map,
            manager: TilesetManager {
                tile_sets: tile_sets,
            },
        };

        return Ok(map);
    }
}

impl Drawable for Map {
    fn draw<T: RenderTarget>(&mut self, c: &mut Canvas<T>) {
        c.copy(&mut self.texture, None, None);

        for layer in &self.map.layers {
            for x in 0..layer.tiles.len() {
                for y in 0..layer.tiles[x].len() {
                    let tile = layer.tiles[y][x];
                    self.manager.draw(tile as usize, x, y, c);
                }
            }
        }
    }
}