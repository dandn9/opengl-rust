use crate::gl::types::*;
use crate::{gl, to_c_str};
pub struct Program {
    pub vs_s: &'static str,
    pub fs_s: &'static str,
    pub program_id: u32,
}
impl Program {
    pub fn new(vs_s: &'static str, fs_s: &'static str, info_log: &mut Vec<u8>) -> Self {
        let program_id = unsafe {
            let mut vertex_shader: GLuint = 0;
            let mut fragment_shader: GLuint = 0;

            // Vertex shader
            {
                vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
                gl::ShaderSource(vertex_shader, 1, &to_c_str(vs_s).as_ptr(), std::ptr::null());
                gl::CompileShader(vertex_shader);

                let mut success = gl::FALSE as GLint; // 0 = failure; 1 = success
                gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

                if success == gl::FALSE as GLint {
                    gl::GetShaderInfoLog(
                        vertex_shader,
                        512,
                        std::ptr::null_mut(),
                        info_log.as_mut_ptr() as *mut GLchar,
                    );
                    panic!(
                        "ERROR::SHADER::VERTEX::COMPILATION_FAILED \n{:?}",
                        std::ffi::CString::from_vec_unchecked(info_log.clone()).to_str()
                    );
                }
            }
            // Fragment shader
            {
                fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
                gl::ShaderSource(
                    fragment_shader,
                    1,
                    &to_c_str(fs_s).as_ptr(),
                    std::ptr::null(),
                );
                gl::CompileShader(fragment_shader);

                let mut success = gl::FALSE as GLint; // 0 = failure; 1 = success
                gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);

                if success == gl::FALSE as GLint {
                    gl::GetShaderInfoLog(
                        fragment_shader,
                        512,
                        std::ptr::null_mut(),
                        info_log.as_mut_ptr() as *mut GLchar,
                    );
                    panic!(
                        "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED \n{:?}",
                        std::ffi::CString::from_vec_unchecked(info_log.clone()).to_str()
                    );
                }
            }

            let program_id: u32 = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader);
            gl::AttachShader(program_id, fragment_shader);
            gl::LinkProgram(program_id);

            let mut success = gl::FALSE as GLint;

            // Check if program linking was ok
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            if success == gl::FALSE as GLint {
                gl::GetProgramInfoLog(
                    program_id,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "ERROR::PROGRAM::LINKING_FAILED\n{:?}",
                    // std::str::from_utf8(&info_log).unwrap()
                    std::ffi::CString::from_vec_unchecked(info_log.clone()).to_str()
                );
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            program_id
        };
        return Self {
            program_id,
            vs_s,
            fs_s,
        };
    }
}
