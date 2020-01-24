use crate::color::Color;
use std::fs::File;
use std::io::Write;

pub struct FrameBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32, pixels: Vec<Color>) -> FrameBuffer {
        FrameBuffer {
            width,
            height,
            pixels,
        }
    }

    /// Desenha um pixel com a cor `color` no ponto (x, y).
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.pixels[(x + y * self.width) as usize] = color;
    }

    /// Desenha um retângulo preenchido com a cor `color` com origem no ponto (x, y).
    ///
    /// O ponto de origem (origin_x, origin_y) representa o canto superior esquerdo do retângulo.
    pub fn draw_rect(
        &mut self,
        origin_x: u32,
        origin_y: u32,
        width: u32,
        height: u32,
        color: Color,
    ) {
        assert_eq!(self.pixels.len(), (self.height * self.width) as usize);
        for i in 0..width {
            for j in 0..height {
                let pos_x = origin_x + i;
                let pos_y = origin_y + j;
                // Não desenha fora janela
                if pos_x >= self.width || pos_y >= self.height {
                    continue;
                };
                self.set_pixel(pos_x, pos_y, color);
            }
        }
    }

    /// Grava o buffer de pixels em um arquivo png.
    pub fn write_png_image(&self, filename: &str) {
        let file = File::create(filename).expect("erro ao tentar criar o arquivo png");
        let encoder = image::png::PNGEncoder::new(file);
        let mut bytes = Vec::with_capacity((self.width * self.height) as usize);
        for pixel in &self.pixels {
            bytes.extend(&[pixel.r(), pixel.g(), pixel.b(), pixel.a()]);
        }
        encoder
            .encode(
                bytes.by_ref(),
                self.width as u32,
                self.height as u32,
                image::ColorType::RGBA(8),
            )
            .expect("erro ao criar arquivo png");
    }

    pub fn clear(&mut self, color: Color) {
        for pixel in &mut self.pixels {
            *pixel = color;
        }
    }
}

impl Default for FrameBuffer {
    fn default() -> Self {
        FrameBuffer::new(1024, 512, vec![Color::black(); 1024 * 512])
    }
}
