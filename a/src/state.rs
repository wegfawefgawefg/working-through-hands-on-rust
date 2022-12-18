use crate::{obstacle::Obstacle, player::Player};

pub enum Mode {
    Title,
    Playing,
    GameOver,
}

pub struct State {
    pub mode: Mode,

    pub running: bool,
    pub now: f64,
    pub time_since_last_update: f32,

    pub play_time: f32,
    pub score: u32,

    pub pause: bool,
    pub win: bool,

    pub player: Player,
    pub obstacle: Obstacle,
}

impl State {
    pub fn new() -> State {
        State {
            mode: Mode::Title,

            running: true,
            now: 0.0,
            time_since_last_update: 0.0,

            play_time: 0.0,
            score: 0,

            pause: false,
            win: false,

            player: Player::new(glam::IVec2 { x: 0, y: 0 }),
            obstacle: Obstacle::new(glam::IVec2 { x: 64, y: 64 }, glam::UVec2 { x: 8, y: 8 }),
        }
    }
}
