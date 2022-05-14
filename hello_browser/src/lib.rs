use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(arg: &str) {
    let mut name = arg.to_owned();
    if arg.is_empty() {
      name = String::from("world");
    }

    alert(&format!("hello {}!", name));
}
