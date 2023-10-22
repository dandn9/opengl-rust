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
    pub fn set_vec2_g(&self, name: &str, value: &glm::Vec2) {
        unsafe {
            gl::Uniform2fv(
                gl::GetUniformLocation(self.id, to_c_str(name).as_ptr()),
                1,
                &value[0],
            )
        }
    }
    pub fn set_vec2_f(&self, name: &str, x: f32, y: f32) {
        unsafe {
            gl::Uniform2f(
                gl::GetUniformLocation(self.id, to_c_str(name).as_ptr()),
                x,
                y,
            );
        }
    }
    pub fn set_vec3_g(&self, name: &str, value: &glm::Vec3) {
        unsafe {
            gl::Uniform3fv(
                gl::GetUniformLocation(self.id, to_c_str(name).as_ptr()),
                1,
                &value[0],
            )
        }
    }
    pub fn set_vec3_f(&self, name: &str, x: f32, y: f32, z: f32) {
        unsafe {
            gl::Uniform3f(
                gl::GetUniformLocation(self.id, to_c_str(name).as_ptr()),
                x,
                y,
                z,
            );
        }
    }
    pub fn set_vec4_g(&self, name: &str, value: &glm::Vec4) {
        unsafe {
            gl::Uniform4fv(
                gl::GetUniformLocation(self.id, to_c_str(name).as_ptr()),
                1,
                &value[0],
            )
        }
    }
    pub fn set_vec4_f(&self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        unsafe {
            gl::Uniform4f(
                gl::GetUniformLocation(self.id, to_c_str(name).as_ptr()),
                x,
                y,
                z,
                w,
            );
        }
    }
    pub fn set_mat2(&self, name: &str, value: &glm::Mat2) {
        unsafe {
            gl::UniformMatrix2fv(
                gl::GetUniformLocation(self.id, to_c_str(name).as_ptr()),
                1,
                gl::FALSE,
                &value.m11,
            )
        }
    }
    pub fn set_mat3(&self, name: &str, value: &glm::Mat3) {
        unsafe {
            gl::UniformMatrix3fv(
                gl::GetUniformLocation(self.id, to_c_str(name).as_ptr()),
                1,
                gl::FALSE,
                &value.m11,
            )
        }
    }
    pub fn set_mat4(&self, name: &str, value: &glm::Mat4) {
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.id, to_c_str(name).as_ptr()),
                1,
                gl::FALSE,
                &value.m11,
            )
        }
    }
    fn check_compile_errors(shader: GLuint, compile_type: &str) {
        unsafe {
            let mut success = gl::FALSE as GLint; // 0 = failure ; 1 = success
            let mut info_log: Vec<u8> = Vec::with_capacity(1024);
            info_log.set_len(1024 - 1); // set the last byte to null char
            if compile_type != "PROGRAM" {
                gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
                if success == gl::FALSE as GLint {
                    gl::GetShaderInfoLog(
                        shader,
                        1024,
                        std::ptr::null_mut(),
                        info_log.as_mut_slice().as_mut_ptr() as *mut GLchar,
                    );
                    println!(
                        "\nERROR::SHADER_COMPILATION_ERROR of type:{}\n{}\n-------------------------------------\n",
                        compile_type, std::str::from_utf8_unchecked(&info_log)
                    );
                }
            } else {
                gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
                if success == gl::FALSE as GLint {
                    gl::GetProgramInfoLog(
                        shader,
                        1024,
                        std::ptr::null_mut(),
                        info_log.as_mut_slice().as_mut_ptr() as *mut GLchar,
                    );
                    println!(
                    "\nERROR::PROGRAM_LINKING_ERROR of type:{}\n{}\n-------------------------------------\n",
                    compile_type,std::str::from_utf8_unchecked(&info_log)
                );
                }
            }
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
                Self::check_compile_errors(vertex_shader, "VERTEX");

                ///////////// FRAGMENT SHADER /////////////
                fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
                gl::ShaderSource(
                    fragment_shader,
                    1,
                    &to_c_str(fragment_code).as_ptr(),
                    std::ptr::null(),
                );
                gl::CompileShader(fragment_shader);
                Self::check_compile_errors(fragment_shader, "FRAGMENT");

                ///////////// PROGRAM /////////////
                let program_id: u32 = gl::CreateProgram();
                gl::AttachShader(program_id, vertex_shader);
                gl::AttachShader(program_id, fragment_shader);
                gl::LinkProgram(program_id);
                Self::check_compile_errors(program_id, "PROGRAM");

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
