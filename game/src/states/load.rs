use crate::{
    audio,
    states::MainMenuState,
};

use std::time;

use amethyst::{
    prelude::*,
    ecs::Entity,
    ui::{
        UiCreator,
    },
    assets::{
        AssetStorage,
        Loader,
        ProgressCounter,
    },
    renderer::{
        PngFormat, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
    },
};

pub struct LoadState {
    start_time: time::Instant,
    ui_root_entity: Option<Entity>,
    snake_spritesheet_handle: Option<SpriteSheetHandle>,
    progress_counter: ProgressCounter,
}

impl LoadState {
    pub fn new() -> Self {
        LoadState {
            start_time: time::Instant::now(),
            ui_root_entity: None,
            progress_counter: ProgressCounter::new(),
            snake_spritesheet_handle: None,
        }
    }

    fn load_snake_sprite_sheet(&mut self, world: &mut World) {
        debug!("loading snake sprite sheet");

        let loader = world.read_resource::<Loader>();
        let spritesheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        let texture_handle = loader.load(
            "texture/snake.spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            &mut self.progress_counter,
            &texture_storage
        );
        
        self.snake_spritesheet_handle = Some(loader.load(
            "texture/snake.spritesheet.ron",
            SpriteSheetFormat,
            texture_handle,
            &mut self.progress_counter,
            &spritesheet_storage
        ));
    }
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        debug!("LoadState::on_start - move along, there is nothing to see here");

        let world = data.world;

        world.exec(|mut creator: UiCreator<'_>| {
            self.ui_root_entity = Some(creator.create("resources/load.ron", ()));
        });

        audio::initialize_audio(world);
        self.load_snake_sprite_sheet(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.start_time.elapsed() > time::Duration::from_secs(1) {
            
            if self.progress_counter.is_complete() {
                if data.world.delete_entity(self.ui_root_entity.unwrap()).is_ok() {
                    return Trans::Switch(Box::new(MainMenuState));
                }
            }
        }

        return Trans::None;
    }
}