use std::num::ParseIntError;
use structopt::StructOpt;

fn parse_color(src: &str) -> Result<u32, ParseIntError> {
    if src.starts_with("0x") {
        return u32::from_str_radix(&src[2..], 16);
    } else {
        return u32::from_str_radix(src, 10);
    }
}

#[derive(StructOpt)]
pub struct Cli {
    /// Path to the rom file
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,

    /// Pixels color, in RGB format
    #[structopt(short, long, default_value = "0x00F", parse(try_from_str = parse_color))]
    pub pixel_color: u32,
    /// Background color, in RGB format
    #[structopt(short, long, default_value = "0x000", parse(try_from_str = parse_color))]
    pub bg_color: u32,

    #[structopt(short, long, default_value = "14")]
    pub scale_factor: u32,
}

impl Cli {
    pub fn rgb_color(color: u32) -> (u8, u8, u8) {
        let r = ((color & 0xFF0000) >> 16) as u8;
        let g = ((color & 0x00FF00) >> 8) as u8;
        let b = (color & 0x0000FF) as u8;

        (r, g, b)
    }
}
