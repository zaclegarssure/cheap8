use sdl2;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use cheap8::{WIDTH,HEIGHT};
pub const SCALE_FACTOR: u32 = 16;

pub struct DisplayDriver {
    canvas: Canvas<Window>,
}

impl DisplayDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Cheap8", (WIDTH as u32)*SCALE_FACTOR, (HEIGHT as u32)*SCALE_FACTOR)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.present();

        DisplayDriver {
            canvas,
        }
    }

    pub fn draw(&mut self,image: &[bool;WIDTH*HEIGHT]) -> () {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if image[x + y*WIDTH] {
                    self.canvas.fill_rect(Rect::new(x as i32*SCALE_FACTOR as i32,y as i32*SCALE_FACTOR as i32,SCALE_FACTOR,SCALE_FACTOR)).unwrap();
                }
            }
        }

        self.canvas.present();
    }
}
