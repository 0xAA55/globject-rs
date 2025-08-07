# Rust 的 OpenGL 对象封装库

## 语言 Language

简体中文 | [English](Readme.md)

## 介绍

这是一个 OpenGL 对象的封装库，将 OpenGL 的各个对象按 RAII 规则封装为一个个的 Rust 结构体。底层的 OpenGL 库使用的是 [glcore-rs](https://crates.io/crates/glcore-rs) 这个 crate。

### 已封装的对象有：
* `Buffer`: 缓冲区对象
* `Shader`: 着色器对象
* `Texture`: 纹理对象
* `Framebuffer`: 帧缓冲对象
* `Pipeline`: VAO 对象
* `Mesh`: 网格对象，具有顶点缓冲区、元素缓冲区（顶点索引）、实例缓冲区、渲染命令缓冲区。

### 扩展封装：
* `BufferVecStatic`/`BufferVecDynamic`/trait `BufferVec`: 将缓冲区对象封装成类似于 `Vec` 一样的泛型结构体，允许更方便地修改缓冲区中的内容。
* `Material`: 材质库，按照常规方式（`MaterialLegacy`）和基于物理光学渲染的 PBR 方式（`MaterialPbr`）区分，每一个成员既可以是纹理（`Texture`）也可以是颜色值（`Vec4`）
* `Meshset`/`Pipelineset`: 网格集，每个网格都有对应的名字和材质，以及着色器。

### 特性：
* 所有需要 Bind 的对象，都根据 RAII 规则，提供 `bind()` 方法并返回一个绑定守卫，这个守卫提供被绑定对象支持的各种功能。
  * 绑定守卫既可以通过调用 `unbind()` 来解绑，也可以通过自动的 `drop()` 解绑。
* `Pipeline` 支持以用户提供的顶点结构体和实例结构体内容为输入，自动解析结构体的成员，并自动将结构体的成员按名字和类型关联到着色器的属性输入（Attrib）里面。
* `Shader` 能够导出、导入着色器编译后的二进制文件，支持普通渲染（VS -> GS -> FS）和通用计算（CS）。
* `Texture` 支持 1D、2D、3D、Cube 四种方式，支持直接从 `ImageBuffer` 或者文件路径加载纹理（其中加载 JPEG 的速度得到 [turbojpeg](https://crates.io/crates/turbojpeg) 的优化），支持 PBO 异步上传下载纹理。
* `Framebuffer` 可以从 `Shader` 读取其着色器输出，然后根据其所有的纹理名字对应着色器输出的名字来绑定。
* 因为 `Mesh` 自带 Command Buffer 对象，所以自然支持 `glMultiDrawIndirect()` 的加速。

### 常见用法
* 使用 GLFW、EGL 创建 OpenGL 上下文。
* 在具有 OpenGL 上下文的情况下，实例化 `GLCore`，其管理所有的 OpenGL API 函数指针。
* 具有 `GLCore` 的实例后，即可创建、使用本库的各种对象。

## 关于单元测试

我从 [shadertoy](https://www.shadertoy.com/view/MsjSzz) 借用了着色器来运行单元测试。如果你是作者，欢迎就此事与我联系。

见单元测试。
