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
use std::rc::Rc;

struct Tile {
    id: u32,
    texture: Rc<Texture>,
    source_rect: Rect,
}

impl Tile {
    fn render<T: RenderTarget>(&self, c: &mut Canvas<T>, x: usize, y: usize) {
//        Rect::new(x as i32, y as i32, self.source_rect[2] as u32, self.source_rect[3] as u32);
        let mut dest_rect = self.source_rect.clone();
        dest_rect.set_x(x as i32);
        dest_rect.set_y(y as i32);

        c.copy(&self.texture, self.source_rect, dest_rect).expect("Render failed");
    }

    fn parse<W>(tc: &TextureCreator<W>, ts: &tiled::Tileset) -> Vec<Self> {
        let ref image = ts.images[0];
        let full_path: PathBuf =
            [&get_assets_path(), "assets", "tmx", &image.source].iter().collect();

        debug!("creating with tileset {:?}", full_path);

        let tex = Rc::new(tc.load_texture(full_path).unwrap());

        let mut tiles: Vec<Tile> = Vec::new();

        let mut id = ts.first_gid as i32;
        let mut i = 0;
        let mut j = 0;
        let image = &ts.images[0];
        let i_w = image.width;
        let i_h = image.height;
        let mut x = ts.margin as i32;
        let mut y = ts.margin as i32;
        let t_w = ts.tile_width as i32;
        let t_h = ts.tile_height as i32;
        let spacing = ts.spacing as i32;
        loop {
            info!("id:{} i:{} j:{} x:{} y:{}", id, i, j, x, y);

            tiles.push(Tile{
                id: id as u32,
                texture: tex.clone(),
                source_rect: Rect::new(x, y, ts.tile_width, ts.tile_height),
            });

            //width
            x += t_w + spacing;
            if (x + t_w) > i_w {
                //row ended
                x = ts.margin as i32;
                i = 0;

                //adjust height
                j += 1;
                y += t_h + spacing;
                //height
                if (y + t_h) > i_h {
                    break;
                }
            } else {
                i += 1;
            }

            id = id + 1;
        }

        //i don't think there is ever anything in here
        for tile in &ts.tiles {
            info!("tile: {:?}", tile);
        }

        return tiles;
    }

    fn get_source_rect(local_id: u32, ts: &tiled::Tileset) -> [f64; 4] {
        let (w,h) = (ts.tile_width, ts.tile_height);

        let image = &ts.images[0];
        let col_num = (image.width as u32) / w - 1;
        let col = local_id % col_num;
        let row = local_id / col_num;

        let margin = ts.margin;
        let w_spacing = ts.spacing * col + margin;
        let h_spacing = ts.spacing * row + margin;

        let (x,y) = (col * w + w_spacing, row * h + h_spacing);
        return [ x as f64, y as f64, (x + w) as f64, (y + h) as f64];
    }
}

pub struct Map {
    texture: Texture,
    map: tiled::Map,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new<T>(mut tc: &mut TextureCreator<T>) -> Result<Self, GameError> {
        let texture = tc.load_texture(get_asset("smiley.png")).unwrap();

        let path_str = get_asset("tmx/untitled.tmx");
        let path = Path::new(&path_str);
        let map = tiled::parse_file(path)?;

        //tile sets
        let mut tiles: Vec<Tile> = Vec::new();
        for tile_set in &map.tilesets {
            let temp = Tile::parse(&mut tc, &tile_set);
            tiles.extend(temp);
        }

        //verify the tile ids
        for i in 0..tiles.len() {
            if i+1 != tiles[i].id as usize {
                return Err(GameError::new("Error id conflict"));
            }
        }

        let map = Map {
            texture,
            map,
            tiles,
        };

        return Ok(map);
    }

    fn draw_tile<T: RenderTarget>(&self, id: usize, x: usize, y: usize, mut c: &mut Canvas<T>) {
        if id == 0 {
            return;
        }

//        let tf = c.transform.trans(
//            ((x as i32 - y as i32) * tile_set.tile_set.tile_width  as i32) as f64 / 2.0,
//            ((x as i32 + y as i32) * tile_set.tile_set.tile_height as i32) as f64 / 2.0,
//        );

        let tile = &self.tiles[id - 1];
        assert_eq!(id, tile.id as usize);

        let (x,y) = (((x as i32 - y as i32) * tile.source_rect.width() as i32) as f64 / 2.0,
                     ((x as i32 + y as i32) * tile.source_rect.height() as i32) as f64 / 2.0);


        tile.render(&mut c, x as usize, y as usize);
    }

}

impl Drawable for Map {
    fn draw<T: RenderTarget>(&mut self, c: &mut Canvas<T>) {
        c.copy(&mut self.texture, None, None);

        for layer in &self.map.layers {
            for x in 0..layer.tiles.len() {
                for y in 0..layer.tiles[x].len() {
                    let tile = layer.tiles[y][x];
                    self.draw_tile(tile as usize, x, y, c);
                }
            }
        }
    }
}