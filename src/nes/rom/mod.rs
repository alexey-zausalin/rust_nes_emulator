#[derive(Debug)]
pub struct Rom {
    data: Vec<u8>,
}

impl Rom {
    pub fn new(data: Vec<u8>) -> Rom {
        Rom { data }
    }

    pub fn read(&self, pos: u16) -> u8 {
        self.data[pos as usize]
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}
