use super::Loadable;

use amethyst::{
    ecs::{World},
    assets::{ProgressCounter, Handle},
    ui::{UiLoader, UiPrefab},
};

pub struct UiHandles {
    pub main_menu: Option<Handle<UiPrefab>>,
}

impl Loadable for UiHandles {
    fn load(world: &mut World, progress_counter: &mut ProgressCounter) -> Self {
        world.exec(|loader: UiLoader<'_>| UiHandles {
            main_menu: Some(loader.load("resources/ui/main-menu.ron", &mut *progress_counter)),
        })
    }
}