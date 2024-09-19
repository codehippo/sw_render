# Software Renderer
This is a toy 3D software renderer implemented in Rust utilizing the `glam` & `glamour` libraries for 3D math that should be somewhat SIMD-optimized, and `softbuffet` & `winit` libraries for window management.

## Why?
Sometime ago I bought a tiny retro console called [Funkey S](https://www.funkey-project.com/) that uses the [Allwinner V3s](https://linux-sunxi.org/V3s) SoC to run all its emulation. As you can see, the SoC has no integrated GPU and as such relies on software rendering for all its graphics-related tasks.

I thought it might be an interesting exercise to try to re-implement some common 3D libraries to use only the CPU with NEON instructions. Certain parts of the code were adapted from the [tinyrenderer](https://github.com/ssloy/tinyrenderer) project and the goals of this toy project are somewhat similar to those of the [DFPSR](https://github.com/Dawoodoz/DFPSR) project but instead of C++, this project uses Rust.

## Current State
In its current state, there's not much implemented, just the basics. There's a basic mesh loader module, line-drawing algorithm (with clipping) and a perspective camera to render this to a buffer that's then displayed in a window.

- [x] `*.obj` mesh loading
- [x] Line-drawing algorithm
- [x] Line-clipping algorithm
- [x] Rudimentary buffer implementation
  - [ ] Buffer manipulation helper functions
- [x] Rudimentary windowing
  - [ ] Direct Linux framebuffer rendering
  - [ ] Resizing support
  - [ ] FPS setting
- [ ] Tests
- [ ] Cross-compilation to Funkey S (`arm32`, `musl`)
- [ ] Geometry culling
- [ ] Geometry clipping
- [ ] Z-buffer
- [ ] Shading algorithms
- [ ] Texturing
- [ ] Shadows
- [ ] ... and many more