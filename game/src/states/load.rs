use crate::states::MainMenuState;

use amethyst::{
    prelude::*,
};

pub struct LoadState;

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<GameData<'_, '_>>) {
        debug!("LoadState::on_start - move along, there is nothing to see here");
    }

    fn handle_event(
        &mut self,
        _data: StateData<GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        Trans::Switch(Box::new(MainMenuState))
    }
}