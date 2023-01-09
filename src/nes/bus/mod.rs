use super::{ram::Ram, rom::Rom};

pub struct Bus<'a> {
    program_rom: &'a Rom,
    work_ram: &'a mut Ram,
}

pub trait CpuBus {
    fn read_word(&mut self, addr: u16) -> u16;

    fn read(&mut self, addr: u16) -> u8;

    fn write(&mut self, addr: u16, data: u8);
}

impl<'a> Bus<'a> {
    pub fn new(program_rom: &'a Rom, work_ram: &'a mut Ram) -> Bus<'a> {
        Bus {
            program_rom,
            work_ram,
        }
    }
}

impl<'a> CpuBus for Bus<'a> {
    fn read_word(&mut self, addr: u16) -> u16 {
        let lower = self.read(addr) as u16;
        let upper = self.read(addr + 1) as u16;
        (upper << 8 | lower) as u16
    }

    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.work_ram.read(addr & 0x07FF),
            0x6000..=0x7FFF => {
                println!(
                    "Not implemented. This area is battery backup ram area 0x{:x}",
                    addr
                );
                0
            }
            0x8000..=0xBFFF => self.program_rom.read(addr - 0x8000),
            0xC000..=0xFFFF if self.program_rom.size() <= 0x4000 => {
                self.program_rom.read(addr - 0xC000)
            }
            0xC000..=0xFFFF => self.program_rom.read(addr - 0x8000),
            _ => panic!("[READ] There is an illegal address (0x{:x}) access.", addr),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x1FFF => self.work_ram.write(addr & 0x07FF, data),
            0x6000..=0x7FFF => {
                println!(
                    "Not implemented. This area is battery backup ram area 0x{:x}",
                    addr
                );
            }
            _ => panic!("[WRITE] There is an illegal address (0x{:x}) access.", addr),
        };
    }
}
