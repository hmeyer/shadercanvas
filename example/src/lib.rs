use shadercanvas::ShaderCanvas;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[macro_use]
extern crate log;
use log::Level;

use std::rc::Rc;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Debug).unwrap();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let shader_canvas = ShaderCanvas::new(canvas.clone())?;
    let shader_program = include_str!("shader.glsl");
    shader_canvas.set_shader(shader_program);
    let shader_canvas = Rc::new(shader_canvas);

    {
        let clone = shader_canvas.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            clone.draw();
        });
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    shader_canvas.draw();

    Ok(())
}
