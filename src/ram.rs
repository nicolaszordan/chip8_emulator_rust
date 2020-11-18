pub struct RAM {
    memory: [u8; 4096],
}

impl Default for RAM {
    fn default() -> RAM {
        RAM::new()
    }
}

impl RAM {
    pub fn new() -> RAM {
        RAM { memory: [0; 4096] }
    }

    pub fn reset(&mut self) {
        self.memory.iter_mut().for_each(|m| *m = 0)
    }

    pub fn write(&mut self, addr: u16, buffer: &[u8]) {
        self.memory[addr as usize..addr as usize + buffer.len()].copy_from_slice(buffer)
    }

    pub fn read(&self, addr: u16, buffer: &mut [u8]) {
        buffer.copy_from_slice(&self.memory[addr as usize..addr as usize + buffer.len()])
    }

    pub fn dump(&self) {
        println!("{:0X?}", self.memory);
    }

    //fn dump_at(&self, addr: u16, length: usize)
    //{
    //    println!("{:0X?}", self.memory[addr as usize..addr as usize + length]);
    //}

    pub fn write8(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }

    pub fn write16(&mut self, addr: u16, val: u16) {
        self.write8(addr, (val >> 8) as u8);
        self.write8(addr + 1, val as u8);
    }

    pub fn read8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn read16(&self, addr: u16) -> u16 {
        ((self.memory[addr as usize] as u16) << 8) | (self.memory[(addr + 1) as usize] as u16)
    }
}
