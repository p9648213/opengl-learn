use image::GenericImageView;
use opengl_learn::shader::Shader;
use opengl_learn::{GlApp, run_app};
use std::ffi::c_void;
use std::ptr::null;

struct Image {
    width: u32,
    height: u32,
    nr_channel: u32,
    data: Vec<u8>,
}

struct TextureLession {
    vao: u32,
    shader: Shader,
    texture: u32,
}

impl GlApp for TextureLession {
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

        let image = load_image(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/texture/container.jpg"
        ));

        // texture
        let mut texture: u32 = 0;

        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            // set the texture wrapping/filtering options (on the currently bound texture object)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // generate the texture
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                image.width as i32,
                image.height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                image.data.as_ptr() as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Self {
            vao,
            shader,
            texture,
        }
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.shader.use_shader();
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
        }
    }
}

fn setup_scene() -> u32 {
    let vertices: [f32; 32] = [
        // positions     // colors      // texture coords
        0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
        -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
    ];

    let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];

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

        // position attribute
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * size_of::<f32>()) as i32,
            null(),
        );
        gl::EnableVertexAttribArray(0);

        // color attribute
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * size_of::<f32>()) as i32,
            (3 * size_of::<f32>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        // texture attribut
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            (8 * size_of::<f32>()) as i32,
            (6 * size_of::<f32>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(2);

        vao
    }
}

fn load_image(path: &str) -> Image {
    let img = image::open(path).expect("Failed to load image");
    let (width, height) = img.dimensions();
    let rgb = img.flipv().to_rgb8();
    let data = rgb.into_raw();

    Image {
        width,
        height,
        nr_channel: 3,
        data,
    }
}

fn main() {
    run_app::<TextureLession>("Texture");
}
