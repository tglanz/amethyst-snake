use crate::{
    components::{Tile, SnakeHead, SnakeLimb},
};

use log;

use amethyst::{
    core::Transform,
    ecs::{Entities, Component, System, WriteStorage, ReadStorage, Join},
};

pub struct AppendMissingTransformsSystem;

type Data<'a> = (
    Entities<'a>,
    WriteStorage<'a, Transform>,
    ReadStorage<'a, Tile>,
    ReadStorage<'a, SnakeLimb>,
    ReadStorage<'a, SnakeHead>,
);

fn append_missing_transforms<'a, C>(
    entities: &Entities<'a>,
    transforms: &mut WriteStorage<'a, Transform>,
    storage: &ReadStorage<'a, C>)
    where
        C: Component {
    
    let mut pending = Vec::new();
    for (entity, _, _) in (entities, storage, !&(*transforms)).join() {
        pending.push((entity, Transform::default()));
    }

    for (entity, transform) in pending.into_iter() {
        if !transforms.insert(entity, transform).is_ok() {
            log::error!("failed to add transform to entity: {:#?}", entity);
        }
    }
}

impl<'a> System<'a> for AppendMissingTransformsSystem {
    type SystemData = Data<'a>;
    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut transforms, tiles, heads, limbs) = data;
        append_missing_transforms(&entities, &mut transforms, &tiles);
        append_missing_transforms(&entities, &mut transforms, &heads);
        append_missing_transforms(&entities, &mut transforms, &limbs);
    }
}