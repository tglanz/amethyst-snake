use crate::{
    types::*,
    components::*,
};

use amethyst::{
    prelude::*,
    core::{ transform::Transform, },
    renderer::{ Camera, Projection, },
};

pub struct LevelState {
    rows: usize,
    cols: usize,
}

impl LevelState {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }
}

fn initialize_grid(world: &mut World, rows: usize, cols: usize) {
    for row in 0..rows {
        for col in 0..cols {
            world.create_entity()
                .with(GridPosition::new(row, col, 0))
                .with(Tile::new(
                    if row * col == 0 || row == rows - 1 || col == cols - 1 {
                        TileType::Wall
                    } else {
                        TileType::Ground
                    }
                ))
                .build();
        }
    }
}    

fn initialize_snake(world: &mut World, rows: usize, cols: usize) {
    world.create_entity()
        .with(GridPosition::new(rows / 2, cols / 2 + 1, 1))
        .with(SnakeHead)
        .with(SnakeLimb {
            directions: (Direction::Right, Direction::Right),
            ttl: 3
        })
        .build();
    
    world.create_entity()
        .with(GridPosition::new(rows / 2, cols / 2, 1))
        .with(SnakeLimb {
            directions: (Direction::Right, Direction::Right),
            ttl: 2
        })
        .build();

    world.create_entity()
        .with(GridPosition::new(rows / 2, cols / 2 - 1, 1))
        .with(SnakeLimb {
            directions: (Direction::Right, Direction::Right),
            ttl: 1
        })
        .build();
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(100.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0, 1024.0,
            0.0, 768.0,
        )))
        .with(transform)
        .build();
}

impl SimpleState for LevelState {
    fn on_start(&mut self, data: StateData<GameData<'_, '_>>) {
        let world = data.world;
        initialize_camera(world);
        initialize_grid(world, self.rows, self.cols);
        initialize_snake(world, self.rows, self.cols);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}