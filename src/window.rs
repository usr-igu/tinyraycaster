use crate::color::Color;
use std::fs::File;
use std::io::Write;

pub struct Window {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Window {
    pub fn new(width: usize, height: usize, pixels: Vec<Color>) -> Window {
        Window {
            width,
            height,
            pixels,
        }
    }

    /// Desenha um pixel com a cor `color` no ponto (x, y).
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[x + y * self.width] = color;
    }

    /// Desenha um retângulo preenchido com a cor `color` com origem no ponto (x, y).
    ///
    /// O ponto de origem representa o canto superior esquerdo do retângulo.
    pub fn draw_rect(
        &mut self,
        x: usize,
        y: usize,
        rect_width: usize,
        rect_height: usize,
        color: Color,
    ) {
        assert_eq!(self.pixels.len(), self.height * self.width);
        for i in 0..rect_width {
            for j in 0..rect_height {
                let cx = x + i;
                let cy = y + j;
                if cx >= self.width || cy >= self.height {
                    continue;
                };
                self.set_pixel(cx, cy, color);
            }
        }
    }

    /// Grava o buffer de pixels em um arquivo png.
    pub fn write_png_image(&self, filename: &str) {
        let mut file = File::create(filename).expect("erro ao tentar criar o arquivo png");
        let encoder = image::png::PNGEncoder::new(file);
        let mut bytes = Vec::with_capacity(self.width * self.height);

        for pixels in &self.pixels {
            bytes.push(pixels.r());
            bytes.push(pixels.g());
            bytes.push(pixels.b());
        }

        encoder
            .encode(
                bytes.by_ref(),
                self.width as u32,
                self.height as u32,
                image::ColorType::Rgb8,
            )
            .expect("erro ao criar arquivo png");
    }

    pub fn clear(&mut self, color: Color) {
        for pixel in &mut self.pixels {
            *pixel = color;
        }
    }
}

impl Default for Window {
    fn default() -> Self {
        Window::new(1024, 512, vec![Color::black(); 1024 * 512])
    }
}
