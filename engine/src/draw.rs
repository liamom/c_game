use sdl2::render::Canvas;
use sdl2::render::RenderTarget;

pub trait Drawable {
    fn draw<T: RenderTarget>(&mut self, tc: &mut Canvas<T>);
}

pub const X0 :u8 = 0;
pub const Y0 :u8 = 1;
pub const X1 :u8 = 2;
pub const Y1 :u8 = 3;


//impl Drawable for Map {
//    fn draw(&mut self, args: &RenderArgs, c: &Context, gl: &mut GlGraphics) {
//        const COLOR: Color = [1.0, 1.0, 1.0, 1.0];
//        const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
//        const BLACK: Color = [0.0; 4];
//        Rectangle::new(COLOR)
//            .border(Border{ color: WHITE, radius: 2.0})
//            .draw(rectangle::rectangle_by_corners(0.0, 0.0, 50.0, 50.0),&c.draw_state, c.transform, gl);
//    }
//}