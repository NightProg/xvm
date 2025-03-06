pub const HUNDRED_MO: usize = 104_857_600;

pub struct Memory {
    data: Vec<u8>,
}


impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn read_many_u8(&self, addr: usize, size: usize) -> Vec<u8> {
        self.data[addr..addr + size].to_vec()
    }
    pub fn read_u8(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn read_u16(&self, addr: usize) -> u16 {
        u16::from_le_bytes([self.data[addr], self.data[addr + 1]])
    }

    pub fn read_u32(&self, addr: usize) -> u32 {
        u32::from_le_bytes([
            self.data[addr],
            self.data[addr + 1],
            self.data[addr + 2],
            self.data[addr + 3],
        ])
    }

    pub fn write_u8(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
    }

    pub fn write_u16(&mut self, addr: usize, value: u16) {
        let bytes = value.to_le_bytes();
        self.data[addr] = bytes[0];
        self.data[addr + 1] = bytes[1];
    }

    pub fn write_u32(&mut self, addr: usize, value: u32) {
        let bytes = value.to_le_bytes();
        self.data[addr] = bytes[0];
        self.data[addr + 1] = bytes[1];
        self.data[addr + 2] = bytes[2];
        self.data[addr + 3] = bytes[3];
    }

    pub fn write_u64(&mut self, addr: usize, value: u64) {
        let bytes = value.to_le_bytes();
        self.data[addr] = bytes[0];
        self.data[addr + 1] = bytes[1];
        self.data[addr + 2] = bytes[2];
        self.data[addr + 3] = bytes[3];
        self.data[addr + 4] = bytes[4];
        self.data[addr + 5] = bytes[5];
        self.data[addr + 6] = bytes[6];
        self.data[addr + 7] = bytes[7];
    }
}
