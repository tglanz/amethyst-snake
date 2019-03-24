mod ui_handles;
mod render_handles;
mod audio_handles;

use amethyst::{
    assets::{
        ProgressCounter,
    },
    ecs::{
        World,
    },
};

pub use self::ui_handles::UiHandles;
pub use self::render_handles::RenderHandles;
pub use self::audio_handles::AudioHandles;

pub trait Loadable {
    fn load(world: &mut World, progress_counter: &mut ProgressCounter) -> Self;
}

pub fn load_to_storage(world: &mut World, progress_counter: &mut ProgressCounter) {
    let audio_handles = AudioHandles::load(world, progress_counter);
    world.add_resource(audio_handles);

    let ui_handles = UiHandles::load(world, progress_counter);
    world.add_resource(ui_handles);

    let render_handles = RenderHandles::load(world, progress_counter);
    world.add_resource(render_handles);
}