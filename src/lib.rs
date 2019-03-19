extern crate cfg_if;
extern crate wasm_bindgen;
extern crate nalgebra as na;

extern crate web_sys;

mod utils;
pub mod world;
pub mod game_object;
pub mod renderers;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use na::{Vector2, Point2};

use game_object::*;
use world::*;

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
