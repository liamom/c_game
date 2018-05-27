
use engine;
use engine::*;
use engine::map::map::Map;
use engine::draw::Drawable;
use engine::Canvas;
use engine::error::GameError;
use engine::sdl2;
use engine::sdl2::*;
use sdl2::render::RenderTarget;
use sdl2::render::WindowCanvas;
use engine::sdl2::event::Event;
use engine::sdl2::event::EventType;

struct Bar{

}

impl Drawable for Bar {
    fn draw<T: RenderTarget>(&mut self, tc: &mut Canvas<T>) {
        unimplemented!()
    }
//    fn draw(&mut self, tc: &mut Canvas<T>) {
//        use graphics::*;
//
//        let args = window.draw_size();
//        let (h, w) = (args.height as f64, args.width as f64);
//
//        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
//
//        let (x0, y0) = (0.0, h - 50.0);
//        let (x1, y1) = (w  , h);
//        let square = rectangle::rectangle_by_corners(x0, y0, x1, y1);
//
//        window.draw_2d(event, |c, gl| {
//            let bar = Rectangle::new(RED).draw(square, &c.draw_state, c.transform, gl);
//        });

//        let transform = c.transform;
//
//        // Draw a box rotating around the middle of the screen.
//        rectangle(RED, square, transform, gl);

//    }
}

struct Objects {
    bar: Bar,
    map: Map,
}

pub struct MyApp {
    sdl_context: sdl2::Sdl,
    video_subsystem: VideoSubsystem,
//    window : sdl2::video::Window,

    canvas: sdl2::render::WindowCanvas,
//    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,

    rotation: f64,  // Rotation for the square.
    objects: Objects,
}



impl MyApp {
    pub fn new() -> Result<Self, GameError> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window  = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas : sdl2::render::WindowCanvas = window.into_canvas().build().unwrap();
        let mut texture_creator = canvas.texture_creator();
//        texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,

        Ok(MyApp {
            sdl_context,
            video_subsystem,
            canvas: canvas,
//            texture_creator,
            rotation: 0.0,
            objects: Objects{
                bar: Bar{},
                map: Map::new(&mut texture_creator)?,
            },
        })
    }
}

impl AppHandler for MyApp {
    fn update(&mut self) -> bool {
        for event in self.sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => {return false;},
                _ => {}
            }
        }

        return true;
    }

    fn render(&mut self) {
        self.canvas.clear();
        self.objects.map.draw(&mut self.canvas);
        self.canvas.present();
    }

    fn event(&mut self) {
        unimplemented!()
    }
}
