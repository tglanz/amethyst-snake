mod config;
mod audio_handles;
mod ui_handles;
mod render_handles;

use amethyst::{
    config::Config,
    assets::{
        ProgressCounter,
    },
    ecs::{
        World,
    },
};

pub use self::config::{GridConfig};
pub use self::audio_handles::AudioHandles;
pub use self::ui_handles::UiHandles;
pub use self::render_handles::RenderHandles;

pub trait Loadable {
    fn load(world: &mut World, progress_counter: &mut ProgressCounter) -> Self;
}

pub fn load_to_storage(world: &mut World, progress_counter: &mut ProgressCounter) {
    // we have a load function cause of amethyt's Config trait
    let game_config = config::GameConfig::load("resources/config.ron");
    world.add_resource(game_config.grid);

    let audio_handles = AudioHandles::load(world, progress_counter);
    world.add_resource(audio_handles);

    let ui_handles = UiHandles::load(world, progress_counter);
    world.add_resource(ui_handles);

    let render_handles = RenderHandles::load(world, progress_counter);
    world.add_resource(render_handles);

}