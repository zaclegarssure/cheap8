pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Display {
    pub display: [bool;WIDTH*HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Display {
            display: [false;WIDTH*HEIGHT],
        }
    }

    pub fn clear(&mut self) -> () {
        self.display = [false;WIDTH*HEIGHT];
    }

    fn at(&self, x: usize, y: usize) -> bool {
        let x = x % WIDTH;
        let y = y % HEIGHT;
        self.display[y*WIDTH + x]
    }

    fn set(&mut self, x: usize, y: usize, val: bool) -> () {
        let x = x % WIDTH;
        let y = y % HEIGHT;
        self.display[y*WIDTH + x] = val;
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut vf = false;
        let rows = sprite.len();

        for i in 0..rows {
            let row = sprite[i];
            let yi = y + i;
            for col in 0..8 {
                let pixel = (row & (0b10000000 >> col)) != 0;
                let xi = x + col;
                if pixel {
                    let old_val = self.at(xi,yi);
                    self.set(xi,yi,pixel ^ old_val);
                    if old_val {
                        vf = true;
                    }
                }
                if xi == WIDTH -1 {
                    break;
                }
            }

            if yi == HEIGHT - 1 {
                return vf;
            }

        }


        vf
    }

    pub fn get(&self) -> &[bool;WIDTH*HEIGHT] {
        &self.display
    }

    pub fn debug_draw(&self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if self.at(x,y) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}
