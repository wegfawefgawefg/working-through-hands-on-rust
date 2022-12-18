use glam::*;
use raylib::prelude::*;

pub const TIMESTEP: f32 = 1.0 / 60.0;
pub const GRAVITY: f32 = 1.0;

pub const SPACE_RADIUS: i32 = 400;
pub const CEILING_POS: i32 = -SPACE_RADIUS;
pub const FLOOR_POS: i32 = SPACE_RADIUS;

use crate::{
    player::Player,
    state::{Mode, State},
};

pub fn step(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {
    let dt = rl.get_frame_time();
    state.time_since_last_update += dt;
    while state.time_since_last_update > TIMESTEP {
        match state.mode {
            Mode::Title => step_title(rl, rlt, state),
            Mode::Playing => step_playing(rl, rlt, state),
            Mode::GameOver => step_game_over(rl, rlt, state),
        }
        state.time_since_last_update -= TIMESTEP;
    }
}

pub fn step_title(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {}

pub fn step_playing(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {
    let player = &mut state.player;

    player.vel.y += GRAVITY;
    player.pos += player.vel.as_ivec2();
    player.vel.x += 0.01;

    if (player.pos.y + Player::SIZE.y as i32) > FLOOR_POS as i32 || player.pos.y < CEILING_POS {
        state.mode = Mode::GameOver;
    }
}
pub fn step_game_over(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {}

pub fn reset(state: &mut State) {
    state.player.pos = IVec2 { x: 0, y: 0 };
    state.player.vel = Vec2 {
        x: Player::STARTING_SPEED,
        y: 0.0,
    };
}
