use std::f32::consts::PI;

use crate::{
    resources::SpriteConfig,
    types::{Direction},
    components::{SnakeHead, SnakeLimb},
};

use amethyst::{
    core::{
        Transform,
    },
    ecs::{System, WriteStorage, ReadStorage, Read, Join},
    renderer::{SpriteRender},
};

pub struct SnakeSprite;

type Data<'a> = (
    WriteStorage<'a, SpriteRender>,
    WriteStorage<'a, Transform>,
    ReadStorage<'a, SnakeHead>,
    ReadStorage<'a, SnakeLimb>,
    Read<'a, SpriteConfig>,
);

impl<'a> System<'a> for SnakeSprite {
    type SystemData = Data<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (mut sprites, mut transforms, heads, limbs, config) = data;

        for (head, sprite, transform) in (&heads, &mut sprites, &mut transforms).join() {
            sprite.sprite_number = config.snake_head_up;
            transform.set_rotation_euler(0.0, 0.0, match head.direction {
                Direction::Up => 0.0,
                Direction::Right => -PI / 2.0,
                Direction::Down => PI,
                Direction::Left => PI / 2.0,
            });
        }

        for (limb, sprite, transform) in (&limbs, &mut sprites, &mut transforms).join() {
            let (number, rotation) = match limb.ttl {
                1 => (config.snake_tail_down, match limb.directions {
                    (_, Direction::Down) => 0.0,
                    (_, Direction::Left) => - PI / 2.0,
                    (_, Direction::Up) => PI,
                    (_, Direction::Right) => PI / 2.0,
                }),
                _ => match limb.directions {
                    // just vertical
                    (Direction::Down, Direction::Up) => (config.snake_limb_up_up, 0.0),
                    (Direction::Up, Direction::Down) => (config.snake_limb_up_up, 0.0),
                    (Direction::Up, Direction::Up) => (config.snake_limb_up_up, 0.0),
                    (Direction::Down, Direction::Down) => (config.snake_limb_up_up, 0.0),

                    // just horizontal
                    (Direction::Right, Direction::Right) => (config.snake_limb_up_up, PI / 2.0),
                    (Direction::Left, Direction::Left) => (config.snake_limb_up_up, PI / 2.0), // same as right right
                    (Direction::Right, Direction::Left) => (config.snake_limb_up_up, PI / 2.0), // same as right right
                    (Direction::Left, Direction::Right) => (config.snake_limb_up_up, PI / 2.0), // same as right right

                    // For below, remember the semantics of the direction property! is DIRECTION (towards), not FROM where

                    // from bottom, to right - no rotation
                    (Direction::Up, Direction::Right) => (config.snake_limb_up_up, 0.0),
                    (Direction::Left, Direction::Down) => (config.snake_limb_up_right, 0.0), 

                    // from bottom, to left - rotate once, negatively
                    (Direction::Up, Direction::Left) => (config.snake_limb_up_right, -PI / 2.0),
                    (Direction::Right, Direction::Down) => (config.snake_limb_up_up, -PI / 2.0),

                    // from left, to top - rotate twice, positively
                    (Direction::Right, Direction::Up) => (config.snake_limb_up_right, PI),
                    (Direction::Down, Direction::Left) => (config.snake_limb_up_right, PI),

                    // from top, to right - rotate once, positively
                    (Direction::Down, Direction::Right) => (config.snake_limb_up_right, PI / 2.0),
                    (Direction::Left, Direction::Up) => (config.snake_limb_up_right, PI / 2.0),
                }
            };

            sprite.sprite_number = number;
            transform.set_rotation_euler(0.0, 0.0, rotation);
        }
    }
}