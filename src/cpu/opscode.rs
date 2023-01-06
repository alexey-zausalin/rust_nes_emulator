use crate::cpu::mem::AddressingMode;
use std::collections::HashMap;

pub struct OpsCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpsCode {
    fn new(code: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpsCode {
            code,
            mnemonic,
            len,
            cycles,
            mode,
        }
    }
}

lazy_static! {
    pub static ref CPU_OPS_CODES: Vec<OpsCode> = vec![
        OpsCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
        OpsCode::new(0xea, "NOP", 1, 2, AddressingMode::NoneAddressing),

        /* Arithmetic */
        OpsCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
        OpsCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),

        /* Shifts */
        OpsCode::new(0x4a, "LSR", 1, 2, AddressingMode::Accumulator),

        OpsCode::new(0xe6, "INC", 2, 5, AddressingMode::ZeroPage),

        OpsCode::new(0xe8, "INX", 1, 2, AddressingMode::NoneAddressing),
        OpsCode::new(0xc8, "INY", 1, 2, AddressingMode::NoneAddressing),

        OpsCode::new(0xca, "DEX", 1, 2, AddressingMode::NoneAddressing),


        /* Comparisons */
        OpsCode::new(0xc9, "CMP", 2, 2, AddressingMode::Immediate),
        OpsCode::new(0xc5, "CMP", 2, 3, AddressingMode::ZeroPage),

        OpsCode::new(0xe4, "CPX", 2, 3, AddressingMode::ZeroPage),


        /* Branching */
        OpsCode::new(0x20, "JSR", 3, 6, AddressingMode::NoneAddressing),

        OpsCode::new(0x60, "RTS", 1, 6, AddressingMode::NoneAddressing),

        OpsCode::new(0xd0, "BNE", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
        OpsCode::new(0xf0, "BEQ", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
        OpsCode::new(0xb0, "BCS", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
        OpsCode::new(0x10, "BPL", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),

        OpsCode::new(0x24, "BIT", 2, 3, AddressingMode::ZeroPage),


        /* Stores, Loads */
        OpsCode::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate),
        OpsCode::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpsCode::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
        OpsCode::new(0xad, "LDA", 3, 4, AddressingMode::Absolute),
        OpsCode::new(0xbd, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X_PageCross),
        OpsCode::new(0xb9, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y_PageCross),
        OpsCode::new(0xa1, "LDA", 2, 6, AddressingMode::Indirect_X),
        OpsCode::new(0xb1, "LDA", 2, 5/*+1 if page crossed*/, AddressingMode::Indirect_Y_PageCross),

        OpsCode::new(0xa2, "LDX", 2, 2, AddressingMode::Immediate),
        OpsCode::new(0xa6, "LDX", 2, 3, AddressingMode::ZeroPage),
        OpsCode::new(0xb6, "LDX", 2, 4, AddressingMode::ZeroPage_Y),
        OpsCode::new(0xae, "LDX", 3, 4, AddressingMode::Absolute),
        OpsCode::new(0xbe, "LDX", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y_PageCross),

        OpsCode::new(0xa0, "LDY", 2, 2, AddressingMode::Immediate),
        OpsCode::new(0xa4, "LDY", 2, 3, AddressingMode::ZeroPage),
        OpsCode::new(0xb4, "LDY", 2, 4, AddressingMode::ZeroPage_X),
        OpsCode::new(0xac, "LDY", 3, 4, AddressingMode::Absolute),
        OpsCode::new(0xbc, "LDY", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X_PageCross),

        OpsCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        OpsCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
        OpsCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
        OpsCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),

        /* Flags clear */
        OpsCode::new(0x18, "CLC", 1, 2, AddressingMode::NoneAddressing),

        OpsCode::new(0xaa, "TAX", 1, 2, AddressingMode::NoneAddressing),
        OpsCode::new(0xa8, "TAY", 1, 2, AddressingMode::NoneAddressing),
        OpsCode::new(0x8a, "TXA", 1, 2, AddressingMode::NoneAddressing),
    ];

    pub static ref OPSCODES_MAP: HashMap<u8, &'static OpsCode> = {
        let mut map = HashMap::new();
        for cpuop in &*CPU_OPS_CODES {
            map.insert(cpuop.code, cpuop);
        }
        map
    };
}
