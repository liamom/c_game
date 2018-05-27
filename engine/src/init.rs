use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use sdl2::Sdl;
use sdl2;

fn init() -> Sdl {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

    sdl_context
}