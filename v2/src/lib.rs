use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("No global `window` exists.");
    let document = window
        .document()
        .expect("Should have a document on the window.");

    let closure = Closure::wrap(Box::new(move |_event: MouseEvent| {
        web_sys::console::log_1(&"Button clicked!".into())
    }) as Box<dyn FnMut(MouseEvent)>);

    document
        .query_selector_all("button")
        .expect("No result from query_selector_all")
        .get(0)
        .expect("No button found on the page")
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;

    closure.forget();

    Ok(())
}
