use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct InputDriver {
    event_pump: sdl2::EventPump,
}

impl InputDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let event_pump = sdl_context.event_pump().unwrap();
        InputDriver {
            event_pump,
        }
    }

    pub fn poll(&mut self) -> Option<[bool;16]> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => return None,
                _ => ()
            }
        }

        let mut key_pressed = [false;16];

        self.event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .filter_map(Self::key_code_to_hex)
            .for_each(|hex_key| key_pressed[hex_key] = true);


        Some(key_pressed)
    }

    fn key_code_to_hex(keycode: Keycode) -> Option<usize> {
        match keycode {
            Keycode::Num1 => Some(0x1),
            Keycode::Num2 => Some(0x2),
            Keycode::Num3 => Some(0x3),
            Keycode::Num4 => Some(0xc),
            Keycode::Q => Some(0x4),
            Keycode::W => Some(0x5),
            Keycode::E => Some(0x6),
            Keycode::R => Some(0xd),
            Keycode::A => Some(0x7),
            Keycode::S => Some(0x8),
            Keycode::D => Some(0x9),
            Keycode::F => Some(0xe),
            Keycode::Z => Some(0xa),
            Keycode::X => Some(0x0),
            Keycode::C => Some(0xb),
            Keycode::V => Some(0xf),
            _ => None,
        }
    }
}
