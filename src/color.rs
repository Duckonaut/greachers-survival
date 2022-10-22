#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color { 
        Color { r, g, b, a  }
    }
}

impl From<Color> for Vec<u8> {
    fn from(val: Color) -> Self {
        vec![val.r, val.g, val.b, val.a]
    }
}
