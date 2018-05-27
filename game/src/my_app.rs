use engine::*;
use engine::init::init;
use engine::map::map::Map;
use engine::draw::Drawable;
use engine::error::GameError;
use engine::sdl2;
use engine::sdl2::event::Event;
use engine::util::math::Trans;
use engine::sdl2::keyboard::Keycode;

struct Objects {
    map: Map,
}

pub struct MyApp {
    sdl_context: sdl2::Sdl,
    canvas: sdl2::render::WindowCanvas,

    objects: Objects,
    camera: Trans,
}



impl MyApp {
    pub fn new() -> Result<Self, GameError> {
//        let sdl_context = sdl2::init().unwrap();
        let sdl_context = init();
        let video_subsystem = sdl_context.video().unwrap();

        let window  = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .resizable()
            .opengl()
            .build()
            .unwrap();

        let canvas : sdl2::render::WindowCanvas = window.into_canvas().build().unwrap();
        let mut texture_creator = canvas.texture_creator();

        Ok(MyApp {
            sdl_context,
            canvas,
            objects: Objects{
                map: Map::new(&mut texture_creator)?,
            },
            camera: Trans::new().scale(-0.5).build(),
        })
    }
}

const SPEED: f64 = 10.0;
const SCALE_SPEED: f64 = 0.1;

impl AppHandler for MyApp {
    fn update(&mut self) -> bool {
        for event in self.sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => {return false;},
                Event::KeyDown{ keycode: Some(Keycode::Up), .. } => {self.camera.trans_y(SPEED);}
                Event::KeyDown{ keycode: Some(Keycode::Down), .. } => {self.camera.trans_y(-SPEED);}
                Event::KeyDown{ keycode: Some(Keycode::Left), .. } => {self.camera.trans_x(SPEED);}
                Event::KeyDown{ keycode: Some(Keycode::Right), .. } => {self.camera.trans_x(-SPEED);}
                Event::KeyDown{ keycode: Some(Keycode::Z), .. } => {self.camera.scale(-SCALE_SPEED);}
                Event::KeyDown{ keycode: Some(Keycode::X), .. } => {self.camera.scale(SCALE_SPEED);}
                Event::MouseMotion {xrel, yrel, mousestate, ..} => {
                    if mousestate.left() {
                        self.camera.trans_x(xrel as f64);
                        self.camera.trans_y(yrel as f64);
                    }
                }
                Event::MouseWheel {y, ..} => {
                    self.camera.scale(y as f64);
                }
                _ => {}
            }
        }

        return true;
    }

    fn render(&mut self) {
        self.canvas.clear();
        self.objects.map.draw(&mut self.canvas, &self.camera);
        self.canvas.present();
    }

    fn event(&mut self) {
        unimplemented!()
    }
}
