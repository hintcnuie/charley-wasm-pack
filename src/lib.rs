mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("你好, 这里是 Charley的 Wasm示例代码， 由Rust 代码 经过wasm-pack 转译而成");
}
// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_text_content(Some("你好, 这里是 Charley的 Wasm示例代码， 由Rust 代码 经过wasm-pack 转译而成"));

    body.append_child(&val)?;

    Ok(())
}