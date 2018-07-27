#![feature(use_extern_macros)]

extern crate wasm_bindgen;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

#[test]
fn test() {
    assert!(false);
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

fn main() {
    alert("HI");
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}
