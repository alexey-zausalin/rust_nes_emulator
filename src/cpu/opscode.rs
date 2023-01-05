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

        /* Shifts */
        OpsCode::new(0xe8, "INX", 1, 2, AddressingMode::NoneAddressing),
        OpsCode::new(0xc8, "INY", 1, 2, AddressingMode::NoneAddressing),

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

        /* Flags clear */
        OpsCode::new(0xaa, "TAX", 1, 2, AddressingMode::NoneAddressing),
        OpsCode::new(0xa8, "TAY", 1, 2, AddressingMode::NoneAddressing),
    ];

    pub static ref OPSCODES_MAP: HashMap<u8, &'static OpsCode> = {
        let mut map = HashMap::new();
        for cpuop in &*CPU_OPS_CODES {
            map.insert(cpuop.code, cpuop);
        }
        map
    };
}
