use crate::{
    resources::{RenderHandles},
    components::{Tile, SnakeHead, SnakeLimb},
};

use log;

use amethyst::{
    ecs::{Entities, Component, System, WriteStorage, ReadStorage, Read, Join},
    renderer::{SpriteRender, Transparent},
};

pub struct AppendMissingSpritesSystem;

type Data<'a> = (
    Entities<'a>,
    WriteStorage<'a, SpriteRender>,
    WriteStorage<'a, Transparent>,
    ReadStorage<'a, Tile>,
    ReadStorage<'a, SnakeLimb>,
    ReadStorage<'a, SnakeHead>,
    Read<'a, RenderHandles>,
);

fn append_missing_sprites<'a, C>(
    entities: &Entities<'a>,
    sprites: &mut WriteStorage<'a, SpriteRender>,
    transparencies: &mut WriteStorage<'a, Transparent>,
    render_handles: &Read<'a, RenderHandles>,
    storage: &ReadStorage<'a, C>)
    where
        C: Component {
    
    let mut pending = Vec::new();
    for (entity, _, _) in (entities, storage, !&(*sprites)).join() {
        pending.push((entity, SpriteRender {
            sprite_sheet: render_handles.spritesheet.clone().unwrap(),
            sprite_number: 0
        }));
    }

    for (entity, sprite_render) in pending.into_iter() {
        if !sprites.insert(entity, sprite_render).is_ok() {
            log::error!("failed to add sprite render to entity: {:#?}", entity);
        }

        if !transparencies.insert(entity, Transparent).is_ok() {
            log::error!("failed to add transparency to entity: {:#?}", entity);
        }
    }
}

impl<'a> System<'a> for AppendMissingSpritesSystem {
    type SystemData = Data<'a>;
    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut sprites, mut transparencies, tiles, heads, limbs, render_handles) = data;
        append_missing_sprites(&entities, &mut sprites, &mut transparencies, &render_handles, &tiles);
        append_missing_sprites(&entities, &mut sprites, &mut transparencies, &render_handles, &heads);
        append_missing_sprites(&entities, &mut sprites, &mut transparencies, &render_handles, &limbs);
    }
}