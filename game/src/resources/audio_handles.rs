use super::Loadable;

use amethyst::{
    assets::{ Loader, ProgressCounter },
    audio::{ OggFormat, SourceHandle },
    ecs::World,
};

pub struct Sounds {
    pub boop: SourceHandle,
    pub discreet: SourceHandle
}

pub struct AudioHandles {
    pub sounds: Sounds,
}

fn ogg(file: &str, loader: &Loader, world: &World, progress_counter: &mut ProgressCounter) -> SourceHandle {
    loader.load(file, OggFormat, (), &mut *progress_counter, &world.read_resource())
}

impl Loadable for AudioHandles {
    fn load(world: &mut World, progress_counter: &mut ProgressCounter) -> Self {
        let loader = world.read_resource::<Loader>();

        AudioHandles {
            sounds: Sounds {
                boop: ogg("resources/audio/boop.ogg", &loader, &world, &mut *progress_counter),
                discreet: ogg("resources/audio/discreet.ogg", &loader, &world, &mut *progress_counter),
            }
        }
    }
}
