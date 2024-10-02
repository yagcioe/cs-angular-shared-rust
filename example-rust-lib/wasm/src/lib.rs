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
pub fn get_factorial(num: i64) -> String {
    let mut f: i64 = 0;
    for _ in 0..10000000 {
        f = core::factorial(num);
    }
    f.to_string()
}

#[wasm_bindgen]
pub fn add_one(num: i32) -> i32 {
    core::add_one(num)
}

#[wasm_bindgen]
pub fn valid(value: String) -> bool {
    core::validate(value)
}
