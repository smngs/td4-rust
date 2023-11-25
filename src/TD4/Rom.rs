
pub struct Rom {
    pub mem_array: Vec<u8>
}

impl Rom {
    pub fn new(mem_array: Vec<u8>) -> Self {
        Rom { mem_array }
    }
}

