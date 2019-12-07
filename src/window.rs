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
                assert!(cx < self.width && cy < self.height);
                self.set_pixel(cx, cy, color);
            }
        }
    }

    /// Grava o buffer de pixels em um arquivo ppm.
    pub fn write_ppm_image(&self, filename: &str) {
        let mut file = File::create(filename).expect("erro ao tentar criar o arquivo ppm");
        let ppm_header = format!("P6\n{} {}\n255\n", self.width, self.height);
        file.write_all(ppm_header.as_bytes())
            .expect("erro ao tentar escrever o header ppm");
        for i in 0..self.width * self.height {
            file.write_all(&[self.pixels[i].r(), self.pixels[i].g(), self.pixels[i].b()])
                .expect("erro ao tentar escrever os bytes da imagem");
        }
    }
}

impl Default for Window {
    fn default() -> Self {
        Window::new(512, 512, vec![Color::black(); 512 * 512])
    }
}
