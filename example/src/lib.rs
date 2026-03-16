use shadercanvas::ShaderCanvas;
use std::cell::RefCell;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

thread_local! {
    static SHADER_CANVAS: RefCell<Option<ShaderCanvas>> = const { RefCell::new(None) };
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let mut shader_canvas = ShaderCanvas::new(canvas.clone())?;
    let shader_program = include_str!("shader.glsl");
    shader_canvas.set_shader(shader_program)?;

    SHADER_CANVAS.with(|sc| {
        *sc.borrow_mut() = Some(shader_canvas);
    });

    // Expose the default shader so the editor can read it without duplicating
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("defaultShader"),
        &JsValue::from_str(shader_program),
    )?;

    // Register set_shader as a global function so the editor can call it
    let set_shader_closure = Closure::<dyn Fn(String) -> JsValue>::new(|code: String| {
        SHADER_CANVAS.with(|sc| {
            let mut borrow = sc.borrow_mut();
            match borrow.as_mut() {
                Some(canvas) => match canvas.set_shader(&code) {
                    Ok(()) => JsValue::NULL,
                    Err(e) => JsValue::from_str(&format!("{:?}", e)),
                },
                None => JsValue::from_str("ShaderCanvas not initialized"),
            }
        })
    });
    js_sys::Reflect::set(
        &window,
        &JsValue::from_str("setShader"),
        set_shader_closure.as_ref(),
    )?;
    set_shader_closure.forget();

    let draw_closure = Closure::<dyn Fn()>::new(move || {
        SHADER_CANVAS.with(|sc| {
            if let Some(ref canvas) = *sc.borrow() {
                canvas.draw();
            }
        });
    });

    let redraw_interval = std::time::Duration::from_millis(50);
    window.set_interval_with_callback_and_timeout_and_arguments_0(
        draw_closure.as_ref().unchecked_ref(),
        redraw_interval.as_millis() as i32,
    )?;
    draw_closure.forget();
    Ok(())
}
