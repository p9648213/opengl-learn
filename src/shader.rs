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
        let mut log_len = 0;
        let mut info_log = vec![0u8; 512];

        unsafe {
            // vertex shader
            vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vertex_shader_code.as_ptr(), null());
            gl::CompileShader(vertex);
            gl::GetShaderiv(vertex, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                gl::GetShaderInfoLog(
                    vertex,
                    info_log.len() as i32,
                    &mut log_len,
                    info_log.as_mut_ptr() as *mut i8,
                );
                eprintln!(
                    "VERTEX SHADER COMPILATION FAILED:\n{}",
                    String::from_utf8_lossy(&info_log[..log_len as usize])
                );
            }
            // fragment shader
            fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fragment_shader_code.as_ptr(), null());
            gl::CompileShader(fragment);
            gl::GetShaderiv(fragment, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                gl::GetShaderInfoLog(
                    fragment,
                    info_log.len() as i32,
                    &mut log_len,
                    info_log.as_mut_ptr() as *mut i8,
                );
                eprintln!(
                    "FRAGMENT SHADER COMPILATION FAILED:\n{}",
                    String::from_utf8_lossy(&info_log[..log_len as usize])
                );
            }
            // shader program
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                gl::GetProgramInfoLog(
                    id,
                    info_log.len() as i32,
                    &mut log_len,
                    info_log.as_mut_ptr() as *mut i8,
                );
                eprintln!(
                    "SHADER PROGRAM LINK FAILED:\n{}",
                    String::from_utf8_lossy(&info_log[..log_len as usize])
                );
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

    pub fn set_bool(&self, name: &str, value: bool) {
        if let Some(id) = self.program_id {
            let c_name = CString::new(name).unwrap();
            unsafe {
                gl::Uniform1i(gl::GetUniformLocation(id, c_name.as_ptr()), value as i32);
            }
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        if let Some(id) = self.program_id {
            let c_name = CString::new(name).unwrap();
            unsafe {
                gl::Uniform1i(gl::GetUniformLocation(id, c_name.as_ptr()), value);
            }
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        if let Some(id) = self.program_id {
            let c_name = CString::new(name).unwrap();
            unsafe {
                gl::Uniform1f(gl::GetUniformLocation(id, c_name.as_ptr()), value);
            }
        }
    }
}
