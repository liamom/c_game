use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::time::Duration;
use std::time::*;
use std::ops::AddAssign;
use sdl2::render::RenderTarget;
use sdl2::render::Canvas;
use std::thread::sleep;
use std::time;
use std::ops::Fn;

pub trait AppHandler {
    fn update(&mut self) -> bool;
    fn render(&mut self);
    fn event(&mut self);
}

fn now() -> u64 {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let sec = (time.as_secs() * 1000) + (time.subsec_nanos() as u64 / 1000_000u64);
    return sec;
}

pub struct App {
}

impl App {
    pub fn new() -> Self {
        App{}
    }

    pub fn exec<T: AppHandler>(&mut self, app: &mut T) {
        const TICKS_PER_SECOND: u64 = 25;
        //    const SKIP_TICKS: i32 = 1000 / TICKS_PER_SECOND;
        const SKIP_TICKS: u64 = 1000u64 / TICKS_PER_SECOND;
        const MAX_FRAMESKIP: u64 = 5;

        let mut next_game_tick = now();

        let mut loops;
        let mut interpolation;

        'outer_loop: loop {
            loops = 0;
            while (now() > next_game_tick && loops < MAX_FRAMESKIP) {
                if (!app.update()) {
                    break 'outer_loop;
                }

                next_game_tick += SKIP_TICKS;
                loops = loops + 1;
            }

            interpolation = (now() + SKIP_TICKS - next_game_tick) / SKIP_TICKS;
            app.render();
        }
    }
}