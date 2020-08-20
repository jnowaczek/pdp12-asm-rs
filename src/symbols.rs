pub struct symbol {
    name: str,
    value: u16,
    class: Option(INSTRUCTION_CLASS),
    base_value: Option(u16),
}

pub enum PROCESSOR_MODES {
    PMODE,
    LMODE
}

pub enum INSTRUCTION_CLASS {
    P_BASIC,
    P_MEMORY_REFERENCE,
    P_OPERATE_GROUP_1,
    P_OPERATE_GROUP_2,
    P_CLA,
    P_EXTENDED_ARITHMETIC,
    P_EXTENDED_ARITHMETIC_LONG,
    L_BASIC,
    L_DIRECT,
    L_ALPHA,
    L_BETA,
    L_TAPE,
    L_EXTENDED_ARITHMETIC,
    L_EXTENDED_ARITHMETIC_LONG
}

impl INSTRUCTION_CLASS {
    const fn MODE() -> PROCESSOR_MODES {
        match *self {
            Self::P_BASIC | Self::P_MEMORY_REFERENCE | Self::P_OPERATE_GROUP_1 | Self::P_OPERATE_GROUP_2 
            | Self::P_CLA | Self::P_EXTENDED_ARITHMETIC | Self::P_EXTENDED_ARITHMETIC_LONG => PROCESSOR_MODES::PMODE,
            Self::L_BASIC | Self::L_ALPHA | Self::L_BETA | Self::L_TAPE | Self::L_EXTENDED_ARITHMETIC 
            | Self::L_EXTENDED_ARITHMETIC_LONG => PROCESSOR_MODES::LMODE
        }
    }
}

const