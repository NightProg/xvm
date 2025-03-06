use iced_x86::Register;
#[derive(Debug, Clone, Copy, Default)]
pub struct GeneralPurposeRegisters {
    pub gp64: GpRegister64,
    pub gp32: GpRegister32,
    pub gp16: GpRegister16,
    pub gp8: GpRegister8,
    pub segment: SegmentRegisters,
}

impl GeneralPurposeRegisters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_register_value(&self, register: Register) -> u64 {
        match register {
            Register::AL => self.gp8.al as u64,
            Register::AH => self.gp8.ah as u64,
            Register::BL => self.gp8.bl as u64,
            Register::BH => self.gp8.bh as u64,
            Register::CL => self.gp8.cl as u64,
            Register::CH => self.gp8.ch as u64,
            Register::DL => self.gp8.dl as u64,
            Register::DH => self.gp8.dh as u64,
            Register::AX => self.gp16.ax as u64,
            Register::BX => self.gp16.bx as u64,
            Register::CX => self.gp16.cx as u64,
            Register::DX => self.gp16.dx as u64,
            Register::SI => self.gp16.si as u64,
            Register::DI => self.gp16.di as u64,
            Register::BP => self.gp16.bp as u64,
            Register::SP => self.gp16.sp as u64,
            Register::EAX => self.gp32.eax as u64,
            Register::EBX => self.gp32.ebx as u64,
            Register::ECX => self.gp32.ecx as u64,
            Register::EDX => self.gp32.edx as u64,
            Register::ESI => self.gp32.esi as u64,
            Register::EDI => self.gp32.edi as u64,
            Register::EBP => self.gp32.ebp as u64,
            Register::ESP => self.gp32.esp as u64,
            Register::RAX => self.gp64.rax,
            Register::RBX => self.gp64.rbx,
            Register::RCX => self.gp64.rcx,
            Register::RDX => self.gp64.rdx,
            Register::RSI => self.gp64.rsi,
            Register::RDI => self.gp64.rdi,
            Register::RBP => self.gp64.rbp,
            Register::RSP => self.gp64.rsp,
            Register::R8 => self.gp64.r8,
            Register::R9 => self.gp64.r9,
            Register::R10 => self.gp64.r10,
            Register::R11 => self.gp64.r11,
            Register::R12 => self.gp64.r12,
            Register::R13 => self.gp64.r13,
            Register::R14 => self.gp64.r14,
            Register::R15 => self.gp64.r15,
            Register::CS => self.segment.cs as u64,
            Register::DS => self.segment.ds as u64,
            Register::ES => self.segment.es as u64,
            Register::FS => self.segment.fs as u64,
            Register::GS => self.segment.gs as u64,
            Register::SS => self.segment.ss as u64,

            _ => 0
        }
    }

    pub fn set_register_value(&mut self, register: Register, value: usize) {
        match  register {
            Register::AL => self.set_al(value as u8),
            Register::AH => self.set_ah(value as u8),
            Register::BL => self.set_bl(value as u8),
            Register::BH => self.set_bh(value as u8),
            Register::CL => self.set_cl(value as u8),
            Register::CH => self.set_ch(value as u8),
            Register::DL => self.set_dl(value as u8),
            Register::DH => self.set_dh(value as u8),
            Register::AX => self.set_ax(value as u16),
            Register::BX => self.set_bx(value as u16),
            Register::CX => self.set_cx(value as u16),
            Register::DX => self.set_dx(value as u16),
            Register::SI => self.set_si(value as u16),
            Register::DI => self.set_di(value as u16),
            Register::BP => self.set_bp(value as u16),
            Register::SP => self.set_sp(value as u16),
            Register::EAX => self.set_eax(value as u32),
            Register::EBX => self.set_ebx(value as u32),
            Register::ECX => self.set_ecx(value as u32),
            Register::EDX => self.set_edx(value as u32),
            Register::ESI => self.set_esi(value as u32),
            Register::EDI => self.set_edi(value as u32),
            Register::EBP => self.set_ebp(value as u32),
            Register::ESP => self.set_esp(value as u32),
            Register::RAX => self.set_rax(value as u64),
            Register::RBX => self.set_rbx(value as u64),
            Register::RCX => self.set_rcx(value as u64),
            Register::RDX => self.set_rdx(value as u64),
            Register::RSI => self.set_rsi(value as u64),
            Register::RDI => self.set_rdi(value as u64),
            Register::RBP => self.set_rbp(value as u64),
            Register::RSP => self.set_rsp(value as u64),
            Register::R8 => self.set_r8(value as u64),
            Register::R9 => self.set_r9(value as u64),
            Register::R10 => self.set_r10(value as u64),
            Register::R11 => self.set_r11(value as u64),
            Register::R12 => self.set_r12(value as u64),
            Register::R13 => self.set_r13(value as u64),
            Register::R14 => self.set_r14(value as u64),
            Register::R15 => self.set_r15(value as u64),
            Register::CS => self.segment.cs = value as u16,
            Register::DS => self.segment.ds = value as u16,
            Register::ES => self.segment.es = value as u16,
            Register::FS => self.segment.fs = value as u16,
            Register::GS => self.segment.gs = value as u16,
            Register::SS => self.segment.ss = value as u16,

            _ => {}
        }
    }

    pub fn set_al(&mut self, val: u8) {
        self.gp8.al = val;
        self.gp16.ax = val as u16;
        self.gp32.eax = val as u32;
        self.gp64.rax = val as u64;
    }

    pub fn set_ah(&mut self, val: u8) {
        self.gp8.ah = val;
        self.gp16.ax = (self.gp8.ah as u16) << 8 | self.gp8.al as u16;
        self.gp32.eax = (self.gp16.ax as u32) << 16 | self.gp16.ax as u32;
        self.gp64.rax = (self.gp32.eax as u64) << 32 | self.gp32.eax as u64;
    }

    pub fn set_bl(&mut self, val: u8) {
        self.gp8.bl = val;
        self.gp16.bx = val as u16;
        self.gp32.ebx = val as u32;
        self.gp64.rbx = val as u64;
    }

    pub fn set_bh(&mut self, val: u8) {
        self.gp8.bh = val;
        self.gp16.bx = (self.gp8.bh as u16) << 8 | self.gp8.bl as u16;
        self.gp32.ebx = (self.gp16.bx as u32) << 16 | self.gp16.bx as u32;
        self.gp64.rbx = (self.gp32.ebx as u64) << 32 | self.gp32.ebx as u64;
    }

    pub fn set_cl(&mut self, val: u8) {
        self.gp8.cl = val;
        self.gp16.cx = val as u16;
        self.gp32.ecx = val as u32;
        self.gp64.rcx = val as u64;
    }

    pub fn set_ch(&mut self, val: u8) {
        self.gp8.ch = val;
        self.gp16.cx = (self.gp8.ch as u16) << 8 | self.gp8.cl as u16;
        self.gp32.ecx = (self.gp16.cx as u32) << 16 | self.gp16.cx as u32;
        self.gp64.rcx = (self.gp32.ecx as u64) << 32 | self.gp32.ecx as u64;
    }

    pub fn set_dl(&mut self, val: u8) {
        self.gp8.dl = val;
        self.gp16.dx = val as u16;
        self.gp32.edx = val as u32;
        self.gp64.rdx = val as u64;
    }

    pub fn set_dh(&mut self, val: u8) {
        self.gp8.dh = val;
        self.gp16.dx = (self.gp8.dh as u16) << 8 | self.gp8.dl as u16;
        self.gp32.edx = (self.gp16.dx as u32) << 16 | self.gp16.dx as u32;
        self.gp64.rdx = (self.gp32.edx as u64) << 32 | self.gp32.edx as u64;
    }

    pub fn set_ax(&mut self, val: u16) {
        self.gp16.ax = val;
        self.gp32.eax = val as u32;
        self.gp64.rax = val as u64;
    }

    pub fn set_bx(&mut self, val: u16) {
        self.gp16.bx = val;
        self.gp32.ebx = val as u32;
        self.gp64.rbx = val as u64;
    }

    pub fn set_cx(&mut self, val: u16) {
        self.gp16.cx = val;
        self.gp32.ecx = val as u32;
        self.gp64.rcx = val as u64;
    }

    pub fn set_dx(&mut self, val: u16) {
        self.gp16.dx = val;
        self.gp32.edx = val as u32;
        self.gp64.rdx = val as u64;
    }

    pub fn set_si(&mut self, val: u16) {
        self.gp16.si = val;
        self.gp32.esi = val as u32;
        self.gp64.rsi = val as u64;
    }

    pub fn set_di(&mut self, val: u16) {
        self.gp16.di = val;
        self.gp32.edi = val as u32;
        self.gp64.rdi = val as u64;
    }

    pub fn set_bp(&mut self, val: u16) {
        self.gp16.bp = val;
        self.gp32.ebp = val as u32;
        self.gp64.rbp = val as u64;
    }

    pub fn set_sp(&mut self, val: u16) {
        self.gp16.sp = val;
        self.gp32.esp = val as u32;
        self.gp64.rsp = val as u64;
    }

    pub fn set_eax(&mut self, val: u32) {
        self.gp32.eax = val;
        self.gp64.rax = val as u64;
    }

    pub fn set_ebx(&mut self, val: u32) {
        self.gp32.ebx = val;
        self.gp64.rbx = val as u64;
    }

    pub fn set_ecx(&mut self, val: u32) {
        self.gp32.ecx = val;
        self.gp64.rcx = val as u64;
    }

    pub fn set_edx(&mut self, val: u32) {
        self.gp32.edx = val;
        self.gp64.rdx = val as u64;
    }

    pub fn set_esi(&mut self, val: u32) {
        self.gp32.esi = val;
        self.gp64.rsi = val as u64;
    }

    pub fn set_edi(&mut self, val: u32) {
        self.gp32.edi = val;
        self.gp64.rdi = val as u64;
    }

    pub fn set_ebp(&mut self, val: u32) {
        self.gp32.ebp = val;
        self.gp64.rbp = val as u64;
    }

    pub fn set_esp(&mut self, val: u32) {
        self.gp32.esp = val;
        self.gp64.rsp = val as u64;
    }

    pub fn set_rax(&mut self, val: u64) {
        self.gp64.rax = val;
    }

    pub fn set_rbx(&mut self, val: u64) {
        self.gp64.rbx = val;
    }

    pub fn set_rcx(&mut self, val: u64) {
        self.gp64.rcx = val;
    }

    pub fn set_rdx(&mut self, val: u64) {
        self.gp64.rdx = val;
    }

    pub fn set_rsi(&mut self, val: u64) {
        self.gp64.rsi = val;
    }

    pub fn set_rdi(&mut self, val: u64) {
        self.gp64.rdi = val;
    }

    pub fn set_rbp(&mut self, val: u64) {
        self.gp64.rbp = val;
    }

    pub fn set_rsp(&mut self, val: u64) {
        self.gp64.rsp = val;
    }

    pub fn set_r8(&mut self, val: u64) {
        self.gp64.r8 = val;
    }

    pub fn set_r9(&mut self, val: u64) {
        self.gp64.r9 = val;
    }

    pub fn set_r10(&mut self, val: u64) {
        self.gp64.r10 = val;
    }

    pub fn set_r11(&mut self, val: u64) {
        self.gp64.r11 = val;
    }

    pub fn set_r12(&mut self, val: u64) {
        self.gp64.r12 = val;
    }

    pub fn set_r13(&mut self, val: u64) {
        self.gp64.r13 = val;
    }

    pub fn set_r14(&mut self, val: u64) {
        self.gp64.r14 = val;
    }

    pub fn set_r15(&mut self, val: u64) {
        self.gp64.r15 = val;
    }



    pub fn set_u8(&mut self, dest: iced_x86::Register, val: u8) {
        match dest {
            iced_x86::Register::AL => self.set_al(val),
            iced_x86::Register::AH => self.set_ah(val),
            iced_x86::Register::BL => self.set_bl(val),
            iced_x86::Register::BH => self.set_bh(val),
            iced_x86::Register::CL => self.set_cl(val),
            iced_x86::Register::CH => self.set_ch(val),
            iced_x86::Register::DL => self.set_dl(val),
            iced_x86::Register::DH => self.set_dh(val),
            _ => {}
        }
    }

    pub fn get_u8(&self, val: iced_x86::Register) -> u8 {
        match val {
            iced_x86::Register::AL => self.gp8.al,
            iced_x86::Register::AH => self.gp8.ah,
            iced_x86::Register::BL => self.gp8.bl,
            iced_x86::Register::BH => self.gp8.bh,
            iced_x86::Register::CL => self.gp8.cl,
            iced_x86::Register::CH => self.gp8.ch,
            iced_x86::Register::DL => self.gp8.dl,
            iced_x86::Register::DH => self.gp8.dh,
            _ => 0
        }
    }

    pub fn set_u16(&mut self, dest: iced_x86::Register, val: u16) {
        match dest {
            iced_x86::Register::AX => self.set_ax(val),
            iced_x86::Register::BX => self.set_bx(val),
            iced_x86::Register::CX => self.set_cx(val),
            iced_x86::Register::DX => self.set_dx(val),
            iced_x86::Register::SI => self.set_si(val),
            iced_x86::Register::DI => self.set_di(val),
            iced_x86::Register::BP => self.set_bp(val),
            iced_x86::Register::SP => self.set_sp(val),
            _ => {}
        }
    }

    pub fn get_u16(&self, val: iced_x86::Register) -> u16 {
        match val {
            iced_x86::Register::AX => self.gp16.ax,
            iced_x86::Register::BX => self.gp16.bx,
            iced_x86::Register::CX => self.gp16.cx,
            iced_x86::Register::DX => self.gp16.dx,
            iced_x86::Register::SI => self.gp16.si,
            iced_x86::Register::DI => self.gp16.di,
            iced_x86::Register::BP => self.gp16.bp,
            iced_x86::Register::SP => self.gp16.sp,
            _ => 0
        }
    }

    pub fn set_u32(&mut self, dest: iced_x86::Register, val: u32) {
        match dest {
            iced_x86::Register::EAX => self.set_eax(val),
            iced_x86::Register::EBX => self.set_ebx(val),
            iced_x86::Register::ECX => self.set_ecx(val),
            iced_x86::Register::EDX => self.set_edx(val),
            iced_x86::Register::ESI => self.set_esi(val),
            iced_x86::Register::EDI => self.set_edi(val),
            iced_x86::Register::EBP => self.set_ebp(val),
            iced_x86::Register::ESP => self.set_esp(val),
            _ => {}
        }
    }

    pub fn get_u32(&self, val: iced_x86::Register) -> u32 {
        match val {
            iced_x86::Register::EAX => self.gp32.eax,
            iced_x86::Register::EBX => self.gp32.ebx,
            iced_x86::Register::ECX => self.gp32.ecx,
            iced_x86::Register::EDX => self.gp32.edx,
            iced_x86::Register::ESI => self.gp32.esi,
            iced_x86::Register::EDI => self.gp32.edi,
            iced_x86::Register::EBP => self.gp32.ebp,
            iced_x86::Register::ESP => self.gp32.esp,
            _ => 0
        }
    }

    pub fn set_u64(&mut self, dest: iced_x86::Register, val: u64) {
        match dest {
            iced_x86::Register::RAX => self.set_rax(val),
            iced_x86::Register::RBX => self.set_rbx(val),
            iced_x86::Register::RCX => self.set_rcx(val),
            iced_x86::Register::RDX => self.set_rdx(val),
            iced_x86::Register::RSI => self.set_rsi(val),
            iced_x86::Register::RDI => self.set_rdi(val),
            iced_x86::Register::RBP => self.set_rbp(val),
            iced_x86::Register::RSP => self.set_rsp(val),
            iced_x86::Register::R8 => self.set_r8(val),
            iced_x86::Register::R9 => self.set_r9(val),
            iced_x86::Register::R10 => self.set_r10(val),
            iced_x86::Register::R11 => self.set_r11(val),
            iced_x86::Register::R12 => self.set_r12(val),
            iced_x86::Register::R13 => self.set_r13(val),
            iced_x86::Register::R14 => self.set_r14(val),
            iced_x86::Register::R15 => self.set_r15(val),
            _ => {}
        }
    }

    pub fn get_u64(&mut self, dest: iced_x86::Register, val: u64) -> u64 {
        match dest {
            iced_x86::Register::RAX => self.gp64.rax,
            iced_x86::Register::RBX => self.gp64.rbx,
            iced_x86::Register::RCX => self.gp64.rcx,
            iced_x86::Register::RDX => self.gp64.rdx,
            iced_x86::Register::RSI => self.gp64.rsi,
            iced_x86::Register::RDI => self.gp64.rdi,
            iced_x86::Register::RBP => self.gp64.rbp,
            iced_x86::Register::RSP => self.gp64.rsp,
            iced_x86::Register::R8 => self.gp64.r8,
            iced_x86::Register::R9 => self.gp64.r9,
            iced_x86::Register::R10 => self.gp64.r10,
            iced_x86::Register::R11 => self.gp64.r11,
            iced_x86::Register::R12 => self.gp64.r12,
            iced_x86::Register::R13 => self.gp64.r13,
            iced_x86::Register::R14 => self.gp64.r14,
            iced_x86::Register::R15 => self.gp64.r15,
            _ => 0
        }
    }
}


