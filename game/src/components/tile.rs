use amethyst::{
    ecs::{Component, DenseVecStorage},
};

#[derive(PartialEq, Eq)]
pub enum TileType {
    Wall,
    Ground,
}

pub struct Tile {
    pub tile_type: TileType
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Tile {
            tile_type
        }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}