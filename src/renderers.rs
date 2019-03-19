extern crate cfg_if;
extern crate wasm_bindgen;
extern crate nalgebra as na;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module="../www/index.js")]
pub extern "C" {
    pub fn draw_circle(x:f32,y:f32);
}