#[derive(Debug, Clone, Copy, Default)]
pub struct GpRegister64 {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GpRegister32 {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub esi: u32,
    pub edi: u32,
    pub ebp: u32,
    pub esp: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GpRegister16 {
    pub ax: u16,
    pub bx: u16,
    pub cx: u16,
    pub dx: u16,
    pub si: u16,
    pub di: u16,
    pub bp: u16,
    pub sp: u16,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GpRegister8 {
    pub al: u8,
    pub ah: u8,
    pub bl: u8,
    pub bh: u8,
    pub cl: u8,
    pub ch: u8,
    pub dl: u8,
    pub dh: u8,
}


#[derive(Debug, Clone, Copy, Default)]
pub struct SegmentRegisters {
    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub ss: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct ControlRegisters {
    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub cr8: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct InstructionPointer {
    pub rip: u64,
}

impl InstructionPointer {
    pub fn increment(&mut self) {
        self.rip += 1;
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FlagsRegister {
    pub rflags: u64,
    pub eflags: u32,
    pub flags: u64,
}

impl FlagsRegister {
    pub fn is_carry(&self) -> bool {
        self.flags & 1 == 1
    }

    pub fn carry(&mut self) -> &mut Self {
        self.flags |= 1;
        self
    }

    pub fn no_carry(&mut self) -> &mut Self {
        self.flags &= !1;
        self
    }

    pub fn is_parity(&self) -> bool {
        self.flags & 4 == 4
    }

    pub fn parity(&mut self) -> &mut Self {
        self.flags |= 4;
        self
    }

    pub fn no_parity(&mut self) -> &mut Self {
        self.flags &= !4;
        self
    }

    pub fn is_adjust(&self) -> bool {
        self.flags & 16 == 16
    }

    pub fn adjust(&mut self) -> &mut Self {
        self.flags |= 16;
        self
    }

    pub fn is_zero(&self) -> bool {
        self.flags & 64 == 64
    }

    pub fn zero(&mut self) -> &mut Self {
        self.flags |= 64;
        self
    }

    pub fn is_sign(&self) -> bool {
        self.flags & 128 == 128
    }

    pub fn sign(&mut self) -> &mut Self {
        self.flags |= 128;
        self
    }

    pub fn is_trap(&self) -> bool {
        self.flags & 256 == 256
    }

    pub fn trap(&mut self) -> &mut Self {
        self.flags |= 256;
        self
    }

    pub fn is_interrupt(&self) -> bool {
        self.flags & 512 == 512
    }

    pub fn interrupt(&mut self) -> &mut Self {
        self.flags |= 512;
        self
    }

    pub fn is_direction(&self) -> bool {
        self.flags & 1024 == 1024
    }

    pub fn direction(&mut self) -> &mut Self {
        self.flags |= 1024;
        self
    }

    pub fn is_overflow(&self) -> bool {
        self.flags & 2048 == 2048
    }

    pub fn overflow(&mut self) -> &mut Self {
        self.flags |= 2048;
        self
    }

    pub fn is_iopl(&self) -> bool {
        self.flags & 12288 == 12288
    }

    pub fn iopl(&mut self) -> &mut Self {
        self.flags |= 12288;
        self
    }


}

