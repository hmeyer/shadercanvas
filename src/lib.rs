use js_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_timer::Instant;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

pub struct ShaderCanvas {
    canvas: web_sys::HtmlCanvasElement,
    context: WebGl2RenderingContext,
    time: Instant,
    iresolution_loc: Option<WebGlUniformLocation>,
    imouse_loc: Option<WebGlUniformLocation>,
    itime_loc: Option<WebGlUniformLocation>,
    program: Option<WebGlProgram>,
}

static VERTICES: &'static [f32] = &[
    -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, -1.0, 0.0,
];

static DEFAULT_SHADER: &'static str = r##"
    void mainImage( out vec4 fragColor, in vec2 fragCoord )
    {
        fragColor = vec4(0.5);
    }
"##;

static VERTEX_SHADER: &'static str = r##"#version 300 es
    in vec4 position;

    void main() {
        gl_Position = position;
    }
"##;

static FRAG_SHADER_PREFIX: &'static str = r##"#version 300 es
    precision highp float;
    out vec4 outColor;
    uniform vec2 iResolution;
    uniform vec2 iMouse;
    uniform float iTime;
"##;

static FRAG_SHADER_SUFFIX: &'static str = r##"
    void main() {
        mainImage(outColor, gl_FragCoord.xy);
    }
"##;

impl ShaderCanvas {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Result<ShaderCanvas, JsValue> {
        let context: WebGl2RenderingContext = canvas
            .get_context("webgl2")
            .map_err(|e| format!("Cannot get webgl2 context: {:?}", e.as_string()))?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .or(Err(String::from(
                "Cannot cast context to WebGl2RenderingContext",
            )))?;

        let mut result = ShaderCanvas {
            canvas,
            context,
            time: Instant::now(),
            iresolution_loc: None,
            imouse_loc: None,
            itime_loc: None,
            program: None,
        };
        result.set_shader(DEFAULT_SHADER)?;
        Ok(result)
    }

    pub fn set_shader(&mut self, shader: &str) -> Result<(), JsValue> {
        let vert_shader = compile_shader(
            &self.context,
            WebGl2RenderingContext::VERTEX_SHADER,
            VERTEX_SHADER,
        )?;

        let frag_shader = format!("{}{}{}", FRAG_SHADER_PREFIX, shader, FRAG_SHADER_SUFFIX);

        let frag_shader = compile_shader(
            &self.context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            &frag_shader,
        )?;

        let program = link_program(&self.context, &vert_shader, &frag_shader)?;
        self.context.use_program(Some(&program));

        self.iresolution_loc = self.context.get_uniform_location(&program, "iResolution");
        self.imouse_loc = self.context.get_uniform_location(&program, "iMouse");
        self.itime_loc = self.context.get_uniform_location(&program, "iTime");

        let position_attribute_location = self.context.get_attrib_location(&program, "position");
        let buffer = self
            .context
            .create_buffer()
            .ok_or("Failed to create buffer")?;
        self.context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
        let positions_array_buf_view = js_sys::Float32Array::from(&VERTICES[..]);
        self.context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        let vao = self
            .context
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        self.context.bind_vertex_array(Some(&vao));

        self.context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.context
            .enable_vertex_attrib_array(position_attribute_location as u32);
        self.context.bind_vertex_array(Some(&vao));
        Ok(())
    }

    pub fn uniform_matrix4fv(&self, uniform_name: &str, data: &[f32]) {
        if let Some(p) = &self.program {
            let loc = self.context.get_uniform_location(&p, uniform_name);
            self.context.uniform_matrix4fv_with_f32_array(loc.as_ref(), false, data);
        }
    }

    pub fn draw(&self) {
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
            (VERTICES.len() / 3) as i32,
        );
    }
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
