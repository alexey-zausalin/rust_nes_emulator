pub struct CPU {
    pub program_counter: u16,
    pub register_a: u8,
    pub status: u8,
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
            status: 0,
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

                    if self.register_a == 0 {
                        self.status |= 0b0000_0010;
                    } else {
                        self.status &= 0b1111_1101;
                    }

                    if self.register_a & 0b1000_0000 != 0 {
                        self.status |= 0b1000_0000;
                    } else {
                        self.status &= 0b0111_1111;
                    }
                }
                /* BRK */
                0x00 => {
                    self.status |= 0b0001_0000;
                    return;
                }
                _ => todo!(""),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.status & 0b0000_0010, 0b00);
        assert_eq!(cpu.status & 0b1000_0000, 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.status & 0b0000_0010, 0b10);
    }
}
