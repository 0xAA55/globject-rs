#![allow(dead_code)]

pub mod glbuffer;
pub mod glshader;
pub mod glcmdbuf;
pub mod gltexture;
pub mod framebuffer;
pub mod buffervec;
pub mod mesh;
pub mod pipeline;

extern crate nalgebra_glm as glm;

#[cfg(test)]
mod tests {
    use super::*;

    use std::process::ExitCode;
    use glfw::{PWindow, Action, Context, Key, GlfwReceiver, WindowEvent, SwapInterval};
    use glcore::*;

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
        glcore: GLCore,
    }

    impl AppInstance {
        pub fn new() -> Result<Self, AppError> {
            let mut glfw = match glfw::init(glfw::fail_on_errors) {
                Ok(glfw) => glfw,
                Err(_) => return Err(AppError::GLFWInitErr), // Due to doc, won't return `glfw::InitError::AlreadyInitialized`
            };
            let (mut window, events) = glfw.create_window(1024, 768, "VXL Editor", glfw::WindowMode::Windowed).ok_or(AppError::GLFWCreateWindowFailed)?;
            window.set_key_polling(true);
            window.make_current();
            glfw.set_swap_interval(SwapInterval::Adaptive);
            let glcore = GLCore::new(|proc_name|window.get_proc_address(proc_name));
            Ok(Self {
                window,
                events,
                glcore,
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
