use std::{ffi::CString, ptr::null};

#[derive(Default)]
pub struct Shader {
    pub program_id: Option<u32>,
}

impl Shader {
    pub fn new() -> Self {
        Self { program_id: None }
    }

    pub fn load_shader(&mut self, vertex_path: &str, fragment_path: &str) {
        // 1. read vertex/fragment source code
        let vertex_shader_code =
            CString::new(std::fs::read_to_string(vertex_path).unwrap()).unwrap();
        let fragment_shader_code =
            CString::new(std::fs::read_to_string(fragment_path).unwrap()).unwrap();
        // 2. compile shaders
        let vertex: u32;
        let fragment: u32;
        let mut success = 0;
        let mut buf_size = 0;
        let info_log = CString::new("hello").unwrap().as_ptr() as *mut i8;

        unsafe {
            // vertex shader
            vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vertex_shader_code.as_ptr(), null());
            gl::CompileShader(vertex);
            gl::GetShaderiv(vertex, gl::COMPILE_STATUS, &mut success);
            if success != 0 {
                gl::GetShaderInfoLog(vertex, 512, &mut buf_size, info_log);
                eprintln!("{:#?}", info_log);
            }
            // fragment shader
            fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fragment_shader_code.as_ptr(), null());
            gl::CompileShader(fragment);
            gl::GetShaderiv(fragment, gl::COMPILE_STATUS, &mut success);
            if success != 0 {
                gl::GetShaderInfoLog(fragment, 512, &mut buf_size, info_log);
                eprintln!("{:#?}", info_log);
            }
            // shader program
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            if success != 0 {
                gl::GetProgramInfoLog(id, 512, &mut buf_size, info_log);
                eprintln!("{:#?}", info_log);
            }
            // delete shader
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            // set id
            self.program_id = Some(id);
        }
    }

    pub fn use_shader(&self) {
        if let Some(id) = self.program_id {
            unsafe {
                gl::UseProgram(id);
            }
        }
    }

    pub fn set_bool(self, name: String, value: bool) {
        if let Some(id) = self.program_id {
            let c_name = CString::new(name).unwrap();
            unsafe {
                gl::Uniform1i(gl::GetUniformLocation(id, c_name.as_ptr()), value as i32);
            }
        }
    }

    pub fn set_int(self, name: String, value: i32) {
        if let Some(id) = self.program_id {
            let c_name = CString::new(name).unwrap();
            unsafe {
                gl::Uniform1i(gl::GetUniformLocation(id, c_name.as_ptr()), value);
            }
        }
    }

    pub fn set_float(self, name: String, value: f32) {
        if let Some(id) = self.program_id {
            let c_name = CString::new(name).unwrap();
            unsafe {
                gl::Uniform1f(gl::GetUniformLocation(id, c_name.as_ptr()), value);
            }
        }
    }
}
