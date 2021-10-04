pub struct Timer {
    pub timer: u8,
}

impl Timer {
    pub fn new() -> Self {
        Timer { timer: 0 }
    }

    pub fn reset(&mut self) -> () {
        self.timer = 0;
    }

    pub fn decrement(&mut self) -> bool {
        if self.timer == 0 {
            return false;
        }

        self.timer -= 1;
        true
    }
}
