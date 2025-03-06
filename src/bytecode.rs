mod mov;

pub use mov::*;

use crate::ast::Bits;
use crate::decoder::Decoder;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rex {
    pub is_64: bool,
    pub reg_ext: bool,
    pub index_ext: bool,
    pub base_ext: bool
}

impl Rex {
    pub fn is_rex_prefix(val: u8) -> bool {
        val >= 0x40 && val <= 0x4F
    }

    pub fn to_byte(&self) -> u8 {
        0x40 |
            ((self.is_64 as u8) << 3) |
            ((self.reg_ext as u8) << 2) |
            ((self.index_ext as u8) << 1) |
            (self.base_ext as u8)
    }
}

impl Default for Rex {
    fn default() -> Self {
        Rex {
            is_64: false,
            reg_ext: false,
            index_ext: false,
            base_ext: false
        }
    }
}

impl TryFrom<u8>  for Rex {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let s = value >> 4;
        if s != 4 {
            Err(format!("{}", s))
        } else {
            let sig = value & 0xF;
            let is_64 = sig & 0b1000 != 0;
            let reg_ext = sig & 0b0100 != 0;
            let index_ext = sig & 0b0010 != 0;
            let base_ext = sig & 0b0001 != 0;

            Ok(Rex {
                is_64,
                reg_ext,
                index_ext,
                base_ext
            })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Segment {
    CS = 0x2E,
    SS = 0x36,
    DS = 0x3E,
    ES = 0x26,
    FS = 0x64,
    GS = 0x65
}

impl From<Segment> for u8 {
    fn from(segment: Segment) -> Self {
        segment as u8
    }
}

impl From<u8> for Segment {
    fn from(value: u8) -> Self {
        match value {
            0x2E => Segment::CS,
            0x36 => Segment::SS,
            0x3E => Segment::DS,
            0x26 => Segment::ES,
            0x64 => Segment::FS,
            0x65 => Segment::GS,
            _ => panic!("Unreachable"),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Prefix {
    Rex(Rex),
    OperandSize,
    AddressSize,
    Segment(Segment),
    Lock,
    RepNZ,
    RepZ,

}

impl Prefix {
    fn opcode(&self) -> u8 {
        match self {
            Prefix::Rex(rex) => rex.to_byte(),
            Prefix::OperandSize => 0x66,
            Prefix::AddressSize => 0x67,
            Prefix::Segment(seg) => seg.clone().into(),
            Prefix::Lock => 0xF0,
            Prefix::RepZ => 0xF2,
            Prefix::RepNZ => 0xF3

        }
    }
}

impl TryFrom<u8> for Prefix {

    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x40..0x4F => Prefix::Rex(Rex::try_from(value)?),
            0x66 => Prefix::OperandSize,
            0x67 => Prefix::AddressSize,
            0x2E | 0x36 | 0x3E | 0x26 | 0x64 | 0x65 => Prefix::Segment(Segment::from(value)),
            0xF0 => Prefix::Lock,
            0xF2 => Prefix::RepNZ,
            0xF3 => Prefix::RepNZ,
            _ => return Err("Prefix not found".to_string())
        })
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModRM {
    pub(crate) mod_: u8,
    reg: u8,
    pub(crate) rm: u8,
}

impl ModRM {
    fn decode(byte: u8) -> Self {
        Self {
            mod_: (byte >> 6) & 0b11,
            reg: (byte >> 3) & 0b111,
            rm: byte & 0b111,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sib {
    scale: u8,
    index: u8,
    base: u8,
}

impl Sib {
    pub fn from_byte(sib: u8) -> Self {
        let scale = (sib >> 6) & 0b11;
        let index = (sib >> 3) & 0b111;
        let base = sib & 0b111;

        Self { scale, index, base }
    }

    pub fn scale_factor(&self) -> u8 {
        match self.scale {
            0b00 => 1,
            0b01 => 2,
            0b10 => 4,
            0b11 => 8,
            _ => unreachable!(),
        }
    }

}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Displacement {
    pub(crate) displacement: usize,
    pub(crate) displacement_size: i32
}


#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Instr {
    pub prefix: Option<Prefix>,
    pub opcode: u8,
    pub operand: Operand
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Operand {
    pub mod_rm: Option<ModRM>,
    pub sib: Option<Sib>,
    pub displacement: Option<Displacement>,
    pub immediate: Option<u64>
}



pub trait Parse {
    fn parse(buffer: &mut Decoder, mode: Bits) -> Result<Instr, String>;
}


#[macro_export]
macro_rules! instr {
    (struct $name:ident($opcode:expr)) => {
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct $name;

        paste! {
            const [<$name _OPCODE>]: u8 = $opcode;
        }
    };
}





