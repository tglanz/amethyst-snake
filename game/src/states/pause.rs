use amethyst::{
    prelude::*,
    renderer::VirtualKeyCode,
    input::is_key_down,
};

pub struct PauseState;

impl SimpleState for PauseState {
    fn handle_event(&mut self, _data: StateData<GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }
        }
        
        Trans::None
    }
}