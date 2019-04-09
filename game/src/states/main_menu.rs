use crate::{
    resources,
    states::{LevelState},
};

use amethyst::{
    prelude::*,
    assets::{AssetStorage,},
    ui::{UiEventType,UiFinder},
    input::{is_close_requested,is_key_down},
    winit::VirtualKeyCode,
};

pub struct MainMenuState;

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<GameData<'_, '_>>) {
        let ui = {
            let handles = data.world.read_resource::<resources::UiHandles>();
            handles.main_menu.clone().unwrap()
        };
        data.world.create_entity().with(ui).build();

        // Play a sound, just to demonstrate how.
        // In systems its easier, just request relevant storages
        {
            let audio_handles = data.world.read_resource::<resources::AudioHandles>();
            let source_storage = data.world.read_resource::<AssetStorage<amethyst::audio::Source>>();
            let output = data.world.read_resource::<amethyst::audio::output::Output>();
            if let Some(sfx) = source_storage.get(&audio_handles.sounds.discreet){
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
                    if let Some(entity) = data.world.exec(|finder: UiFinder| {
                        finder.find("exit_button")
                    }) {
                        if event.target == entity {
                            return Trans::Quit;
                        }
                    }

                    if let Some(entity) = data.world.exec(|finder: UiFinder| {
                        finder.find("start_button")
                    }) {
                        if event.target == entity {
                            let _ = data.world.delete_all();
                            return Trans::Push(Box::new(LevelState::new(40, 60)));
                        }
                    }
                }
            }
        }

        Trans::None
    }
}