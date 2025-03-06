use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bits {
    Bit8,
    Bit16,
    Bit32,
    Bit64
}

impl Into<u32> for Bits {
    fn into(self) -> u32 {
        match self {
            Bits::Bit8 => 8,
            Bits::Bit16 => 16,
            Bits::Bit32 => 32,
            Bits::Bit64 => 64
        }
    }
}


pub enum Value {
    Memory(Box<Value>),
    Register(Register),
    Immediate(Immediate)
}

pub struct Immediate {
    bits: Bits,
    val: usize
}

impl Immediate {
    pub fn new(bits: Bits, val: usize) -> Self {
        Self { bits, val }
    }

    pub fn bits(&self) -> Bits {
        self.bits
    }
}

impl From<u8> for Immediate {
    fn from(value: u8) -> Self {
        Self::new(Bits::Bit8, value as usize)
    }
}

impl From<u16> for Immediate {
    fn from(value: u16) -> Self {
        Self::new(Bits::Bit16, value as usize)
    }
}


impl From<u32> for Immediate {
    fn from(value: u32) -> Self {
        Self::new(Bits::Bit32, value as usize)
    }
}


impl From<u64> for Immediate {
    fn from(value: u64) -> Self {
        Self::new(Bits::Bit64, value as usize)
    }
}

pub struct Register {
    bits: Bits,
    kind: RegisterKind
}



impl Register {
    pub fn new(kind: RegisterKind, bits: Bits) -> Result<Self, String> {
        if kind.is_segment_register() && bits != Bits::Bit16 {
            return Err("segment register must be 16 bit".to_string());
        }

        if kind == RegisterKind::IP && bits == Bits::Bit8 {
            return Err("Instruction Pointer can't be 8 bit".to_string())
        }

        if kind.is_high_8bit() && bits != Bits::Bit16 {
            return Err("High 8bit must be 16 bit".to_string());
        }

        Ok(
            Register {
                kind,
                bits
            }
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RegisterKind {
    Accumulator,
    Base,
    Counter,
    Data,
    SourceIndex,
    DestIndex,
    StackPtr,
    BasePtr,
    Gp1,
    Gp2,
    Gp3,
    Gp4,
    Gp5,
    Gp6,
    Gp7,
    AH,
    BH,
    CH,
    DH,
    IP,
    CS,
    DS,
    SS,
    ES,
    FS,
    GS
}

impl RegisterKind {
    pub fn is_high_8bit(&self) -> bool {
        matches!(self, RegisterKind::AH | RegisterKind::BH | RegisterKind::CH | RegisterKind::DH)
    }
    pub fn is_segment_register(&self) -> bool {
        matches!(self, RegisterKind::CS | RegisterKind::DS | RegisterKind::SS | RegisterKind::ES | RegisterKind::FS | RegisterKind::GS)
    }
}