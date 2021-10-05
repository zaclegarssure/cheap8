use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use cheap8::{HEIGHT, WIDTH};

use crate::parse_args::Cli;

/// Struct that can display chip8 screen to SDL window
pub struct DisplayDriver {
    scale_factor: u32,
    pixel_color: Color,
    bg_color: Color,
    canvas: Canvas<Window>,
}

impl DisplayDriver {
    /// Create new driver from [`sdl2::Sdl`]. `args` are used to
    /// know the `scale_factor` of the window and pixels colors.
    pub fn new(sdl_context: &sdl2::Sdl, args: &Cli) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(
                "Cheap8",
                (WIDTH as u32) * args.scale_factor,
                (HEIGHT as u32) * args.scale_factor,
            )
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

    /// Draw `image` to the screen.
    pub fn draw(&mut self, image: &[bool; WIDTH * HEIGHT]) -> () {
        self.canvas.set_draw_color(self.bg_color);
        self.canvas.clear();
        self.canvas.set_draw_color(self.pixel_color);
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if image[x + y * WIDTH] {
                    self.canvas
                        .fill_rect(Rect::new(
                            x as i32 * self.scale_factor as i32,
                            y as i32 * self.scale_factor as i32,
                            self.scale_factor,
                            self.scale_factor,
                        ))
                        .unwrap();
                }
            }
        }

        self.canvas.present();
    }
}
