mod player;
use player::Player;
mod color;
use color::Color;
mod window;
use window::FrameBuffer;

struct Map {
    pub width: u32,
    pub height: u32,
    data: Vec<u8>,
}

impl Map {
    fn new(width: u32, height: u32, data: Vec<u8>) -> Self {
        assert_eq!(data.len(), (width * height) as usize);
        Self {width, height, data }
    }

    fn len(&self) -> u32 {
        self.data.len() as u32
    }

    fn at(&self, x: u32, y: u32) -> u8 {
        self.data[(x + y * self.width) as usize]
    }

    fn is_empty(&self, x: u32, y: u32) -> bool {
        self.data[(x + y * self.width) as usize] == b' '
    }
}

fn main() {
    let mut window = FrameBuffer::default();

    let colors = vec![
        Color::from_rgb(15, 76, 129),
        Color::from_rgb(141, 0, 69),
        Color::from_rgb(214, 165, 0),
        Color::from_rgb(0, 141, 155),
        Color::from_rgb(221, 167, 155),
    ];

    let map = Map::new(
        16,
        16,
        b"0000222222220000\
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
                        0002222222200000"
            .to_vec(),
    );

    let mut player = Player::new(3.456, 2.345, std::f32::consts::FRAC_PI_4);

    const FRAMES: usize = 5;

    for frame in 0..FRAMES {
        // Desenha o fundo
        window.clear(Color::white());

        // Desenha o mapa
        let rect_w = window.width / (map.width * 2);
        let rect_h = window.height / map.height;

        for j in 0..map.height {
            for i in 0..map.width {
                if map.is_empty(i, j) {
                    continue;
                }
                let rect_x = i * rect_w;
                let rect_y = j * rect_h;
                let color = map.at(i,j) - b'0';
                window.draw_rect(rect_x, rect_y, rect_w, rect_h, colors[color as usize]);
            }
        }

        let fov = std::f32::consts::FRAC_PI_3;

        for i in 0..window.width / 2 {
            let angle = player.angle - (fov / 2.0) + (fov * i as f32) / (window.width as f32 / 2.0);
            let mut distance = 0.0;
            while distance < 20.0 {
                let cx = player.x + distance * angle.cos();
                let cy = player.y + distance * angle.sin();

                let pix_x = (cx * rect_w as f32) as u32;
                let pix_y = (cy * rect_h as f32) as u32;

                // Desenha cone FOV
                window.set_pixel(pix_x, pix_y, Color::from_rgb(160, 160, 160));

                if !map.is_empty(cx as u32, cy as u32) {
                    let column_height =
                        (window.height as f32 / (distance * (angle - player.angle).cos())) as u32;
                    let column_x = window.width / 2 + i;
                    let column_y = window.height / 2 - column_height / 2;
                    let color = map.at(cx as u32, cy as u32) - b'0';
                    // Desenha o céu
                    window.draw_rect(
                        column_x,
                        0,
                        1,
                        window.height - column_height,
                        Color::from_rgb(0, 238, 238),
                    );
                    // Desenha a visão pseudo-3D
                    window.draw_rect(column_x, column_y, 1, column_height, colors[color as usize]);
                    // Desenha o chão
                    window.draw_rect(
                        column_x,
                        column_y + column_height,
                        1,
                        window.height - column_height,
                        Color::from_rgb(120, 120, 120),
                    );
                    break;
                }
                distance += 0.01;
            }
        }
        player.angle += 2.0 * std::f32::consts::PI / 360.0;
        // window.write_png_image(&format!("output/framebuffer_{}.png", frame));
    }
}
