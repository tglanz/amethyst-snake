use crate::{
    types::*,
};

use amethyst::{
    ecs::{Component, HashMapStorage, NullStorage}
};

pub struct SnakeHead;

impl Default for SnakeHead {
    fn default() -> Self { Self }
}

pub struct SnakeLimb {
    /**
     * Indicates what is the direction the head entered with, and exited with
     * 
     * Example
     *   - The limb at the cross mark 1 has enter direction Up, exit direction Right so (Up, Right)
     *   - The limb at the cross mark 2 has enter direction Right, exit direction Right so (Right, Right)
     *         1 2
     *      ---------
     * 1,2  |  ---> |
     *      |  |    |
     *      |  ^    |
     *      ---------
     */
    pub directions: (Direction, Direction),
    pub ttl: usize,
}

impl Component for SnakeHead {
    type Storage = NullStorage<Self>;
}

impl Component for SnakeLimb {
    type Storage = HashMapStorage<Self>;
}