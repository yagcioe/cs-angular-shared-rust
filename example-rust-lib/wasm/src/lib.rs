use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, shared!");
}

#[wasm_bindgen]
pub fn whatddup() {
    alert("whaddup");
}

#[wasm_bindgen]
pub fn get_factorial(num: u8) -> String {
    let mut f: u128 = 0;
    for _ in 0..10000000 {
        f = core::factorial(num as u128);
    }
    f.to_string()
}

#[wasm_bindgen]
pub fn add_one(num: i32) -> i32 {
    core::add_one(num)
}
