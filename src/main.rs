mod player;
use player::Player;
mod color;
use color::Color;
mod window;
use window::Window;

fn main() {
    let mut window = Window::default();

    let colors = vec![
        Color::from_rgb(207, 180, 236),
        Color::from_rgb(37, 0, 86),
        Color::from_rgb(49, 71, 118),
        Color::from_rgb(0, 10, 60),
        Color::from_rgb(178, 191, 197),
    ];

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

    let mut player = Player::new(3.456, 2.345, std::f32::consts::FRAC_PI_4);

    for frame in 0..3 {
        // Desenha o fundo
        for j in 0..window.height {
            for i in 0..window.width {
                window.set_pixel(i, j, Color::white())
            }
        }

        // Desenha o mapa
        let rect_w = window.width / (MAP_W * 2);
        let rect_h = window.height / MAP_H;

        for j in 0..MAP_H {
            for i in 0..MAP_W {
                if MAP[i + j * MAP_W] == b' ' {
                    continue;
                }
                let rect_x = i * rect_w;
                let rect_y = j * rect_h;
                let color = MAP[i + j * MAP_W] - b'0';
                window.draw_rect(rect_x, rect_y, rect_w, rect_h, colors[color as usize]);
            }
        }

        let fov = std::f32::consts::FRAC_PI_3;

        for i in 0..window.width / 2 {
            let angle = player.dir - (fov / 2.0) + (fov * i as f32) / (window.width as f32 / 2.0);
            let mut distance = 0.0;
            while distance < 20.0 {
                let cx = player.x + distance * angle.cos();
                let cy = player.y + distance * angle.sin();

                let pix_x = (cx * rect_w as f32) as usize;
                let pix_y = (cy * rect_h as f32) as usize;

                // Desenha cone FOV
                window.set_pixel(pix_x, pix_y, Color::from_rgb(160, 160, 160));

                if MAP[cx as usize + cy as usize * MAP_W] != b' ' {
                    let column_height = (window.height as f32 / distance) as usize;
                    let column_x = window.width / 2 + i;
                    let column_y = window.height / 2 - column_height / 2;
                    let color = MAP[cx as usize + cy as usize * MAP_W] - b'0';
                    // Desenha a visão pseudo-3D
                    window.draw_rect(column_x, column_y, 1, column_height, colors[color as usize]);
                    break;
                }
                distance += 0.10;
            }
        }
        player.dir += 2.0 * std::f32::consts::PI / 360.0;
        window.write_png_image(&format!("framebuffer_{}.png", frame));
    }
}
