use crate::{
    components::*,
};

use amethyst::{
    prelude::*,
};

pub fn register_components(world: &mut World) {
    world.register::<Tile>();
    world.register::<GridPosition>();
}

pub fn init_audio_output(world: &mut World){
    amethyst::audio::output::init_output(&mut world.res);
}