use opengl_learn::{GlApp, run_app};
use std::ffi::{CString, c_void};
use std::ptr::null;

const VERTEX_SHADER_SOURCE_1: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() { gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0); }
"#;

const FRAGMENT_SHADER_SOURCE_1: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() { FragColor = vec4(1.0, 0.5, 0.2, 1.0); }
"#;

const VERTEX_SHADER_SOURCE_2: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() { gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0); }
"#;

const FRAGMENT_SHADER_SOURCE_2: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() { FragColor = vec4(1.0, 1.0, 0.0, 1.0); }
"#;


struct Ex3 {
    vao_1: u32,
    vao_2: u32,
    shader_program_1: u32,
    shader_program_2: u32,
}

impl GlApp for Ex3 {
    fn setup() -> Self {
        let (vao_1, vao_2, shader_program_1, shader_program_2) = setup_scene();
        Self {
            vao_1,
            vao_2,
            shader_program_1,
            shader_program_2
        }
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // Triangle 1
            gl::UseProgram(self.shader_program_1);
            gl::BindVertexArray(self.vao_1);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
            // Triangle 2
            gl::UseProgram(self.shader_program_2);
            gl::BindVertexArray(self.vao_2);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);

        }
    }
}

fn setup_scene() -> (u32, u32, u32, u32) {
    let vertices_1: [f32; 9] = [-1.0, 0.0, 0.0, -0.5, 1.0, 0.0, 0.0, 0.0, 0.0];

    let vertices_2: [f32; 9] = [0.0, 0.0, 0.0, 0.5, 1.0, 0.0, 1.0, 0.0, 0.0];

    let mut vao_1 = 0;
    let mut vao_2 = 0;
    let mut vbo_1 = 0;
    let mut vbo_2 = 0;

    unsafe {
        // Triangle 1
        gl::GenVertexArrays(1, &mut vao_1);
        gl::GenBuffers(1, &mut vbo_1);
        gl::BindVertexArray(vao_1);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_1);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices_1.len() * size_of::<f32>()) as isize,
            vertices_1.as_ptr() as *const c_void,
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
        gl::BindVertexArray(0);

        // Triangle 2
        gl::GenVertexArrays(1, &mut vao_2);
        gl::GenBuffers(1, &mut vbo_2);
        gl::BindVertexArray(vao_2);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_2);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices_2.len() * size_of::<f32>()) as isize,
            vertices_2.as_ptr() as *const c_void,
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
        gl::BindVertexArray(0);

        let shader_program_1 = build_program(VERTEX_SHADER_SOURCE_1, FRAGMENT_SHADER_SOURCE_1);
        let shader_program_2 = build_program(VERTEX_SHADER_SOURCE_2, FRAGMENT_SHADER_SOURCE_2);

        (vao_1, vao_2, shader_program_1, shader_program_2)
    }
}

fn build_program(vertex_shader_source: &str, fragment_shader_source: &str) -> u32 {
    unsafe {
        let vs = compile_shader(vertex_shader_source, gl::VERTEX_SHADER);
        let fs = compile_shader(fragment_shader_source, gl::FRAGMENT_SHADER);
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
    run_app::<Ex3>("Ex 3");
}

