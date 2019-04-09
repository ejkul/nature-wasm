extern crate nalgebra as na;

use na::{Vector2, Point2};
use renderers::*;

#[derive(Clone, Copy)]
pub enum GameObjectType {
    Circle
}

#[derive(Clone)]
pub struct GameObject {
    tag: GameObjectType,
    pub pos: Point2<f32>,
    pub vel: Vector2<f32>,
    pub angle: f32,
}

impl GameObject {
    pub fn pos(&self) -> Point2<f32> {
        self.pos
    }
    pub fn render(&self) -> () {
        match self.tag {
            GameObjectType::Circle => ()//draw_circle(self.pos[0], self.pos[1])
        }
    }
}

pub fn create_circle() -> GameObject {
    GameObject {
        tag: GameObjectType::Circle,
        pos: Point2::new(10.0, 10.0),
        vel: Vector2::new(1.0, 1.0),
        angle: 0.0
    }
}

pub fn update_position (object: &mut GameObject) {
    object.pos = object.pos + object.vel;
}
