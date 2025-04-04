use std::io::Write;
use std::rc::Rc;
use iced_x86::{Code, Instruction, Mnemonic};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};
use crate::ast::Bits;
use crate::vm::mem::{Memory, HUNDRED_MO};
use crate::vm::{Mode};
use crate::vm::register::{FlagsRegister, GeneralPurposeRegisters, InstructionPointer};
use crate::vm::segment::SegmentRegister;
use crate::vm::virtualdisk::VirtualDisk;

pub struct Cpu<'ttf> {
    mode: Mode,
    mem: Memory,
    gpr: GeneralPurposeRegisters,
    ip: InstructionPointer,
    pub disk: VirtualDisk,
    flags: FlagsRegister,
    sdl_context: sdl2::Sdl,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    font: Font<'ttf, 'static>,
    ttf_context: &'ttf Sdl2TtfContext,
}
impl<'ttf> Cpu<'ttf> {
    pub fn with_mode(mode: Mode, sdl2ttf_context: &'ttf Sdl2TtfContext) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("XVm", 640, 400)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let font = sdl2ttf_context.load_font("/Users/antoine/Library/Fonts/0xProtoNerdFont-Regular.ttf", 16).unwrap();

        Self {
            mode,
            mem: Memory::new(HUNDRED_MO),
            gpr: GeneralPurposeRegisters::default(),
            ip: InstructionPointer::default(),
            disk: VirtualDisk::new("cpu.vdisk"),
            flags: FlagsRegister::default(),
            sdl_context,
            canvas,
            texture_creator,
            font,
            ttf_context: sdl2ttf_context,
        }
    }

    pub fn new(sdl2ttf_context: &'ttf Sdl2TtfContext) -> Self {
        Self::with_mode(Mode::Real, sdl2ttf_context)
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn gpr(&self) -> &GeneralPurposeRegisters {
        &self.gpr
    }

    pub fn gpr_mut(&mut self) -> &mut GeneralPurposeRegisters {
        &mut self.gpr
    }

    pub fn is_real(&self) -> bool {
        self.mode == Mode::Real
    }

    pub fn is_protected(&self) -> bool {
        self.mode == Mode::Protected
    }

    pub fn get_bit(&self) -> Bits {
        match self.mode {
            Mode::Real => Bits::Bit16,
            Mode::Protected => Bits::Bit32,
            Mode::Long => Bits::Bit64
        }
    }

    fn segmentation_to_physical(&self, seg: &iced_x86::Register, offset: u32) -> u32 {
        assert_ne!(self.mode, Mode::Long);
        let base = self.gpr.get_register_value(*seg) as u32;
        if self.is_real() {
            base * 16 + offset
        } else {
            base + offset
        }

    }

    pub fn vga_render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();


        let vga = self.vga_read(80 * 25);
        for (i, &c) in vga.iter().enumerate() {
            if c as u8 == 0 {
                continue;
            }
            let x = (i % 80) as i32 * 8;
            let y = (i / 80) as i32 * 16;
            let surface = self.font.render_char(c).blended(Color::RGB(255, 255, 255)).unwrap();
            let texture = self.texture_creator.create_texture_from_surface(&surface).unwrap();
            let target = Rect::new(x, y, 8, 16);
            self.canvas.copy(&texture, None, target).unwrap();
        }
        self.canvas.present();
    }

