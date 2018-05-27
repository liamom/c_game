extern crate rand;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate engine;

mod name_generator;
mod gfx;
mod my_app;

use std::result::Result;
use std::env;
use name_generator::*;
use engine::error::GameError;

//extern crate sdl2;

use engine::*;

fn rec(i: i32)->i32 {
    match i {
        0 => {1}
        _ => {i+rec(i-1)}
    }
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "game,engine");
    env_logger::init();
    debug!("Logger initialized");

    let mut rng = rand::ChaChaRng::new_unseeded();

    println!("male name={:?}, female name={:?}", Name::male(&mut rng), Name::female(&mut rng));


    println!("rec={}", rec(5));
    println!("Hello, world!");

    if let Err(e) = exec() {
        println!("Exception thrown = {:?}", e);
    }

    engine::do_test();
}

fn exec() -> Result<(), GameError> {
    let mut handler = my_app::MyApp::new()?;
    let mut app = engine::App::new();

    app.exec(&mut handler);

    Ok(())
}