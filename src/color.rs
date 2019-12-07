#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn r(&self) -> u8 {
        return self.r;
    }

    pub fn g(&self) -> u8 {
        return self.g;
    }

    pub fn b(&self) -> u8 {
        return self.b;
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    pub fn red() -> Color {
        Color::from_rgb(255, 0, 0)
    }

    pub fn green() -> Color {
        Color::from_rgb(0, 255, 0)
    }

    pub fn blue() -> Color {
        Color::from_rgb(0, 0, 255)
    }

    pub fn white() -> Color {
        Color::from_rgb(255, 255, 255)
    }

    pub fn black() -> Color {
        Color::from_rgb(0, 0, 0)
    }

    pub fn to_u32(&self) -> u32 {
        ((self.a << 24) + (self.b << 16) + (self.g << 8) + self.r) as u32
    }

    pub fn from_u32(color: u32) -> Color {
        let r = ((color >> 0) & 255) as u8;
        let g = ((color >> 8) & 255) as u8;
        let b = ((color >> 16) & 255) as u8;
        let a = ((color >> 24) & 255) as u8;
        Color::new(r, g, b, a)
    }
}