    pub fn run_instr(&mut self, instr: Instruction) {
        match instr.mnemonic() {
            Mnemonic::Int => {
                let int = instr.immediate8();
                if !self.flags.is_interrupt() {
                    return;
                }
                self.handle_interrupt(int);
            },
            Mnemonic::Lgdt => {
                let addr = self.get_op0addr(instr).expect("gdt expected") as usize;
                let limit = self.mem.read_u16(addr);
                let base = self.mem.read_u32(addr + 2);


                println!("limit: {}, base: {}", limit, base);
            },
            Mnemonic::Cli => {
                self.flags.no_interrupt();
            },
            Mnemonic::Cmp => {
                let op1 = self.get_op1value(instr);
                let op0 = self.get_op0value(instr);
                if op0 == op1 {
                    self.flags.zero();
                } else if op0 < op1 {
                    self.flags.carry();
                } else {
                    self.flags.no_carry();
                }
            }
            Mnemonic::Hlt => {
                loop {}
            },
            Mnemonic::Jmp => {
                let op0 = self.get_op0value(instr);
                self.ip.rip = op0 as u64;
                println!("Jump to {}", op0);
            },
            Mnemonic::Je => {
                if self.flags.is_zero() {
                    let op0 = self.get_op0value(instr);
                    self.ip.rip = op0 as u64;
                }
            },
            Mnemonic::Jne => {
                if !self.flags.is_zero() {
                    let op0 = self.get_op0value(instr);
                    self.ip.rip = op0 as u64;
                }
            },
            Mnemonic::Mov => {
                let op1 = self.get_op1value(instr);
                self.write_op0(instr, op1);
            },
            Mnemonic::Add => {
                let op1 = self.get_op1value(instr);
                let op0 = self.get_op0value(instr);
                self.write_op0(instr, op0 + op1);
            },
            Mnemonic::Sub => {
                let op1 = self.get_op1value(instr);
                let op0 = self.get_op0value(instr);
                self.write_op0(instr, op0 - op1);
            },
            Mnemonic::And => {
                let op1 = self.get_op1value(instr);
                let op0 = self.get_op0value(instr);
                self.write_op0(instr, op0 & op1);
            },
            Mnemonic::Or => {
                let op1 = self.get_op1value(instr);
                let op0 = self.get_op0value(instr);
                self.write_op0(instr, op0 | op1);
            },
            e => {
                println!("unhandled instruction: {:?}", e);
            }
        }
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        loop {

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        return;
                    }
                    _ => {}
                }
            }
            let ip = self.ip.rip;
            let bytes = self.mem.read_many_u8(ip as usize, 15);
            let mut decoder = iced_x86::Decoder::new(self.get_bit().into(), &bytes, iced_x86::DecoderOptions::NONE);
            let instr = decoder.decode();
            println!("{}", instr);
            self.ip.rip += instr.len() as u64;
            self.run_instr(instr);
            self.vga_render();
            println!();
        }
    }

    pub fn get_op0addr(&mut self, instruction: Instruction) -> Option<u64> {
        if instruction.op0_kind() != iced_x86::OpKind::Memory {
            None
        } else {
            let base_register = instruction.memory_base();
            let displacement = if self.mode == Mode::Real || self.mode == Mode::Protected {
                instruction.memory_displacement32() as u64
            } else {
                instruction.memory_displacement64()
            };

            if let iced_x86::Register::None = base_register {
                Some(displacement)
            } else {
                let base_value = self.gpr.get_register_value(base_register);
                Some(base_value + displacement)
            }
        }
    }

    pub fn get_op0value(&mut self, instruction: Instruction) -> usize {
        let op0 = instruction.op0_kind();
        match op0 {
            iced_x86::OpKind::Register => {
                let reg = instruction.op1_register();
                self.gpr.get_register_value(reg) as usize
            },
            iced_x86::OpKind::Memory => {
                let offset = if self.get_bit() == Bits::Bit64 {
                    instruction.memory_displacement64()
                } else {
                    instruction.memory_displacement32() as u64
                };
                let physical = self.segmentation_to_physical(&instruction.segment_prefix(), offset as u32);
                self.mem.read_u32(physical as usize) as usize
            },
            _ => 0
        }
    }

    pub fn get_op1value(&mut self, instruction: Instruction) -> usize {
        let op1 = instruction.op1_kind();
        match op1 {
            iced_x86::OpKind::Register => {
                let reg = instruction.op1_register();
                self.gpr.get_register_value(reg) as usize
            },
            iced_x86::OpKind::Memory => {
                let offset = if self.get_bit() == Bits::Bit64 {
                    instruction.memory_displacement64()
                } else {
                    instruction.memory_displacement32() as u64
                };
                let physical = self.segmentation_to_physical(&instruction.segment_prefix(), offset as u32);
                self.mem.read_u32(physical as usize) as usize
            },
            iced_x86::OpKind::Immediate8 => instruction.immediate8() as usize,
            iced_x86::OpKind::Immediate16 => instruction.immediate16() as usize,
            iced_x86::OpKind::Immediate32 => instruction.immediate32() as usize,
            iced_x86::OpKind::Immediate64 => instruction.immediate64() as usize,
            _ => 0
        }
    }

    pub fn write_op0(&mut self, instruction: Instruction, val: usize) {
        let op0 = instruction.op0_kind();
        match op0 {
            iced_x86::OpKind::Register => {
                let reg = instruction.op0_register();
                self.gpr.set_register_value(reg, val);
            },
            iced_x86::OpKind::Memory => {
                if self.mode != Mode::Long {
                    let offset = if self.get_bit() == Bits::Bit64 {
                        instruction.memory_displacement64()
                    } else {
                        instruction.memory_displacement32() as u64
                    };
                    let physical = self.segmentation_to_physical(&instruction.segment_prefix(), offset as u32);
                    self.write_to_mem(physical as usize, val);
                }
            },
            _ => {}
        }
    }

    pub fn write_to_mem(&mut self, addr: usize, value: usize) {
        match self.get_bit() {
            Bits::Bit8 => self.mem.write_u8(addr, value as u8),
            Bits::Bit16 => self.mem.write_u16(addr, value as u16),
            Bits::Bit32 => self.mem.write_u32(addr, value as u32),
            Bits::Bit64 => self.mem.write_u64(addr, value as u64),

        }
    }

    pub fn vga_read(&self, size: usize) -> Vec<char> {
        let mut vga = Vec::new();
        for i in 0..size {
            let c = self.mem.read_u8(0xb8000 + i);
            vga.push(c as char);
        }
        vga
    }
    pub fn vga_read_chars(&self) -> Vec<char> {
        self.vga_read(80 * 25)
    }
    pub fn console(&self) {
        let vga = self.vga_read_chars();

        for i in 0..vga.len() {
            if (vga[i] as u8) == 0 {
                continue;
            }
            print!("{}", vga[i]);
            std::io::stdout().flush().unwrap();
            if (i + 1) % 80 == 0 {
                println!();
            }
        }

    }

    pub fn handle_interrupt(&mut self, int: u8) {
        match int {
            0x10 => {
                let ah = self.gpr.get_register_value(iced_x86::Register::AH);
                match ah {
                    0x0e => {
                        let mut i = 0;
                        loop {
                            let m = self.mem.read_u16(0xb8000 + i);
                            if m == 0 {
                                break;
                            }
                            i += 1;
                        }
                        let al = self.gpr.get_register_value(iced_x86::Register::AL);
                        self.write_to_mem(0xb8000 + i, al as usize);
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    pub fn init_bios(&mut self) {
        self.gpr.set_register_value(iced_x86::Register::CS, 0xF000);
        self.gpr.set_register_value(iced_x86::Register::DS, 0x0);
        self.gpr.set_register_value(iced_x86::Register::SS, 0x0);
        self.gpr.set_register_value(iced_x86::Register::SP, 0x7C00);

        self.ip.rip = 0xFFF0;

        let bootloader = self.disk.read_sector(0);

        for i in 0..bootloader.len() {
            self.mem.write_u8(0xFFF0 + i, bootloader[i]);
        }
    }



}