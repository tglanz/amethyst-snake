// Crates
extern crate amethyst;

extern crate log;

extern crate serde;
extern crate serde_derive;

// Modules
mod types;
mod hacks;
mod states;
mod components;
mod systems;
mod resources;
mod bundle;

use amethyst::{
    prelude::*,
    assets::{Processor},
    core::{TransformBundle},
    renderer::{DisplayConfig, RenderBundle, Pipeline, DrawFlat2D, Stage, ColorMask, ALPHA},
    ui::{DrawUi, UiBundle},
    input::{InputBundle},
};

const VERSION: (usize, usize, usize) = (0, 0, 1);

pub fn initialize() -> amethyst::Result<()> {
    log::info!("version: {}.{}.{}", VERSION.0, VERSION.1, VERSION.2);
    amethyst::start_logger(Default::default());
    
    let root_dir = amethyst::utils::application_root_dir();
    log::info!("root directory: {}", root_dir);

    let resources_dir = format!("{}/resources", root_dir);
    log::info!("resources directory: {}", resources_dir);

    let display_config_path = format!("{}/display-config.ron", resources_dir);
    let display_config = DisplayConfig::load(&display_config_path);

    // The rendering pipeline
    let pipe = Pipeline::build()
        .with_stage(Stage::with_backbuffer()
            .clear_target([0.2, 0.4, 0.5, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()
                .with_transparency(ColorMask::all(), ALPHA, None))
            .with_pass(DrawUi::new()));
    
    let input_bindings_path = format!("{}/input-bindings.ron", resources_dir);
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(input_bindings_path)?;

    let game_data = GameDataBuilder::default()
        // For audio prefabs
        .with(Processor::<amethyst::audio::Source>::new(), "audio_source_processor", &[])
        // Transformations ofcourse
        .with_bundle(TransformBundle::new())?
        // Render is useful
        .with_bundle(RenderBundle::new(pipe, Some(display_config))
            .with_sprite_sheet_processor())?
        // Input events
        .with_bundle(input_bundle)?
        // UI stuff
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(bundle::Bundle::default())?;


    let mut app = Application::new(root_dir, states::LoadState::new(), game_data)?;

    app.run();

    Ok(())
}