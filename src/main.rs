use std::fs::File;
use std::io::Write;

#[derive(Debug, Copy, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    fn to_u32(&self) -> u32 {
        ((self.a << 24) + (self.b << 16) + (self.g << 8) + self.r) as u32
    }

    fn from_u32(color: u32) -> Color {
        let r = ((color >> 0) & 255) as u8;
        let g = ((color >> 8) & 255) as u8;
        let b = ((color >> 16) & 255) as u8;
        let a = ((color >> 24) & 255) as u8;
        Color::new(r, g, b, a)
    }
}

fn write_ppm_image(filename: &str, image: &Vec<Color>, w: usize, h: usize) {
    assert_eq!(image.len(), w * h);
    let mut file = File::create(filename).expect("erro ao tentar criar o arquivo ppm");
    let ppm_header = format!("P6\n{} {}\n255\n", w, h);
    file.write_all(ppm_header.as_bytes())
        .expect("erro ao tentar escrever o header ppm");
    for i in 0..w * h {
        file.write(&[image[i].r, image[i].g, image[i].b])
            .expect("erro ao tentar escrever os bytes da imagem");
    }
}

fn main() {
    const WIDTH: usize = 512;
    const HEIGHT: usize = 512;

    let mut image_data = vec![Color::from_u32(0); WIDTH * HEIGHT];

    for j in 0..H {
        for i in 0..W {
            let r = (255.0 * j as f32 / H as f32) as u8;
            let g = (255.0 * i as f32 / W as f32) as u8;
            let b = 0u8;
            image_data[i + j * W] = Color::new(r, g, b, 255);
        }
    }

    write_ppm_image("framebuffer.ppm", &image_data, WIDTH, HEIGHT);
}
