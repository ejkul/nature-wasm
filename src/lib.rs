extern crate cfg_if;
extern crate wasm_bindgen;
extern crate nalgebra as na;

extern crate web_sys;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use na::{Vector2, Point2};

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen(module="../www/index.js")]
extern "C" {
    fn draw_circle(x:f32,y:f32);
}

#[derive(Clone, Copy)]
enum GameObjectType {
    Ball
}

#[derive(Clone)]
pub struct GameObject {
    tag: GameObjectType,
    pos: Point2<f32>,
    vel: Vector2<f32>,
    angle: f32,
}

impl GameObject {
    pub fn pos(&self) -> Point2<f32> {
        self.pos
    }
}

fn create_ball() -> GameObject {
    GameObject {
        tag: GameObjectType::Ball,
        pos: Point2::new(10.0, 10.0),
        vel: Vector2::new(1.0, 1.0),
        angle: 0.0
    }
}

fn update_position (object: &mut GameObject) {
    object.pos = object.pos + object.vel;
}

#[wasm_bindgen]
#[derive (Clone)]
pub struct World {
    width: u32,
    height: u32,
    objects: Vec<GameObject>
}

#[wasm_bindgen]
impl World {
    pub fn new(width: u32, height: u32) -> World {
        let mut objects = Vec::new();
        objects.push(create_ball());
        World {
            width,
            height,
            objects
        }
    }
    pub fn update(&mut self) {
        for mut object in &mut self.objects {
            //log!("{}", object.pos[1] > self.height as f32);
            if object.pos[0] > self.width as f32 || object.pos[0] < 0.0{
                object.vel[(0,0)] = object.vel[(0,0)] * (-1.0);
            }
            if object.pos[1] > self.height as f32 || object.pos[1] < 0.0{
                object.vel[(1,0)] = object.vel[(1,0)] * (-1.0);
            }
            draw_circle(object.pos[0], object.pos[1]);
            update_position(&mut object);
        }


    }
}
