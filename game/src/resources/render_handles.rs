use super::Loadable;

use amethyst::{
    ecs::{World},
    assets::{Loader, ProgressCounter, AssetStorage},
    renderer::{PngFormat, Texture, TextureMetadata, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle}
};

pub struct RenderHandles {
    pub spritesheet: Option<SpriteSheetHandle>,
}

impl Loadable for RenderHandles {
    fn load(world: &mut World, progress_counter: &mut ProgressCounter) -> Self {
        let loader = world.read_resource::<Loader>();
        let spritesheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        let texture_handle = loader.load(
            "resources/spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            &mut *progress_counter,
            &texture_storage
        );
        
        let spritesheet = Some(loader.load(
            "resources/spritesheet.ron",
            SpriteSheetFormat,
            texture_handle,
            &mut *progress_counter,
            &spritesheet_storage
        ));

        RenderHandles {
            spritesheet
        }
    }
}