use std::time::Duration;

use bevy::prelude::*;

pub const CLICK_COLOR: Color = Color::WHITE;
pub const CLICK_COLOR_CRIT: Color = Color::rgb(0.3, 0.8, 0.8);
pub const CLEAR_COLOR: Color = Color::rgb(0.6, 0.6, 0.8);

pub const CRIT_SIZE_MULTIPLIER: f32 = 1.5;

pub const CLICK_TEXT_SPEED: f32 = 3.0;
pub const CLICK_TEXT_DURATION: Duration = Duration::from_secs(1);
