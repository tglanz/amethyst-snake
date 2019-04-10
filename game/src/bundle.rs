use crate::{
    systems::*
};

use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::DispatcherBuilder,
    core::Error,
};

#[derive(Default)]
pub struct Bundle;

impl<'a, 'b> SystemBundle<'a, 'b> for Bundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {

        // normalizers
        builder.add(AppendMissingSpritesSystem, "append_missing_sprites", &[]);
        builder.add(AppendMissingTransformsSystem, "append_missing_transforms", &[]);

        // spriters
        builder.add(TileSprite, "tile_sprite", &["append_missing_sprites"]);
        builder.add(SnakeSprite, "snake_sprite", &["append_missing_sprites"]);

        // transformers
        builder.add(GridArrangeSystem, "grid_arrange", &[]);

        // OKers
        Ok(())
    }
}
