use glfw::{Action, Context, Key};
use std::{
    ffi::{CString, c_void},
    ptr::null,
};

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    };
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    uniform vec4 ourColor;
    void main() {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    };
"#;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(800, 600, "Learn OpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    let (width, height) = window.get_framebuffer_size();

    window.set_key_polling(true);
    window.make_current();

    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| match window.get_proc_address(symbol) {
        Some(ptr) => ptr as *const _,
        None => std::ptr::null(),
    });

    let vertices: [f32; 9] = [0.5, -0.5, 0.0, -0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let mut vbo: u32 = 0;
    let mut vao: u32 = 0;
    let vertex_shader: u32;
    let fragment_shader: u32;
    let shader_program: u32;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<f32>()) as isize,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(VERTEX_SHADER_SOURCE).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), null());
        gl::CompileShader(vertex_shader);

        fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), null());
        gl::CompileShader(fragment_shader);

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * size_of::<f32>()) as i32,
            null(),
        );
        gl::EnableVertexAttribArray(0);

        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        gl::Viewport(0, 0, width, height);
    }

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            gl::BindVertexArray(0);
        }

        glfw.poll_events();
        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height)
        },
        _ => {}
    }
}
