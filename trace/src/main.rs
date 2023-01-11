use rust_nes_emulator::bus::Bus;
use rust_nes_emulator::cpu::cpu::CPU;
use rust_nes_emulator::cpu::trace;
use rust_nes_emulator::rom::Rom;

fn main() {
    //load the game
    let bytes: Vec<u8> = std::fs::read("nestest.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();

    let bus = Bus::new(rom);
    let mut cpu = CPU::new(bus);
    cpu.reset();

    cpu.program_counter = 0xc000;

    cpu.run_with_callback(move |cpu| {
        println!("{}", trace(cpu));
    });
}
