use sdl2::audio::AudioCallback;
use sdl2::audio::AudioDevice;
use sdl2::audio::AudioSpecDesired;

/// Struct that drives audio, taken
/// from sdl2 examples.
pub struct AudioDriver {
    device: AudioDevice<SquareWave>,
}

impl AudioDriver {
    /// Create new driver from [`sdl2::Sdl`].
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let audio_substystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1), // mono
            samples: None,     // default sample size
        };

        AudioDriver {
            device: audio_substystem
                .open_playback(None, &desired_spec, |spec| SquareWave {
                    phase_inc: 440.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25,
                })
                .unwrap(),
        }
    }

    pub fn play(&self) {
        self.device.resume();
    }

    pub fn stop(&self) {
        self.device.pause();
    }
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
