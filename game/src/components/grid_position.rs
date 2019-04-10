use amethyst::{
    ecs::{
        Component, 
        DenseVecStorage
    },
};

pub struct GridPosition {
    pub row: usize,
    pub col: usize,
    pub layer: usize,
}

impl GridPosition {
    pub fn new(row: usize, col: usize, layer: usize) -> Self {
        GridPosition {
            row, col, layer
        }
    }
}

impl Component for GridPosition {
    type Storage = DenseVecStorage<Self>;
}