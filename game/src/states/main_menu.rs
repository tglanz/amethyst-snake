use std::path::Path;

use amethyst::{
    prelude::*,
    ui::{
        UiCreator,
        UiEventType,
        UiFinder
    },
    audio:: {
        output::init_output
    },
    input::{is_close_requested, is_key_down, InputBundle},
    winit::VirtualKeyCode,
};

pub struct MainMenuState;

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<GameData<'_, '_>>) {
        debug!("MainMenu::on_start");

        let world = data.world;

        init_output(&mut world.res);

        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("resources/main-menu.ron", ());
        });
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