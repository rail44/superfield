#![feature(custom_attribute)]

extern crate wasm_bindgen;
extern crate squark;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use web_sys::{window, console};

#[wasm_bindgen]
pub fn run(path: &str) {
    let closure = Closure::wrap(Box::new(|_| {
        console::log_1(&"hog".into());
    }) as Box<FnMut(JsValue)>);
    window().unwrap().fetch_with_str(path).then(&closure);
}
