use shadercanvas::ShaderCanvas;
use js_sys;
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
    let cw = (window.inner_width().unwrap().as_f64().unwrap() * 0.8) as u32;
    let ch = (window.inner_height().unwrap().as_f64().unwrap() * 0.8) as u32;
    info!("setting canvas dimensions to [{}x{}].", cw, ch);
    canvas.set_width(cw);
    canvas.set_height(ch);

    let shader_canvas = ShaderCanvas::new(canvas.clone())?;
    let shader_canvas = Rc::new(shader_canvas);

    {
        let clone = shader_canvas.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            info!("called event");
            clone.draw();
        });
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    shader_canvas.draw();

    Ok(())
}
