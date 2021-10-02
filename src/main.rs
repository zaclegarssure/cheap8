use cheap8::*;

fn main() {
    let mut cpu = cpu::Cpu::new();
    cpu.reset();
    cpu.load("IBMLogo.ch8");
    let mut counter: u64 = 0;
    loop {
        cpu.cycle();
        cpu.display.debug_draw();
        if counter == 0 {
            cpu.display.debug_draw();
        }
        counter = (counter + 1) % u64::MAX;
    }
}
