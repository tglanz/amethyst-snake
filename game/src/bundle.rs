use std::{
    time::Duration
};

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
        builder.add(SpriteTileSystem, "sprite_tile", &["append_missing_sprites"]);
        builder.add(SpriteSnakeSystem, "sprite_snake", &["append_missing_sprites"]);

        // transformers
        builder.add(ArrangeGridSystem, "arrange_grid", &[]);

        // logic / input
        builder.add(DirectSnakeSystem, "direct_snake", &["input_system"]);
        builder.add(MoveSnakeSystem::new(), "move_snake", &["direct_snake"]);
        builder.add(RemoveLimbsSystem, "remove_limbs", &["move_snake"]);

        // OKers
        Ok(())
    }
}
