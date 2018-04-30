extern crate rand;
extern crate piston_window;
extern crate tiled;
#[macro_use] extern crate log;
extern crate env_logger;

use piston_window::*;
use std::result::Result;

mod map;
mod name_generator;
use name_generator::*;
mod error;
mod gfx;
mod util;

use std::env;
use error::GameError;

mod app;

fn rec(i: i32)->i32 {
    match i {
        0 => {1}
        _ => {i+rec(i-1)}
    }
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "game");
    env_logger::init();
    debug!("Logger initialized");

    let mut rng = rand::ChaChaRng::new_unseeded();

    println!("male name={:?}, female name={:?}", Name::male(&mut rng), Name::female(&mut rng));


    println!("rec={}", rec(5));
    println!("Hello, world!");

    if let Err(e) = exec() {
        println!("Exception thrown = {:?}", e);
    }
}

// Change this to OpenGL::V2_1 if not working.
const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

fn exec() -> Result<(), GameError> {
    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new(
        "spinning-square",
        [1200, 1200]
    )
        .opengl(OPENGL_VERSION)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = app::App::new(&mut window)?;

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            app.render(&e, &mut window);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }

    Ok(())
}