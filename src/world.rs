extern crate cfg_if;
extern crate wasm_bindgen;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use game_object::*;

use renderers::*;

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
        objects.push(create_circle());
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
            //draw_circle(object.pos[0], object.pos[1]);
            object.render();
            update_position(&mut object);
        }


    }
}
