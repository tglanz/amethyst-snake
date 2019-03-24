use amethyst::{
    ecs::{
        Component, 
        DenseVecStorage
    },
};

pub struct GridPosition {
    pub row: usize,
    pub col: usize,
}

impl GridPosition {
    pub fn new(row: usize, col: usize) -> Self {
        GridPosition {
            row, col
        }
    }
}

impl Component for GridPosition {
    type Storage = DenseVecStorage<Self>;
}