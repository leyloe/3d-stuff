use bevy::math::Vec2;
use std::f32::consts::{FRAC_PI_2, PI};

pub const PLAYER_MOVEMENT_SPEED: f32 = 2.0;
pub const PLAYER_CAMERA_SENSITIVITY: Vec2 = Vec2::new(0.003, 0.002);
pub const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
pub const MAX_SLOPE_ANGLE: f32 = PI * 0.45;
pub const MOVEMENT_ACCELERATION: f32 = 30.0;
pub const JUMP_IMPULSE: f32 = 7.0;
pub const DAMPING: f32 = 0.9;
