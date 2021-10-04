use sdl2;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use cheap8::{WIDTH,HEIGHT};

use crate::Cli;

pub struct DisplayDriver {
    scale_factor: u32,
    pixel_color: Color,
    bg_color: Color,
    canvas: Canvas<Window>,
}

impl DisplayDriver {
    pub fn new(sdl_context: &sdl2::Sdl, args: &Cli) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Cheap8", (WIDTH as u32)*args.scale_factor, (HEIGHT as u32)*args.scale_factor)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let (pr, pg, pb) = Cli::rgb_color(args.pixel_color);
        let (br, bg, bb) = Cli::rgb_color(args.bg_color);

        DisplayDriver {
            scale_factor: args.scale_factor,
            pixel_color: Color::RGB(pr, pg, pb),
            bg_color: Color::RGB(br, bg, bb),
            canvas,
        }
    }

    pub fn draw(&mut self,image: &[bool;WIDTH*HEIGHT]) -> () {
        self.canvas.set_draw_color(self.bg_color);
        self.canvas.clear();
        self.canvas.set_draw_color(self.pixel_color);
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if image[x + y*WIDTH] {
                    self.canvas.fill_rect(Rect::new(x as i32*self.scale_factor as i32,y as i32*self.scale_factor as i32,self.scale_factor,self.scale_factor)).unwrap();
                }
            }
        }

        self.canvas.present();
    }
}
