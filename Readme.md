# OpenGL Object Wrapper for Rust

## 语言 Language

[简体中文](Readme-CN.md) | English

## Intro

This is a wrapper library for OpenGL objects, encapsulating OpenGL objects as Rust structs using RAII rules. The underlying OpenGL library uses the [glcore-rs](https://crates.io/crates/glcore-rs) crate.

### The encapsulated objects are:
* `Buffer`: Buffer object
* `Shader`: Shader object
* `Texture`: Texture object
* `Framebuffer`: Framebuffer object
* `Pipeline`: VAO object
* `Mesh`: Mesh object, which has a vertex buffer, an element buffer (vertex index), an instance buffer, and a rendering command buffer.

### Extended Encapsulation:
* `BufferVecStatic`/`BufferVecDynamic`/trait `BufferVec`: Encapsulates buffer objects into a generic structure similar to `Vec`, allowing for easier modification of buffer contents.
* `Material`: A material library, distinguishing between conventional (`MaterialLegacy`) and physically based optics rendering (PBR) (`MaterialPbr`). Each member can be either a texture (`Texture`) or a color value (`Vec4`).
* `Meshset`/`Pipelineset`: A mesh set, each mesh has a corresponding name, material, and shader.

### Features:
* All objects requiring Bind implement a `bind()` method according to RAII rules and return a binding guard that provides various functions supported by the bound object.
* Bind guards can be undone by calling `unbind()` or automatically by `drop()`.
* `Pipeline` accepts user-provided vertex and instance structs as input, automatically parsing the struct members and associating them with shader attribute inputs (Attrib) by name and type.
* `Shader` can export and import compiled shader binaries, supporting both standard rendering (VS -> GS -> FS) and general computation (CS).
* `Texture` supports 1D, 2D, 3D, and Cube textures. It supports loading textures directly from `ImageBuffer` or file paths (JPEG loading is optimized by [turbojpeg](https://crates.io/crates/turbojpeg)), and supports asynchronous texture uploading and downloading using PBOs.
* `Framebuffer` can read shader outputs from `Shader` and then bind all its texture names to the shader output names.
* Because `Mesh` comes with its own Command Buffer object, it naturally supports acceleration of `glMultiDrawIndirect()`.

### Common Usage
* Create an OpenGL context using GLFW and EGL.
* Once you have an OpenGL context, instantiate `GLCore`, which manages all OpenGL API function pointers.
* Once you have an instance of `GLCore`, you can create and use various objects in this library.

See the unit tests.
