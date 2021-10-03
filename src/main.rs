use cheap8::*;

fn main() {
    let mut cpu = Cpu::new();
    cpu.reset();
    cpu.load("IBMLogo.ch8");
}
