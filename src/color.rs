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

    pub fn a(&self) -> u8 {
        return self.a;
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color::new(r, g, b, 255)
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
}
