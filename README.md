# ShaderCanvas

[![CI](https://github.com/hmeyer/shadercanvas/actions/workflows/ci.yml/badge.svg)](https://github.com/hmeyer/shadercanvas/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/shadercanvas.svg)](https://crates.io/crates/shadercanvas)
[![License: MIT or Apache-2.0](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue.svg)](https://github.com/hmeyer/shadercanvas)

A [Shadertoy](https://www.shadertoy.com/)-compatible WebGL canvas for Rust/WASM applications.

**[Live Demo](https://hmeyer.github.io/shadercanvas/)**

## What is it?

ShaderCanvas lets you run GLSL fragment shaders on an HTML canvas from Rust. You write a `mainImage` function — the same signature used by Shadertoy — and ShaderCanvas takes care of the WebGL boilerplate: compiling the shader, uploading geometry, and feeding the standard uniforms on every frame.

## Why Rust?

If you're building a WASM application in Rust and want to embed a shader effect, ShaderCanvas keeps everything in one language. There's no need to drop into raw JavaScript to manage WebGL state. You hand it a canvas element, call `set_shader`, call `draw` in your render loop, and you're done.

## Uniforms

The following uniforms are automatically available in every shader:

| Uniform | Type | Description |
|---|---|---|
| `iResolution` | `vec2` | Canvas size in pixels |
| `iTime` | `float` | Elapsed time in seconds |
| `iMouse` | `vec2` | Mouse position in pixels (origin bottom-left) |

Additional uniforms can be set from Rust via `uniform_matrix4fv`.

## How it works

ShaderCanvas renders a full-screen quad (two triangles covering the clip space) and runs your fragment shader on every pixel. The vertex shader is fixed — it just positions the quad. Your `mainImage` function is wrapped with the uniform declarations and a `main()` that calls it, matching the Shadertoy convention:

```glsl
void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    // fragCoord is the pixel coordinate (origin bottom-left)
    // fragColor is the output colour
}
```

## Usage

```rust
use shadercanvas::ShaderCanvas;

// canvas is a web_sys::HtmlCanvasElement
let mut sc = ShaderCanvas::new(canvas)?;
sc.set_shader("void mainImage(out vec4 c, in vec2 f) { c = vec4(1.0, 0.0, 0.0, 1.0); }")?;

// in your render loop:
sc.draw();

// pass mouse position (y flipped: HTML measures from top, WebGL from bottom):
sc.set_mouse(x, canvas_height - y);
```

## Running the example locally

```
trunk serve
```

Then open http://localhost:8080. The example includes a live shader editor — edit the GLSL and press **Compile** (or Ctrl+Enter) to update the canvas in place.
