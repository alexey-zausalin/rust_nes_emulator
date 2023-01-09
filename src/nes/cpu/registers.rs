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
        const RESERVED          = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIV           = 0b10000000;
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Registers {
    A: u8,
    X: u8,
    Y: u8,
    SP: u8,
    PC: u16,
    P: CpuFlags,
}

#[allow(non_snake_case)]
pub trait CpuRegisters {
    fn get_PC(&self) -> u16;

    fn get_A(&self) -> u8;

    fn get_X(&self) -> u8;

    fn get_Y(&self) -> u8;

    fn get_SP(&self) -> u8;

    fn get_P(&self) -> u8;

    fn set_A(&mut self, v: u8) -> &mut Self;

    fn set_X(&mut self, v: u8) -> &mut Self;

    fn set_Y(&mut self, v: u8) -> &mut Self;

    fn set_PC(&mut self, v: u16) -> &mut Self;

    fn set_P(&mut self, v: u8) -> &mut Self;

    fn set_SP(&mut self, v: u8) -> &mut Self;

    fn set_negative(&mut self, v: bool) -> &mut Self;

    fn set_overflow(&mut self, v: bool) -> &mut Self;

    fn set_reserved(&mut self, v: bool) -> &mut Self;

    fn set_break(&mut self, v: bool) -> &mut Self;

    fn set_interrupt(&mut self, v: bool) -> &mut Self;

    fn set_zero(&mut self, v: bool) -> &mut Self;

    fn set_decimal(&mut self, v: bool) -> &mut Self;

    fn set_carry(&mut self, v: bool) -> &mut Self;

    fn get_negative(&self) -> bool;

    fn get_overflow(&self) -> bool;

    fn get_reserved(&self) -> bool;

    fn get_break(&self) -> bool;

    fn get_interrupt(&self) -> bool;

    fn get_zero(&self) -> bool;

    fn get_decimal(&self) -> bool;

    fn get_carry(&self) -> bool;

    fn update_negative_by(&mut self, v: u8) -> &mut Self;

    fn update_zero_by(&mut self, v: u8) -> &mut Self;

    fn inc_SP(&mut self) -> &mut Self;

    fn dec_SP(&mut self) -> &mut Self;

    fn inc_PC(&mut self) -> &mut Self;

    fn dec_PC(&mut self) -> &mut Self;
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            A: 0,
            X: 0,
            Y: 0,
            PC: 0x8000,
            SP: 0xFD,
            P: CpuFlags::from_bits_truncate(0b00110100),
        }
    }
}

#[allow(non_snake_case)]
impl CpuRegisters for Registers {
    fn get_PC(&self) -> u16 {
        self.PC
    }

    fn get_A(&self) -> u8 {
        self.A
    }

    fn get_X(&self) -> u8 {
        self.X
    }

    fn get_Y(&self) -> u8 {
        self.Y
    }

    fn get_SP(&self) -> u8 {
        self.SP
    }

    fn get_P(&self) -> u8 {
        self.P.bits
    }

    fn set_A(&mut self, v: u8) -> &mut Self {
        self.A = v;
        self
    }

    fn set_X(&mut self, v: u8) -> &mut Self {
        self.X = v;
        self
    }

    fn set_Y(&mut self, v: u8) -> &mut Self {
        self.Y = v;
        self
    }

    fn set_PC(&mut self, v: u16) -> &mut Self {
        self.PC = v;
        self
    }

    fn set_P(&mut self, v: u8) -> &mut Self {
        self.P = CpuFlags::from_bits_truncate(v);
        self
    }

    fn set_SP(&mut self, v: u8) -> &mut Self {
        self.SP = v;
        self
    }

    fn set_negative(&mut self, v: bool) -> &mut Self {
        self.P.set(CpuFlags::NEGATIV, v);
        self
    }

    fn set_overflow(&mut self, v: bool) -> &mut Self {
        self.P.set(CpuFlags::OVERFLOW, v);
        self
    }

    fn set_reserved(&mut self, v: bool) -> &mut Self {
        self.P.set(CpuFlags::RESERVED, v);
        self
    }

    fn set_break(&mut self, v: bool) -> &mut Self {
        self.P.set(CpuFlags::BREAK, v);
        self
    }

    fn set_interrupt(&mut self, v: bool) -> &mut Self {
        self.P.set(CpuFlags::INTERRUPT_DISABLE, v);
        self
    }

    fn set_zero(&mut self, v: bool) -> &mut Self {
        self.P.set(CpuFlags::ZERO, v);
        self
    }

    fn set_decimal(&mut self, v: bool) -> &mut Self {
        self.P.set(CpuFlags::DECIMAL_MODE, v);
        self
    }

    fn set_carry(&mut self, v: bool) -> &mut Self {
        self.P.set(CpuFlags::CARRY, v);
        self
    }

    fn get_negative(&self) -> bool {
        self.P.contains(CpuFlags::NEGATIV)
    }

    fn get_overflow(&self) -> bool {
        self.P.contains(CpuFlags::OVERFLOW)
    }

    fn get_reserved(&self) -> bool {
        self.P.contains(CpuFlags::RESERVED)
    }

    fn get_break(&self) -> bool {
        self.P.contains(CpuFlags::BREAK)
    }

    fn get_interrupt(&self) -> bool {
        self.P.contains(CpuFlags::INTERRUPT_DISABLE)
    }

    fn get_zero(&self) -> bool {
        self.P.contains(CpuFlags::ZERO)
    }

    fn get_decimal(&self) -> bool {
        self.P.contains(CpuFlags::DECIMAL_MODE)
    }

    fn get_carry(&self) -> bool {
        self.P.contains(CpuFlags::CARRY)
    }

    fn update_negative_by(&mut self, v: u8) -> &mut Self {
        self.set_negative(v & 0x80 == 0x80);
        self
    }

    fn update_zero_by(&mut self, v: u8) -> &mut Self {
        self.set_zero(v == 0);
        self
    }

    fn inc_SP(&mut self) -> &mut Self {
        self.SP += 1;
        self
    }

    fn dec_SP(&mut self) -> &mut Self {
        self.SP -= 1;
        self
    }

    fn inc_PC(&mut self) -> &mut Self {
        self.PC += 1;
        self
    }

    fn dec_PC(&mut self) -> &mut Self {
        self.PC -= 1;
        self
    }
}

#[test]
fn get_p() {
    let reg = Registers::new();
    let p = reg.get_P();
    assert_eq!(p, 0x34);
}

#[test]
fn update_zero() {
    let mut reg = Registers::new();
    reg.update_zero_by(0);
    let p = reg.get_P();
    assert_eq!(p, 0x36);
}

#[test]
fn update_negative() {
    let mut reg = Registers::new();
    reg.update_negative_by(0x80);
    let p = reg.get_P();
    assert_eq!(p, 0xB4);
}
