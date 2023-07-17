# AsciiOpengl-Rust

Ascii Game Engine written in Rust using OpenGL (Glium)

To run example run `cargo run --example basic_example`


## Usage

Cargo.toml
```toml
[dependencies]
ascii_opengl_rust = { git = "https://github.com/F1L1Pv2/AsciiOpengl-Rust " }
```
i plan on adding it to crates.io.
i will add more documentation later

> ***Note***
> it is recommended to create and export scenes using my blender plugin https://github.com/F1L1Pv2/ascii_scene_exporter

i also recomend to look at `basic_example` and source code of [MonkeRun](https://github.com/F1L1Pv2/MonkeRun) to get idea how this engine works

## TODO

- [x] Textures
- [x] Scene editor ([ascii_scene_exporter](https://github.com/F1L1Pv2/ascii_scene_exporter))
- [x] Scene loader
- [ ] Audio (2D and 3D)
- [x] Physics ([MonkeRun](https://github.com/F1L1Pv2/MonkeRun))
- [x] UI
- [ ] UI Editor
- [x] Loading OBJ files
- [ ] Vertex Colors (FBX)
- [ ] Lighting
- [x] AsciiRenderer
- [x] Game loop
- [x] Camera
- [x] Mesh
- [x] Shaders
- [ ] Loading fbx
- [ ] Animations (FBX)
- [x] KeyboardInput (Key up, down, pressed events)
- [x] MouseInput (mouse_delta, mouse_position, mouse_button up, down, pressed events)
- [x] Lib

more may be added later
