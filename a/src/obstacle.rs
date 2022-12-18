use glam::{IVec2, UVec2};

use crate::graphics::Graphics;

pub struct Obstacle {
    pos: IVec2,
    size: UVec2,
}

impl Obstacle {
    pub fn new(pos: IVec2, size: UVec2) -> Obstacle {
        Obstacle { pos, size }
    }

    pub fn render(graphics: &mut Graphics) {}
}
