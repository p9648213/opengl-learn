// Output the vertex position to the fragment shader using the out keyword
// and set the fragment's color equal to this vertex position (see how even
// the vertex position values are interpolated across the triangle). Once
// you managed to do this; try to answer the following question:
// why is the bottom-left side of our triangle black?

/*
Answer to the question: Do you know why the bottom-left side is black?
-- --------------------------------------------------------------------
Think about this for a second: the output of our fragment's color is equal to the (interpolated) coordinate of
the triangle. What is the coordinate of the bottom-left point of our triangle? This is (-0.5f, -0.5f, 0.0f). Since the
xy values are negative they are clamped to a value of 0.0f. This happens all the way to the center sides of the
triangle since from that point on the values will be interpolated positively again. Values of 0.0f are of course black
and that explains the black side of the triangle.
*/

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
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/shader/01_ex_3/vertex.vs"),
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/shader/01_ex_3/fragment.fs"
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

        vao
    }
}

fn main() {
    run_app::<ShaderLession>("Shader");
}
