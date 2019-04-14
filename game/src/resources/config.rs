use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct GridConfig {
    pub offset: (f32, f32),
    pub tile_size: f32,
    pub tile_scale: f32,
    pub tile_margin: f32,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct SpriteConfig {
    pub snake_head_up: usize,
    pub snake_limb_up_up: usize,
    pub snake_limb_up_right: usize,
    pub snake_tail_down: usize,
    pub apple: usize,
    pub tile_ground: usize,
    pub tile_wall: usize
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct GameplayConfig {
    pub base_update_millis: u64,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct GameConfig {
    pub gameplay: GameplayConfig,
    pub grid: GridConfig,
    pub sprite: SpriteConfig,
}