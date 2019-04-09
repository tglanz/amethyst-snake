use crate::{
    components::{GridPosition},
    resources::GridConfig,
};

use log;

use amethyst::{
    core::Transform,
    ecs::{Entities, System, WriteStorage, ReadStorage, Read, Join},
};

pub struct GridArrangeSystem;

type GridArrangeSystemData<'a> = (
    ReadStorage<'a, GridPosition>,
    WriteStorage<'a, Transform>,
    Read<'a, GridConfig>,
);

impl<'a> System<'a> for GridArrangeSystem {
    type SystemData = GridArrangeSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (positions, mut transforms, grid_config) = data;

        for (position, transform) in (&positions, &mut transforms).join() {
            let (offset_x, offset_y) = grid_config.offset;
            let multiplier = grid_config.tile_size * grid_config.tile_scale + grid_config.tile_margin;
            let y = position.row as f32 * multiplier;
            let x = position.col as f32 * multiplier;
            transform
                .set_xyz(offset_x, offset_y, 0.0).translate_xyz(x, y, 0.0)
                .set_scale(grid_config.tile_scale, grid_config.tile_scale, 0.0);
        }
    }
}