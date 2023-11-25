use core::fmt;

#[derive(Debug)]
pub struct Port {
    pub input: u8,
    pub output: u8
}

impl Port {
    pub fn new() -> Self {
        Port { input: 0, output: 0 }
    }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Port {{ input: {:04b}, output: {:04b} }}", self.input, self.output)
    }
}

