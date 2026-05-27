use std::ffi::{CString, c_void};
use std::num::NonZeroU32;
use std::ptr::null;

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasWindowHandle;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const TITLE: &str = "Triangle";

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0, 0.5, 0.2, 1.0);
    }
"#;

struct GlState {
    gl_context: PossiblyCurrentContext,
    gl_surface: Surface<WindowSurface>,
    window: Window,
    vao: u32,
    shader_program: u32,
}

#[derive(Default)]
struct App {
    state: Option<GlState>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }

        // 1. Describe the window + the kind of GL config we want.
        let window_attributes = Window::default_attributes().with_title(TITLE);
        let template = ConfigTemplateBuilder::new();

        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

        // 2. DisplayBuilder bridges winit + glutin
        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                configs
                    .reduce(|best, cfg| {
                        if cfg.num_samples() > best.num_samples() {
                            cfg
                        } else {
                            best
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let window = window.unwrap();
        let raw_window_handle = window.window_handle().unwrap().as_raw();

        // 3. The Config knows which platform Display it came from.
        let gl_display = gl_config.display();

        // 4. Build the context — NOT current yet.
        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(Some(raw_window_handle));

        let not_current_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap()
        };

        // 5. Build the surface backed by the window.
        let surface_attributes = window
            .build_surface_attributes(SurfaceAttributesBuilder::default())
            .unwrap();

        let gl_surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &surface_attributes)
                .unwrap()
        };

        // 6. Make current — only after this can gl::* do anything.
        let gl_context = not_current_context.make_current(&gl_surface).unwrap();

        // 7. Wire glutin's loader into the `gl` bindings.
        gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        });

        // 8. GL is live — build the scene and set the viewport.
        let (vao, shader_program) = setup_scene();
        let size = window.inner_size();
        unsafe {
            gl::Viewport(0, 0, size.width as i32, size.height as i32);
        }
        self.state = Some(GlState {
            gl_context,
            gl_surface,
            window,
            vao,
            shader_program,
        })
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = self.state.as_ref() else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if size.width != 0 && size.height != 0 {
                    state.gl_surface.resize(
                        &state.gl_context,
                        NonZeroU32::new(size.width).unwrap(),
                        NonZeroU32::new(size.height).unwrap(),
                    );
                    unsafe {
                        gl::Viewport(0, 0, size.width as i32, size.height as i32);
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                unsafe {
                    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    gl::UseProgram(state.shader_program);
                    gl::BindVertexArray(state.vao);
                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
                }
                state
                    .gl_surface
                    .swap_buffers(&state.gl_context)
                    .expect("Failed to swap buffers");
            }
            _ => (),
        }
    }
}

fn setup_scene() -> (u32, u32) {
    let vertices: [f32; 12] = [
        0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0,
    ];
    let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<f32>()) as isize,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<u32>()) as isize,
            indices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * size_of::<f32>()) as i32,
            null(),
        );
        gl::EnableVertexAttribArray(0);

        let shader_program = build_program();
        gl::BindVertexArray(0);
        (vao, shader_program)
    }
}

fn build_program() -> u32 {
    unsafe {
        let vs = compile_shader(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
        let fs = compile_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);

        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        let mut ok = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut ok);
        if ok == 0 {
            let mut log = vec![0u8; 512];
            let mut len = 0;
            gl::GetProgramInfoLog(program, 512, &mut len, log.as_mut_ptr() as *mut _);
            panic!(
                "program link error: {}",
                String::from_utf8_lossy(&log[..len as usize])
            );
        }

        gl::DeleteShader(vs);
        gl::DeleteShader(fs);
        program
    }
}

fn compile_shader(source: &str, shader_type: u32) -> u32 {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(source).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), null());
        gl::CompileShader(shader);

        let mut ok = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut ok);
        if ok == 0 {
            let mut log = vec![0u8; 512];
            let mut len = 0;
            gl::GetShaderInfoLog(shader, 512, &mut len, log.as_mut_ptr() as *mut _);
            panic!(
                "shader compile error: {}",
                String::from_utf8_lossy(&log[..len as usize])
            );
        }
        shader
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
