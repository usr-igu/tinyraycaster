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
            let r = (255.0 * j as f32 / window.height as f32) as u8;
            let g = (255.0 * i as f32 / window.width as f32) as u8;
            let b = 0u8;
            window.set_pixel(i, j, Color::from_rgb(r, g, b))
        }
    }

    // Desenha o mapa
    let rect_h = window.height / MAP_H;
    let rect_w = window.width / MAP_W;

    for j in 0..MAP_H {
        for i in 0..MAP_W {
            if MAP[i + j * MAP_W] == b' ' {
                continue;
            }
            let rect_x = i * rect_w;
            let rect_y = j * rect_h;
            window.draw_rect(rect_x, rect_y, rect_w, rect_h, Color::from_rgb(0, 255, 255));
        }
    }

    let player = Player::new(3.456, 2.345, std::f32::consts::FRAC_PI_2);

    // Desenha o jogador
    window.set_pixel(
        (player.x * rect_w as f32) as usize,
        (player.y * rect_h as f32) as usize,
        Color::white(),
    );

    // Desenha a field of view do player
    let fov = std::f32::consts::FRAC_PI_3;
    for i in 0..window.width {
        let angle = player.dir - fov / 2.0 + fov * i as f32 / window.width as f32;
        let mut distance = 0.0;
        while distance < 10.0 {
            let cx = player.x + distance * angle.cos();
            let cy = player.y + distance * angle.sin();
            if MAP[cx.floor() as usize + cy.floor() as usize * MAP_W] != b' ' {
                break;
            }
            let pix_x = cx * rect_w as f32;
            let pix_y = cy * rect_h as f32;
            window.set_pixel(pix_x as usize, pix_y as usize, Color::white());
            distance += 0.05;
        }
    }

    window.write_ppm_image("framebuffer.ppm");
}
