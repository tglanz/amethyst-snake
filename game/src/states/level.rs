use crate::{
    components::*,
    resources::RenderHandles,
};

use amethyst::{
    prelude::*,
    core::{ transform::Transform, },
    renderer::{ SpriteRender, Camera, Projection, },
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
    let floor_render = {
        let render_handles = world.read_resource::<RenderHandles>();
        SpriteRender {
            sprite_sheet: render_handles.spritesheet.clone().unwrap(),
            sprite_number: 5
        }
    };

    for row in 0..rows-1 {
        for col in 0..cols-1 {
            let mut transform = Transform::default();
            transform
                .translate_xyz(row as f32 * 32.0 + 14.0, col as f32 * 32.0 + 10.0, 0.0);
            world.create_entity()
                .with(Tile::new(TileType::Ground))
                .with(GridPosition::new(row, col))
                .with(transform)
                .with(floor_render.clone())
                .build();
        }
    }
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
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}