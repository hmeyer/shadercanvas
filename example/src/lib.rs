use js_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_timer::Instant;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

#[macro_use]
extern crate log;
use log::Level;

use std::rc::Rc;

extern crate shadercanvas;

struct ShaderCanvas {
    canvas: web_sys::HtmlCanvasElement,
    context: WebGl2RenderingContext,
    iresolution_loc: Option<WebGlUniformLocation>,
    imouse_loc: Option<WebGlUniformLocation>,
    itime_loc: Option<WebGlUniformLocation>,
    _vertex_count: usize,
    time: Instant,
}

impl ShaderCanvas {
    fn new(canvas: web_sys::HtmlCanvasElement) -> Result<ShaderCanvas, JsValue> {
        let context: WebGl2RenderingContext = canvas
            .get_context("webgl2")
            .map_err(|e| format!("Cannot get webgl2 context: {:?}", e.as_string()))?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .or(Err(String::from(
                "Cannot cast context to WebGl2RenderingContext",
            )))?;

        let vert_shader = compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            r##"#version 300 es
    
            in vec4 position;

            void main() {
                gl_Position = position;
            }
            "##,
        )?;

        let frag_shader = compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            r##"#version 300 es
        
            precision highp float;
            out vec4 outColor;


            uniform vec2 iResolution;
            uniform vec2 iMouse;
            uniform float iTime;

            void mainImage( out vec4 fragColor, in vec2 fragCoord )
            {
                // Normalized pixel coordinates (from 0 to 1)
                vec2 uv = fragCoord/iResolution.xy;
            
                // Time varying pixel color
                vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));
            
                // Output to screen
                fragColor = vec4(col,1.0);
            }        
            
            void main() {
                mainImage(outColor, gl_FragCoord.xy);
            }
            "##,
        )?;

        let program = link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));

        let iresolution_loc = context.get_uniform_location(&program, "iResolution");
        let imouse_loc = context.get_uniform_location(&program, "iMouse");
        let itime_loc = context.get_uniform_location(&program, "iTime");

        let vertices: [f32; 18] = [
            -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
            -1.0, 0.0,
        ];

        let position_attribute_location = context.get_attrib_location(&program, "position");
        let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        let positions_array_buf_view = js_sys::Float32Array::from(&vertices[..]);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        let vao = context
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        context.bind_vertex_array(Some(&vao));

        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(position_attribute_location as u32);

        context.bind_vertex_array(Some(&vao));

        Ok(ShaderCanvas {
            canvas,
            context,
            iresolution_loc,
            imouse_loc,
            itime_loc,
            _vertex_count: vertices.len() / 3,
            time: Instant::now(),
        })
    }

    fn draw(&self) {
        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        self.context.uniform2fv_with_f32_array(
            self.iresolution_loc.as_ref(),
            &[self.canvas.width() as f32, self.canvas.height() as f32],
        );
        let now = (self.time.elapsed().as_millis() as f64 / 1000.0) as f32;
        self.context
            .uniform1fv_with_f32_array(self.itime_loc.as_ref(), &[now]);
        self.context.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            self._vertex_count as i32,
        );
    }
}

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

    let shader_canvas = ShaderCanvas::new(canvas)?;
    let shader_canvas = Rc::new(shader_canvas);

    {
        let clone = shader_canvas.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            info!("called event");
            clone.draw();
        });
        shader_canvas
            .canvas
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    shader_canvas.draw();

    Ok(())
}

fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
