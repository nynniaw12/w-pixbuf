# WebGPU Pixel Buffer Renderer

It leverages the capabilities of modern GPUs through the WebGPU API for rendering, using a texture buffer between the CPU and the GPU,
similar to pixels.rs a vertex buffer of a single triangle along with a simple shader and texture sampler is used for rendering.
The renderer is compiled to wasm with wasm-pack from rust code and runs on the WebGL backend for WGPU.
Comes with a very simple demo.

## Features

- Performance: Utilizes GPU for faster rendering
- Flexibility: Custom rendering algorithms
- Cross-platform: WebGPU supports Vulkan, Metal, WebGL, DirectX and OpenGL

- [More Optimizations](https://maxisom.me/posts/applying-5-million-pixel-updates-per-second)

## Getting Started
```bash
cargo run # run natively

cargo build # build natively

wasm-pack build --target web # build for web
```

## [Demo](https://curious-semifreddo-32a300.netlify.app)

## Other
License is MIT. Feel free to contribute.

Development continues in tandem with raycasting engine. A better version of this
renderer lives there.
