use sdl2::rect::Rect;


#[derive(Clone, Debug)]
pub struct TRect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl TRect {
    pub fn new() -> Self {
        TRect{
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
        }
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
    }
}

/// transform
#[derive(Clone, Debug)]
pub struct Trans {
    x: f64,
    y: f64,
    scale: f64,
}

impl Trans {
    pub fn new() -> Self {
        Trans{
            x: 0.0,
            y: 0.0,
            scale: 1.0,
        }
    }

    pub fn set_offset(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn set_offset_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_offset_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn trans_x(&mut self, x: f64) -> &mut Self {
        self.x += x;
        return self;
    }

    pub fn trans_y(&mut self, y: f64) -> &mut Self {
        self.y += y;
        return self;
    }

    pub fn scale(&mut self, scale: f64) -> &mut Self {
        self.scale += scale;
        return self;
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }

    pub fn build(&mut self) -> Self {
        return self.clone();
    }

    pub fn apply(&self, rect: &TRect) -> TRect {
        let x = (rect.x + self.x) as f64;
        let y = (rect.y + self.y) as f64;
        let w = rect.w as f64;
        let h = rect.h as f64;
        TRect {
            x: x * self.scale,
            y: y * self.scale,
            w: w * self.scale,
            h: h * self.scale,
        }
    }
}