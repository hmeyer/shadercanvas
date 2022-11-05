use shadercanvas::ShaderCanvas;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::rc::Rc;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let mut shader_canvas = ShaderCanvas::new(canvas.clone())?;
    let shader_program = include_str!("shader.glsl");
    shader_canvas.set_shader(shader_program)?;
    let closure = Closure::<dyn Fn()>::new(move || {
        shader_canvas.draw();
    });        
    
    let redraw_interval = std::time::Duration::from_millis(50);
    window.set_interval_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(), redraw_interval.as_millis() as i32)?;
    closure.forget();
    Ok(())
}
