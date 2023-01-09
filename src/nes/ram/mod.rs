pub struct Ram {
    data: Vec<u8>,
}

impl Ram {
    pub fn new(data: Vec<u8>) -> Ram {
        Ram { data }
    }

    pub fn read(&self, pos: u16) -> u8 {
        self.data[pos as usize]
    }
    pub fn write(&mut self, pos: u16, value: u8) {
        self.data[pos as usize] = value;
    }
}
