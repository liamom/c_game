pub extern crate sdl2;
extern crate tiled;
#[macro_use] extern crate log;

pub mod surface;
pub use surface::*;

pub mod map;
pub use map::*;

pub mod util;
pub use util::*;

pub mod error;
pub mod draw;

pub mod init;
pub use init::*;

pub mod app;
pub use app::*;

pub use sdl2::render::Canvas;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
