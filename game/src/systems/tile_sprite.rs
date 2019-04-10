use crate::{
    resources::SpriteConfig,
    components::{Tile, TileType},
};

use amethyst::{
    ecs::{System, WriteStorage, ReadStorage, Read, Join},
    renderer::{SpriteRender},
};

pub struct TileSprite;

type Data<'a> = (
    WriteStorage<'a, SpriteRender>,
    ReadStorage<'a, Tile>,
    Read<'a, SpriteConfig>,
);

impl<'a> System<'a> for TileSprite {
    type SystemData = Data<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (mut sprites, tiles, sprite_config) = data;
        
        // Set the sprite numbers for tiles according to their type
        for (mut sprite, tile) in (&mut sprites, &tiles).join() {
            sprite.sprite_number = match tile.tile_type {
                TileType::Ground => sprite_config.tile_ground,
                TileType::Wall => sprite_config.tile_wall,
            };
        }
    }
}