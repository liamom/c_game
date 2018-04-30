
use piston_window::*;
use map::map::*;
use gfx::draw::Drawable;
use error::GameError;

struct Bar{

}



impl Drawable for Bar {
    fn draw(&mut self, event: &Event, window: &mut PistonWindow) {
//        use graphics::*;

        let args = window.draw_size();
        let (h, w) = (args.height as f64, args.width as f64);

        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let (x0, y0) = (0.0, h - 50.0);
        let (x1, y1) = (w  , h);
        let square = rectangle::rectangle_by_corners(x0, y0, x1, y1);

        window.draw_2d(event, |c, gl| {
            let bar = Rectangle::new(RED).draw(square, &c.draw_state, c.transform, gl);
        });

//        let transform = c.transform;
//
//        // Draw a box rotating around the middle of the screen.
//        rectangle(RED, square, transform, gl);

    }
}

struct Objects {
    bar: Bar,
    map: Map,
}

pub struct App {
    rotation: f64,  // Rotation for the square.
    objects: Objects,
}



impl App {
    pub fn new(window: &mut PistonWindow) -> Result<Self, GameError> {
        Ok(App {
            rotation: 0.0,
            objects: Objects{
                bar: Bar{},
                map: Map:: new(window)?,
            }
        })
    }

    pub fn render(&mut self, event: &Event, window: &mut PistonWindow) {

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let size = window.draw_size();
        let (x, y) = ((size.width / 2) as f64,
                      (size.height / 2) as f64);

//        let ref mut bar = self.bar;
        let ref mut objects = self.objects;

        window.draw_2d(event, |c, gl|{

            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);

        });

        objects.bar.draw(event, window);
        objects.map.draw(event, window);
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}
