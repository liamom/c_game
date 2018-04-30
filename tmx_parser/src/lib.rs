pub mod map;
pub use self::map::*;
pub mod types;
pub use self::types::*;
pub mod tile_set;
pub mod tile_layer;
pub use self::tile_layer::*;
pub mod tile_error;

extern crate xml;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

