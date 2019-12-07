#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub dir: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, dir: f32) -> Player {
        Player { x, y, dir }
    }
}
