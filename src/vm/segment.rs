use lazy_static::lazy_static;

const GDT_SIZE: usize = 8;


#[derive(Debug, Default, Clone, Copy)]
pub struct SegmentRegister {
    selector: u16,
    pub(crate) base: u32,
    limit: u32,
    flags: u16,
}




lazy_static! {
    pub static ref CS: SegmentRegister = SegmentRegister {
        selector: 0x08,
        base: 0,
        limit: 0,
        flags: 0x9A
    };

    pub static ref DS: SegmentRegister = SegmentRegister {
        selector: 0x10,
        base: 0,
        limit: 0,
        flags: 0x92
    };

    pub static ref SS: SegmentRegister = SegmentRegister {
        selector: 0x18,
        base: 0,
        limit: 0,
        flags: 0x92
    };

    pub static ref ES: SegmentRegister = SegmentRegister {
        selector: 0x20,
        base: 0,
        limit: 0,
        flags: 0x92
    };

    pub static ref FS: SegmentRegister = SegmentRegister {
        selector: 0x28,
        base: 0,
        limit: 0,
        flags: 0x92
    };

    pub static ref GS: SegmentRegister = SegmentRegister {
        selector: 0x30,
        base: 0,
        limit: 0,
        flags: 0x92
    };

    pub static ref TR: SegmentRegister = SegmentRegister {
        selector: 0x38,
        base: 0,
        limit: 0,
        flags: 0x82
    };

    pub static ref LDTR: SegmentRegister = SegmentRegister {
        selector: 0x40,
        base: 0,
        limit: 0,
        flags: 0x82
    };


}

impl SegmentRegister {
    pub fn from_register(register: iced_x86::Register) -> &'static SegmentRegister {
        match register {
            iced_x86::Register::CS => &*CS,
            iced_x86::Register::DS => &*DS,
            iced_x86::Register::SS => &*SS,
            iced_x86::Register::ES => &*ES,
            iced_x86::Register::FS => &*FS,
            iced_x86::Register::GS => &*GS,
            _ => &*CS
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SegmentDescriptor {
    base: u32,
    limit: u32,
    flags: u16,
}

pub struct GDT {
    pub gdt: [SegmentDescriptor; GDT_SIZE]
}

impl GDT {
    pub fn new(gdt: [SegmentDescriptor; GDT_SIZE]) -> Self {
        Self {
            gdt
        }
    }

    pub fn load_segment(&mut self, selector: u16, seg: &mut SegmentRegister) {
        let index = (selector >> 3) as usize;

        if index < self.gdt.len() {
            let desc = self.gdt[index];
            seg.selector = selector;
            seg.base = desc.base;
            seg.limit = desc.limit;
            seg.flags = desc.flags;
        }
    }
}
