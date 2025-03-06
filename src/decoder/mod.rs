use crate::bytecode::{Displacement, ModRM, Operand, Sib};

pub struct Decoder {
    buffer: Vec<u8>,
    current: usize
}

impl Decoder {

    pub fn from_file(file: &str) -> Result<Self, String> {
        let file = std::fs::read(file).map_err(|x| "File Not Found".to_string())?;
        Ok(Self::new(file))
    }
    pub fn new(buffer: Vec<u8>) -> Self {
        Decoder {
            buffer,
            current: 0
        }
    }

    pub fn is_eof(&self) -> bool {
        self.current == self.buffer.len()
    }

    pub fn next(&mut self) -> Result<u8, String> {
        if self.is_eof() {
            return Err("EOF".to_string());
        }
        self.current += 1;

        self.buffer.get(self.current-1).copied().ok_or("EOF".to_string())
    }


    pub fn decode_operand(&mut self, modrm: ModRM) -> Result<Operand, String>  {
        let mut operand = Operand::default();
        operand.mod_rm = Some(modrm);
        if modrm.rm == 0b100 {
            operand.sib = Some(Sib::from_byte(self.next()?))
        }

        if modrm.mod_ == 0b01 {
            operand.displacement = Some(Displacement {
                displacement: self.next()? as usize,
                displacement_size: 8
            })
        } else if modrm.mod_ == 0b10 {
            let bytes = [self.next()?, self.next()?, self.next()?, self.next()?];
            operand.displacement = Some(Displacement {
                displacement: u32::from_le_bytes(bytes) as usize,
                displacement_size: 32
            })
        }

        Ok(operand)
    }
}