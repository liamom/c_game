use std::io;
use std::error::Error;
use std::any::TypeId;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use tiled;

#[derive(Debug)]
pub struct GameError{
    what: String,
}

impl GameError {
    pub fn new(str: &str) -> Self {
        GameError{
            what: str.to_string(),
        }
    }
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        write!(f, "{}", self.what);
        Ok(())
    }
}

impl Error for GameError {
    fn description(&self) -> &str {
//        self.what.as_ref()
        "err"
    }

    fn cause(&self) -> Option<&Error> {
        return None;
    }
}

impl From<io::Error> for GameError {
    fn from(ref e: io::Error) -> Self {
        GameError {
            what: e.to_string(),
        }
    }
}

impl From<tiled::TiledError> for GameError {
    fn from(t: tiled::TiledError) -> Self {
        GameError{
            what: t.to_string(),
        }
    }
}
