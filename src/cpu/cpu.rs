use bitflags::bitflags;

bitflags! {
/// # Status Register (P) http://wiki.nesdev.com/w/index.php/Status_flags
///
///  7 6 5 4 3 2 1 0
///  N V _ B D I Z C
///  | |   | | | | +--- Carry Flag
///  | |   | | | +----- Zero Flag
///  | |   | | +------- Interrupt Disable
///  | |   | +--------- Decimal Mode (not used on NES)
///  | |   +----------- Break Command
///  | +--------------- Overflow Flag
///  +----------------- Negative Flag
///
    pub struct CpuFlags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const BREAK2            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIV           = 0b10000000;
    }
}

pub struct CPU {
    pub program_counter: u16,
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub flags: CpuFlags,
    memory: [u8; 0xffff],
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            program_counter: 0,
            register_a: 0,
            register_x: 0,
            register_y: 0,
            flags: CpuFlags::from_bits_truncate(0b0010_0100),
            memory: [0; 0xffff],
        }
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.program_counter = 0x8000;
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            match opcode {
                /* BRK */
                0x00 => {
                    self.flags.insert(CpuFlags::BREAK);
                    return;
                }
                /* INX */
                0xe8 => {
                    self.register_x = self.register_x.wrapping_add(1);

                    self.update_zero_flag(self.register_y);
                    self.update_negative_flag(self.register_y);
                }
                /* INY */
                0xc8 => {
                    self.register_y = self.register_y.wrapping_add(1);

                    self.update_zero_flag(self.register_y);
                    self.update_negative_flag(self.register_y);
                }
                /* LDA immediate */
                0xa9 => {
                    let param = self.mem_read(self.program_counter);
                    self.program_counter += 1;

                    self.register_a = param;

                    self.update_zero_flag(self.register_a);
                    self.update_negative_flag(self.register_a);
                }
                /* LDX immediate */
                0xa2 => {
                    let param = self.mem_read(self.program_counter);
                    self.program_counter += 1;

                    self.register_x = param;

                    self.update_zero_flag(self.register_x);
                    self.update_negative_flag(self.register_x);
                }
                /* LDY immediate */
                0xa0 => {
                    let param = self.mem_read(self.program_counter);
                    self.program_counter += 1;

                    self.register_y = param;

                    self.update_zero_flag(self.register_y);
                    self.update_negative_flag(self.register_y);
                }
                /* TAX */
                0xaa => {
                    self.register_x = self.register_a;

                    self.update_zero_flag(self.register_x);
                    self.update_negative_flag(self.register_x);
                }
                /* TAY */
                0xa8 => {
                    self.register_y = self.register_a;

                    self.update_zero_flag(self.register_y);
                    self.update_negative_flag(self.register_y);
                }
                _ => todo!(""),
            }
        }
    }

    fn mem_read(&mut self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn update_zero_flag(&mut self, last_operation: u8) {
        if last_operation == 0 {
            self.flags.insert(CpuFlags::ZERO);
        } else {
            self.flags.remove(CpuFlags::ZERO);
        }
    }

    fn update_negative_flag(&mut self, last_operation: u8) {
        if last_operation >> 7 == 1 {
            self.flags.insert(CpuFlags::NEGATIV)
        } else {
            self.flags.remove(CpuFlags::NEGATIV)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_ne!(cpu.flags.contains(CpuFlags::ZERO), true);
        assert_ne!(cpu.flags.contains(CpuFlags::NEGATIV), true);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.flags.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_0xa2_ldx_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
        assert_eq!(cpu.register_x, 0x05);
        assert_ne!(cpu.flags.contains(CpuFlags::ZERO), true);
        assert_ne!(cpu.flags.contains(CpuFlags::NEGATIV), true);
    }

    #[test]
    fn test_0xa2_ldx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x00, 0x00]);
        assert!(cpu.flags.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_0xa0_ldy_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05, 0x00]);
        assert_eq!(cpu.register_y, 0x05);
        assert_ne!(cpu.flags.contains(CpuFlags::ZERO), true);
        assert_ne!(cpu.flags.contains(CpuFlags::NEGATIV), true);
    }

    #[test]
    fn test_0xa0_ldy_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x00, 0x00]);
        assert!(cpu.flags.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.load_and_run(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_iny_overflow() {
        let mut cpu = CPU::new();
        cpu.register_y = 0xff;
        cpu.load_and_run(vec![0xc8, 0xc8, 0x00]);

        assert_eq!(cpu.register_y, 1)
    }

    #[test]
    fn test_lda_tax_inx_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_lda_tay_iny_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xa8, 0xc8, 0x00]);

        assert_eq!(cpu.register_y, 0xc1)
    }
}
