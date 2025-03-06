use crate::ast::Bits;
use crate::bytecode::{Displacement, Instr, ModRM, Parse, Prefix, Rex, Sib};
use crate::decoder::Decoder;
use crate::instr;
use paste::paste;

instr!(struct MovV8R8(0x88));

impl Parse for MovV8R8 {
    fn parse(buffer: &mut Decoder, _bits: Bits) -> Result<Instr, String> {
        let mut instr = Instr::default();
        instr.opcode = 0x88;
        buffer.next()?;
        let modrm = ModRM::decode(buffer.next()?);
        instr.operand = buffer.decode_operand(modrm)?;

        Ok(instr)
    }
}

instr!(struct Mov64V8R8(0x88));

impl Parse for Mov64V8R8 {
    fn parse(buffer: &mut Decoder, _bits: Bits) -> Result<Instr, String> {
        let mut instr = Instr::default();
        instr.prefix = Some(
            Prefix::Rex(Rex::try_from(buffer.next()?)?)
        );

        instr.opcode = 0x88;
        buffer.next()?;
        let modrm = ModRM::decode(buffer.next()?);
        instr.operand = buffer.decode_operand(modrm)?;
        Ok(instr)

    }
}

instr!(struct MovV16R16(0x89));

impl Parse for MovV16R16 {
    fn parse(buffer: &mut Decoder, bits: Bits) -> Result<Instr, String> {
        let mut instr = Instr::default();
        if bits != Bits::Bit16 {
            instr.prefix = Some(Prefix::OperandSize);
            buffer.next()?;
        }

        instr.opcode = 0x89;
        buffer.next()?;
        let modrm = ModRM::decode(buffer.next()?);
        instr.operand = buffer.decode_operand(modrm)?;

        Ok(instr)
    }
}