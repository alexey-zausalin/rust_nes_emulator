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
        }
    }
    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opcode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                /* LDA immediate */
                0xa9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;

                    self.register_a = param;

                    self.update_zero_flag(self.register_a);
                    self.update_negative_flag(self.register_a);
                }
                /* LDX immediate */
                0xa2 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;

                    self.register_x = param;

                    self.update_zero_flag(self.register_x);
                    self.update_negative_flag(self.register_x);
                }
                /* LDY immediate */
                0xa0 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;

                    self.register_y = param;

                    self.update_zero_flag(self.register_y);
                    self.update_negative_flag(self.register_y);
                }
                /* BRK */
                0x00 => {
                    self.flags.insert(CpuFlags::BREAK);
                    return;
                }
                _ => todo!(""),
            }
        }
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
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_ne!(cpu.flags.contains(CpuFlags::ZERO), true);
        assert_ne!(cpu.flags.contains(CpuFlags::NEGATIV), true);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.flags.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_0xa2_ldx_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa2, 0x05, 0x00]);
        assert_eq!(cpu.register_x, 0x05);
        assert_ne!(cpu.flags.contains(CpuFlags::ZERO), true);
        assert_ne!(cpu.flags.contains(CpuFlags::NEGATIV), true);
    }

    #[test]
    fn test_0xa2_ldx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa2, 0x00, 0x00]);
        assert!(cpu.flags.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_0xa0_ldy_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa0, 0x05, 0x00]);
        assert_eq!(cpu.register_y, 0x05);
        assert_ne!(cpu.flags.contains(CpuFlags::ZERO), true);
        assert_ne!(cpu.flags.contains(CpuFlags::NEGATIV), true);
    }

    #[test]
    fn test_0xa0_ldy_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa0, 0x00, 0x00]);
        assert!(cpu.flags.contains(CpuFlags::ZERO));
    }
}
