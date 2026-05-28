use rs_opengl::{GlApp, run_app};
use std::ffi::{CString, c_void};
use std::ptr::null;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() { gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0); }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() { FragColor = vec4(1.0, 0.5, 0.2, 1.0); }
"#;

struct Ex1 {
    vao: u32,
    shader_program: u32,
}

impl GlApp for Ex1 {
    fn setup() -> Self {
        let (vao, shader_program) = setup_scene();
        Self {
            vao,
            shader_program,
        }
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(self.shader_program);
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
        }
    }
}

fn setup_scene() -> (u32, u32) {
    let vertices: [f32; 15] = [
        -1.0, 0.0, 0.0, -0.5, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 0.0, 1.0, 0.0, 0.0,
    ];
    let indices: [u32; 6] = [0, 1, 2, 2, 3, 4];

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
        shader
    }
}

fn main() {
    run_app::<Ex1>("Ex 1");
}
