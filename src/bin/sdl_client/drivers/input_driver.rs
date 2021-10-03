use sdl2;

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
}
