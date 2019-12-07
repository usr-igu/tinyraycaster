mod player;
use player::Player;
mod color;
use color::Color;
mod window;
use window::Window;

fn main() {
    let mut window = Window::default();

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
            window.draw_rect(
                rect_x,
                rect_y,
                rect_w,
                rect_h,
                Color::from_rgb(50, 200, 200),
            );
        }
    }

    let player = Player::new(3.456, 2.345, std::f32::consts::FRAC_PI_2);

    // Desenha o jogador
    window.set_pixel(
        (player.x * rect_w as f32) as usize,
        (player.y * rect_h as f32) as usize,
        Color::red(),
    );

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
                // Desenha a visão pseudo-3D
                window.draw_rect(
                    column_x,
                    column_y,
                    1,
                    column_height,
                    Color::from_rgb(50, 200, 200),
                );
                break;
            }
            distance += 0.05;
        }
    }

    window.write_ppm_image("framebuffer.ppm");
}
