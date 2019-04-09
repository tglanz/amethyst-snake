use super::Loadable;

use serde::{Deserialize, Serialize};

use amethyst::{
    config::Config
};

#[derive(Debug, Deserialize, Serialize)]
pub struct GridConfig {
    pub offset: (f32, f32),
    pub tile_size: f32,
    pub tile_scale: f32,
    pub tile_margin: f32,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            offset: (0.0, 0.0),
            tile_size: 0.0,
            tile_scale: 0.0,
            tile_margin: 0.0
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameConfig {
    pub grid: GridConfig
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            grid: GridConfig::default()
        }
    }
}