use std::fs::File;
use std::io::Write;

#[derive(Debug, Copy, Clone)]
struct Player {
    x: f32,
    y: f32,
    dir: f32,
}

impl Player {
    fn new(x: f32, y: f32, dir: f32) -> Player {
        Player { x, y, dir }
    }
}

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

    fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
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

fn write_ppm_image(filename: &str, pixel_data: &Vec<Color>, w: usize, h: usize) {
    assert_eq!(pixel_data.len(), w * h);
    let mut file = File::create(filename).expect("erro ao tentar criar o arquivo ppm");
    let ppm_header = format!("P6\n{} {}\n255\n", w, h);
    file.write_all(ppm_header.as_bytes())
        .expect("erro ao tentar escrever o header ppm");
    for i in 0..w * h {
        file.write(&[pixel_data[i].r, pixel_data[i].g, pixel_data[i].b])
            .expect("erro ao tentar escrever os bytes da imagem");
    }
}

fn draw_rect(
    image_data: &mut Vec<Color>,
    image_w: usize,
    image_h: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    c: Color,
) {
    assert_eq!(image_data.len(), image_h * image_w);
    for i in 0..w {
        for j in 0..h {
            let cx = x + i;
            let cy = y + j;
            assert!(cx < image_w && cy < image_h);
            image_data[cx + cy * image_w] = c;
        }
    }
}

fn main() {
    const WIDTH: usize = 512;
    const HEIGHT: usize = 512;

    let mut pixel_data = vec![Color::from_u32(0); WIDTH * HEIGHT];

    const MAP_W: usize = 16;
    const MAP_H: usize = 16;

    const MAP: &[u8] = b"0000222222220000\
                        1              0\
                        1      11111   0\
                        1     0        0\
                        0     0  1110000\
                        0     3        0\
                        0   10000      0\
                        0   0   11100  0\
                        0   0   0      0\
                        0   0   1  00000\
                        0       1      0\
                        2       1      0\
                        0       0      0\
                        0 0000000      0\
                        0              0\
                        0002222222200000";

    assert_eq!(MAP.len(), MAP_W * MAP_H);

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let r = (255.0 * j as f32 / HEIGHT as f32) as u8;
            let g = (255.0 * i as f32 / WIDTH as f32) as u8;
            let b = 0u8;
            pixel_data[i + j * WIDTH] = Color::rgb(r, g, b);
        }
    }

    let rect_h = HEIGHT / MAP_H;
    let rect_w = WIDTH / MAP_W;

    for j in 0..MAP_H {
        for i in 0..MAP_W {
            if MAP[i + j * MAP_W] == b' ' {
                continue;
            }
            let rect_x = i * rect_w;
            let rect_y = j * rect_h;
            draw_rect(
                &mut pixel_data,
                WIDTH,
                HEIGHT,
                rect_x,
                rect_y,
                rect_w,
                rect_h,
                Color::rgb(0, 255, 255),
            );
        }
    }

    let player = Player::new(3.456, 2.345, 1.523);

    draw_rect(
        &mut pixel_data,
        WIDTH,
        HEIGHT,
        (player.x * rect_w as f32) as usize,
        (player.y * rect_h as f32) as usize,
        5,
        5,
        Color::rgb(255, 255, 255),
    );

    let mut t = 0.0;
    while t < 20.0 {
        let cx = player.x + t * player.dir.cos();
        let cy = player.y + t * player.dir.sin();
        if (MAP[cx.floor() as usize + cy.floor() as usize * MAP_W] != b' ') {
            break;
        }
        let pix_x = cx * rect_w as f32;
        let pix_y = cy * rect_h as f32;
        pixel_data[pix_x as usize + pix_y as usize * WIDTH] = Color::rgb(255, 255, 255);
        t = t + 0.05;
    }

    write_ppm_image("framebuffer.ppm", &pixel_data, WIDTH, HEIGHT);
}
