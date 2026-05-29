use opengl_learn::shader::Shader;
use opengl_learn::{GlApp, run_app};
use std::ffi::c_void;
use std::ptr::null;

struct ShaderLession {
    vao: u32,
    shader: Shader,
}

impl GlApp for ShaderLession {
    fn setup() -> Self {
        let vao = setup_scene();
        let mut shader = Shader::new();
        shader.load_shader(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/shader/02_texture/vertex.vs"
            ),
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/shader/02_texture/fragment.fs"
            ),
        );
        Self { vao, shader }
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.shader.use_shader();
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

fn setup_scene() -> u32 {
    let vertices: [f32; 18] = [
        // positions    // colors
        0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
    ];

    let tex_coords: [f32; 6] = [
        0.0, 0.0, // lower-left corner
        1.0, 0.0, // lower-right corner
        0.5, 1.0, // top-center corner
    ];

    let mut vao = 0;
    let mut vbo = 0;

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

        // position attribute
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * size_of::<f32>()) as i32,
            null(),
        );
        gl::EnableVertexAttribArray(0);

        // color attribute
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * size_of::<f32>()) as i32,
            (3 * size_of::<f32>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        // texture
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as i32);

        vao
    }
}

fn main() {
    run_app::<ShaderLession>("Shader");
}
