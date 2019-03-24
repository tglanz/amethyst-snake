use std::time;

use crate::{
    hacks, resources,
    states::{MainMenuState},
};

use amethyst::{
    prelude::*,
    ui::{UiCreator},
    assets::{Completion, ProgressCounter},
};

pub struct LoadState {
    start_time: time::Instant,
    progress_counter: ProgressCounter,
}

impl LoadState {
    pub fn new() -> Self {
        LoadState {
            start_time: time::Instant::now(),
            progress_counter: ProgressCounter::new(),
        }
    }
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.exec(|mut creator: UiCreator| {
            creator.create("resources/ui/loading.ron", &mut self.progress_counter);
        });

        hacks::init_audio_output(world);
        hacks::register_components(world);
        resources::load_to_storage(world, &mut self.progress_counter);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        match self.progress_counter.complete() {
            Completion::Loading => {
                Trans::None
            },
            Completion::Failed => {
                eprint!("Failed to load stuff, aborting");
                Trans::Quit
            },
            Completion::Complete => {
                if self.start_time.elapsed() > time::Duration::from_secs(1) {
                    let _ = data.world.delete_all();
                    Trans::Switch(Box::new(MainMenuState))
                } else {
                    Trans::None
                }
            },
        }
    }
}