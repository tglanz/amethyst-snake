use crate::{
    audio::{
        Sounds
    },
};

use amethyst::{
    assets::{
        AssetStorage,
    },
    prelude::*,
    ui::{
        UiCreator,
        UiEventType,
        UiFinder
    },
    input::{
        is_close_requested,
        is_key_down
    },
    winit::VirtualKeyCode,
};

pub struct MainMenuState;

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<GameData<'_, '_>>) {
        debug!("MainMenu::on_start");
        data.world.exec(|mut creator: UiCreator<'_>| {
            creator.create("resources/main-menu.ron", ());
        });

        // Play a sound, just to demonstrate how.
        // In systems its easier, just request relevant storages
        {
            let sounds = data.world.read_resource::<Sounds>();
            let source_storage = data.world.read_resource::<AssetStorage<amethyst::audio::Source>>();
            let output = data.world.read_resource::<amethyst::audio::output::Output>();
            if let Some(sfx) = source_storage.get(&sounds.discreet){
                output.play_once(sfx, 1.0);
            }
        }
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    return Trans::Quit
                }
            }
            StateEvent::Ui(event) => {
                if event.event_type == UiEventType::Click {
                    data.world.setup::<UiFinder>();
                    let finder = data.world.system_data::<UiFinder>();
                    if let Some(entity) = finder.find("exit_button") {
                        if event.target == entity {
                            return Trans::Quit;
                        }
                    }
                }
            }
        }

        Trans::None
    }
}