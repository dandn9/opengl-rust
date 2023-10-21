use crate::gl;
use crate::gl::types::*;
use crate::utils::to_c_str;
pub struct Shader {
    pub id: u32,
}
impl Shader {
    pub fn use_shader(&self) -> () {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
    pub fn set_bool(&self, name: &str, value: bool) -> () {
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, to_c_str(name).as_ptr()),
                value as i32,
            );
        }
    }
    pub fn set_int(&self, name: &str, value: i32) -> () {
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, to_c_str(name).as_ptr()),
                value,
            );
        }
    }
    pub fn set_float(&self, name: &str, value: f32) -> () {
        unsafe {
            gl::Uniform1f(
                gl::GetUniformLocation(self.id, to_c_str(name).as_ptr()),
                value,
            );
        }
    }

    pub fn new(vertex_path: &'static str, fragment_path: &'static str) -> Self {
        // Read the source code files
        if let (Ok(vertex_code_v), Ok(fragment_code_v)) =
            (std::fs::read(vertex_path), std::fs::read(fragment_path))
        {
            let vertex_code = std::str::from_utf8(&vertex_code_v).unwrap();
            let fragment_code = std::str::from_utf8(&fragment_code_v).unwrap();

            let program_id = unsafe {
                let (mut vertex_shader, mut fragment_shader) = (0, 0);
                let mut success = gl::FALSE as GLint; // 0 = failure ; 1 = success
                let mut info_log: Vec<u8> = Vec::with_capacity(512);
                info_log.set_len(512 - 1); // set the last byte to null char

                ///////////// VERTEX SHADER /////////////
                vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
                gl::ShaderSource(
                    vertex_shader,
                    1,
                    &to_c_str(vertex_code).as_ptr(),
                    std::ptr::null(),
                );
                gl::CompileShader(vertex_shader);
                gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

                if success == gl::FALSE as GLint {
                    gl::GetShaderInfoLog(
                        vertex_shader,
                        512,
                        std::ptr::null_mut(),
                        info_log.as_mut_slice().as_mut_ptr() as *mut GLchar,
                    );
                    println!(
                        "\nERROR::SHADER::VERTEX::COMPILATION_FAILED \n{}",
                        std::str::from_utf8_unchecked(&info_log)
                    );
                }
                ///////////// FRAGMENT SHADER /////////////
                fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
                gl::ShaderSource(
                    fragment_shader,
                    1,
                    &to_c_str(fragment_code).as_ptr(),
                    std::ptr::null(),
                );
                gl::CompileShader(fragment_shader);
                gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);

                if success == gl::FALSE as GLint {
                    gl::GetShaderInfoLog(
                        fragment_shader,
                        512,
                        std::ptr::null_mut(),
                        info_log.as_mut_slice().as_mut_ptr() as *mut GLchar,
                    );
                    println!(
                        "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED \n{}",
                        std::str::from_utf8_unchecked(&info_log)
                    );
                }

                ///////////// PROGRAM /////////////
                let program_id: u32 = gl::CreateProgram();
                gl::AttachShader(program_id, vertex_shader);
                gl::AttachShader(program_id, fragment_shader);
                gl::LinkProgram(program_id);
                gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
                if success == gl::FALSE as GLint {
                    gl::GetProgramInfoLog(
                        program_id,
                        512,
                        std::ptr::null_mut(),
                        info_log.as_mut_slice().as_mut_ptr() as *mut GLchar,
                    );
                    println!(
                        "ERROR::PROGRAM::LINKING_FAILED\n{}",
                        // std::str::from_utf8(&info_log).unwrap()
                        std::str::from_utf8_unchecked(&info_log)
                    );
                }

                gl::DeleteShader(vertex_shader);
                gl::DeleteShader(fragment_shader);

                program_id
            };
            Self { id: program_id }
        } else {
            panic!("Couldn't read vertex or fragment file")
        }
    }
}
