use sdl2::image::{INIT_PNG, INIT_JPG};
use sdl2::Sdl;
use sdl2;

pub fn init() -> Sdl {
    let sdl_context = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

    sdl_context
}