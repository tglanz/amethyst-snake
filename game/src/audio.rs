use amethyst::{
    assets::{
        Loader
    },
    audio::{
        output::{
            init_output
        },
        OggFormat, 
        SourceHandle
    },
    ecs::{
        prelude::
        {
            World,
        },
    },
};

pub struct Sounds {
    pub boop: SourceHandle,
    pub discreet: SourceHandle
}

fn ogg(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), (), &world.read_resource())
}

pub fn initialize_audio(world: &mut World){
    
    init_output(&mut world.res);

    world.add_resource({
        let loader = world.read_resource::<Loader>();
        Sounds {
            boop: ogg(&loader, &world, "audio/boop.ogg"),
            discreet: ogg(&loader, &world, "audio/discreet.ogg"),
        }
    });
}