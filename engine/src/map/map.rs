use error::*;
use util::*;
use tiled;
use std::path::Path;
use std::path::PathBuf;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::image::*;
use sdl2::render::Canvas;
use draw::Drawable;
use sdl2::render::RenderTarget;
use std::rc::Rc;
use util::math::TRect;

struct Tile {
    id: u32,
    texture: Rc<Texture>,
    source_rect: TRect,
}

impl Tile {
    fn render<T: RenderTarget>(&self, c: &mut Canvas<T>, dest_rect: TRect) {
        let dr2 = dest_rect.to_rect();
        c.copy(&self.texture, self.source_rect.to_rect(), dr2).expect("Render failed");
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
                source_rect: TRect{x: x as f64, y: y as f64, w: t_w as f64, h: t_h as f64},
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

    fn draw_tile<T: RenderTarget>(&self, id: usize, x: usize, y: usize, mut c: &mut Canvas<T>, trans: &math::Trans) {
        if id == 0 {
            return;
        }

        let tile = &self.tiles[id - 1];
        assert_eq!(id, tile.id as usize);

        let mut dest_rect = tile.source_rect.clone();

        let xf= ((x as i32 - y as i32) * dest_rect.w as i32) as f64 / 2.0;
        let yf= ((x as i32 + y as i32) * dest_rect.h as i32) as f64 / 2.0;

        dest_rect.x = xf;
        dest_rect.y = yf;

        let t_rect = trans.apply(&dest_rect);

        tile.render(&mut c, t_rect);
    }

}

impl Drawable for Map {
    fn draw<T: RenderTarget>(&mut self, c: &mut Canvas<T>, trans: &math::Trans) {
        c.copy(&mut self.texture, None, None).unwrap();

        for layer in &self.map.layers {
            for x in 0..layer.tiles.len() {
                for y in 0..layer.tiles[x].len() {
                    let tile = layer.tiles[y][x];
                    self.draw_tile(tile as usize, x, y, c, trans);
                }
            }
        }
    }
}