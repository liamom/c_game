use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Orientation {
    Orthogonal,
    Isometric,
    Staggered,
    Hexagonal
}

impl FromStr for Orientation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "Orthogonal" => Ok(Orientation::Orthogonal),
            "isometric" => Ok(Orientation::Isometric),
            "staggered" => Ok(Orientation::Staggered),
            "hexagonal" => Ok(Orientation::Hexagonal),
            _ => {Err(())},
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RenderOrder {
    RightDown,
    RightUp,
    LeftDown,
    LeftUp
}

impl FromStr for RenderOrder {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "right-down" => {Ok(RenderOrder::RightDown)}
            "right-up" => {Ok(RenderOrder::RightUp)}
            "left-down" => {Ok(RenderOrder::LeftDown)}
            "left-up" => {Ok(RenderOrder::LeftUp)}
            _=>{Err(())}
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum StaggerAxis{
    X,
    Y,
}

impl FromStr for StaggerAxis {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "x" => {Ok(StaggerAxis::X)}
            "y" => {Ok(StaggerAxis::Y)}
            _ => {Err(())}
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum StaggerIndex {
    Odd,
    Even
}

impl FromStr for StaggerIndex {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "even" => Ok(StaggerIndex::Even),
            "odd" => Ok(StaggerIndex::Odd),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color{ r, b, g, a,}
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {r: 0, g: 0, b: 0, a: 0, }
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut color_string= s.to_string();
        //removes preceding #
        let result = color_string.rfind('#');
        if let Some(r) = result {
            let (_,cs) = s.split_at(r + 1);
            color_string = cs.to_string();
        }

        match color_string.len() {
            6 | 8 => {
                let value:i64 = i64::from_str_radix(color_string.as_ref(), 16).unwrap();

                let color = Color {
                    r: ((value >> 16) & 0xff) as u8,
                    g: ((value >> 8) & 0xff) as u8,
                    b: (value & 0xff) as u8,
                    a: match color_string.len() {
                        8 => ((value >> 24) & 0xff) as u8,
                        _ =>255
                    },
                };



                return Ok(color);
            }
            _=> {panic!("invalid color");}
        };
    }
}

pub enum Property {
    Boolean(String, bool),
    Float(String, f32),
    Int(String, i32),
    String(String, String),
    Colour(String, Color),
    //    File(String, File),
    Undef
}


#[derive(Default, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

pub type Vector2u = Vector2<u32>;
pub type Vector2i = Vector2<i32>;
pub type Vector2f = Vector2<f32>;

impl<T> Vector2<T> {

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn color() {
        let color = Color::from_str("#4286f4").unwrap();
        assert_eq!(color, Color{
            r: 66,
            g: 134,
            b: 244,
            a: 255,
        });
    }

    #[test]
    fn color_alpha() {
        let color = Color::from_str("#424286f4").unwrap();
        assert_eq!(color, Color{
            r: 66,
            g: 134,
            b: 244,
            a: 66,
        });
    }
}