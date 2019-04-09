use crate::{
    resources::{RenderHandles},
    components::{Tile, TileType},
};

use log;

use amethyst::{
    ecs::{Entities, System, WriteStorage, ReadStorage, Read, Join},
    renderer::{SpriteRender},
};

pub struct SpriteAssignSystem;

type SpriteAssignSystemData<'a> = (
    Entities<'a>,
    WriteStorage<'a, SpriteRender>,
    ReadStorage<'a, Tile>,
    Read<'a, RenderHandles>,
);

impl<'a> System<'a> for SpriteAssignSystem {
    type SystemData = SpriteAssignSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut sprites, tiles, render_handles) = data;
        
        // Find tiles that do not have a sprite render
        let mut pending = Vec::new();
        for (entity, _, _) in (&entities, &tiles, !&sprites).join() {
            let sprite_render = SpriteRender {
                sprite_sheet: render_handles.spritesheet.clone().unwrap(),
                sprite_number: 0
            };

            pending.push((entity, sprite_render));
        }

        // Add components to the entities that need them
        for (entity, sprite_render) in pending.into_iter() {
            if !sprites.insert(entity, sprite_render).is_ok() {
                log::error!("failed to add sprite render to entity: {:#?}", entity);
            }
        }

        // Set the sprite numbers for tiles according to their type
        for (mut sprite, tile) in (&mut sprites, &tiles).join() {
            sprite.sprite_number = match tile.tile_type {
                TileType::Ground => 5,
                TileType::Wall => 6,
            };
        }
    }
}