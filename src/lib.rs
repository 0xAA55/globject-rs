
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod glbuffer;
pub mod glshader;
pub mod glcmdbuf;
pub mod gltexture;
pub mod glframebuffer;
pub mod buffervec;
pub mod mesh;
pub mod pipeline;

extern crate nalgebra_glm as glm;
extern crate zerocopy;
extern crate struct_iterable;

#[cfg(test)]
mod tests {
    use super::gltexture::*;
    use super::glframebuffer::*;
    use super::glshader::*;
    use super::glbuffer::*;
    use super::buffervec::*;
    use super::mesh::*;
    use super::pipeline::*;

    use std::{
        ffi::c_void,
        mem::size_of_val,
        process::ExitCode,
        rc::Rc,
    };
    use glfw::{PWindow, Action, Context, Key, GlfwReceiver, WindowEvent, SwapInterval};
    use glcore::*;
    use glm::*;

    use struct_iterable::Iterable;
    use zerocopy::Unaligned;

    #[derive(Iterable, Default, Debug, Clone, Copy)]
    pub struct MyVertex {
        position: Vec2,
    }

    #[derive(Debug)]
    enum AppError {
        GLFWInitErr,
        GLFWCreateWindowFailed,
        GLFWErr(glfw::Error),
    }

    #[derive(Debug)]
    struct AppInstance {
        window: PWindow,
        events: GlfwReceiver<(f64, WindowEvent)>,
        glcore: Rc<GLCore>,
        mesh: Rc<StaticMesh>,
        shader: Rc<Shader>,
        pipeline: Rc<Pipeline<StaticMesh>>,
    }

    impl AppInstance {
        pub fn new() -> Result<Self, AppError> {
            let mut glfw = match glfw::init(glfw::fail_on_errors) {
                Ok(glfw) => glfw,
                Err(_) => return Err(AppError::GLFWInitErr), // Due to doc, won't return `glfw::InitError::AlreadyInitialized`
            };
            let (mut window, events) = glfw.create_window(1024, 768, "GLFW Window", glfw::WindowMode::Windowed).ok_or(AppError::GLFWCreateWindowFailed)?;
            window.set_key_polling(true);
            window.make_current();
            glfw.set_swap_interval(SwapInterval::Adaptive);
            let glcore = Rc::new(GLCore::new(|proc_name|window.get_proc_address(proc_name)));
            let vertices = [
                MyVertex{position: Vec2::new(0.0, 0.0)},
                MyVertex{position: Vec2::new(1.0, 0.0)},
                MyVertex{position: Vec2::new(0.0, 1.0)},
                MyVertex{position: Vec2::new(1.0, 1.0)},
            ];
            let vertex_buffer = Buffer::new(glcore.clone(), BufferTarget::ArrayBuffer, size_of_val(&vertices), BufferUsage::StaticDraw, vertices.as_ptr() as *const c_void);
            let mesh = Rc::new(StaticMesh::new(glcore.clone(), vertex_buffer, None, None, None));
            let shader = Rc::new(Shader::new(glcore.clone(),
                Some("
                    #version 330\n

                    in vec2 position;

                    void main()
                    {
                        gl_Position = vec4(position, 0.0, 1.0);
                    }
                "),
                None,
                Some("
                    #version 330\n

                    out vec4 Color;

                    void main()
                    {
                        Color = vec4(0.0, 0.6, 0.8, 1.0);
                    }
                ")
            ).unwrap());
            let pipeline = Rc::new(Pipeline::new::<MyVertex, MyVertex>(glcore.clone(), mesh.clone(), None, shader.clone()));
            Ok(Self {
                window,
                events,
                glcore,
                mesh,
                shader,
                pipeline,
            })
        }

        pub fn run(&mut self, timeout: f64) -> ExitCode {
            let start_debug_time = self.window.glfw.get_time();
            while !self.window.should_close() {
                let time_cur_frame = self.window.glfw.get_time();
                self.glcore.glClearColor(0.0, 0.6, 0.9, 1.0);
                self.glcore.glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

                self.window.swap_buffers();
                self.window.glfw.poll_events();
                for (_, event) in glfw::flush_messages(&self.events) {
                    match event {
                        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                            self.window.set_should_close(true)
                        }
                        _ => {}
                    }
                }

                if timeout > 0.0 {
                    if time_cur_frame - start_debug_time >= timeout {
                        self.window.set_should_close(true)
                    }
                }
            }

            ExitCode::from(0)
        }
    }

    #[test]
    fn test_glfw() -> ExitCode {
        const DEBUG_TIME: f64 = 10.0;
        let mut test_app = match AppInstance::new() {
            Ok(app) => app,
            Err(e) => {
                eprintln!("GLFW App Initialize failed: {:?}", e);
                return ExitCode::from(2)
            }
        };
        test_app.run(DEBUG_TIME)
    }
}
