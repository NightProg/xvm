use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

const SECTOR_SIZE: usize = 512;
const DISK_SIZE: usize = 100 * 1024 * 1024;


pub struct VirtualDisk {
    file: File
}

impl VirtualDisk {
    pub fn new(filename: &str) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .unwrap();
        file.set_len(DISK_SIZE as u64).unwrap();
        Self { file }
    }

    pub fn read_sector(&mut self, sector: usize) -> Vec<u8> {
        let mut buffer = vec![0; SECTOR_SIZE];
        self.file.seek(SeekFrom::Start((sector * SECTOR_SIZE) as u64)).unwrap();
        self.file.read_exact(&mut buffer).unwrap();
        buffer
    }

    pub fn write_sector(&mut self, sector: usize, data: Vec<u8>) {
        self.file.seek(SeekFrom::Start((sector * SECTOR_SIZE) as u64)).unwrap();
        self.file.write_all(&data).unwrap();
    }
}