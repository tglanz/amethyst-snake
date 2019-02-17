use amethyst::{
    prelude::*,
    renderer::VirtualKeyCode,
    input::is_key_down,
};

use super::PauseState;

pub struct LevelState;

impl SimpleState for LevelState {
    fn handle_event(&mut self, _data: StateData<GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PauseState));
            }
        }
        
        Trans::None
    }
}