use std::cell::RefCell;
use std::ffi;
use std::ffi::CString;
use std::os;

use std::os::raw::c_void;
use std::ptr;

use glfw::Context;

pub mod gl;
use gl::types::*;

fn framebuffer_size_callback(width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}

fn process_input(window: &mut glfw::Window) {
    if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
        window.set_should_close(true);
    }
}

const vertex_shader_source: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const fragment_shader_source: &str = r#"
    #version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
} 
"#;

fn to_c_str(str: &str) -> CString {
    CString::new(str.as_bytes()).unwrap()
}

// const fragment_shader_source: &str = r#

fn main() {
    println!("[rust] start");
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    let (mut window, _) = glfw
        .create_window(800, 600, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create Glfw window");

    glfw.make_context_current(Some(&window));

    gl::load(|symbol| glfw.get_proc_address_raw(symbol));

    unsafe { gl::Viewport(0, 0, 800, 600) };
    window.set_framebuffer_size_callback(framebuffer_size_callback);

    // let vbo: *mut u32 = std::ptr::null();

    // Create shaders
    let (program, vao) = unsafe {
        let mut info_log: Vec<u8> = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // set the last byte to null char

        let mut vertex_shader: GLuint = 0;
        let mut fragment_shader: GLuint = 0;
        // Vertex shader
        {
            vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &to_c_str(vertex_shader_source).as_ptr(),
                ptr::null(),
            );
            gl::CompileShader(vertex_shader);

            let mut success = gl::FALSE as GLint; // 0 = failure; 1 = success
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

            if success == gl::FALSE as GLint {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );

                println!(
                    "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{:?}",
                    // std::str::from_utf8(&info_log).unwrap()
                    CString::from_vec_unchecked(info_log.clone()).to_str()
                );
            }
        }
        // Fragment Shader
        {
            fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader,
                1,
                &to_c_str(fragment_shader_source).as_ptr(),
                ptr::null(),
            );
            gl::CompileShader(fragment_shader);

            let mut success = gl::FALSE as GLint; // 0 = failure; 1 = success
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);

            if success == gl::FALSE as GLint {
                gl::GetShaderInfoLog(
                    fragment_shader,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );

                println!(
                    "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{:?}",
                    // std::str::from_utf8(&info_log).unwrap()
                    CString::from_vec_unchecked(info_log.clone()).to_str()
                );
            }
        }

        let program: u32 = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        let mut success = gl::FALSE as GLint;

        // Check if program linking was ok
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == gl::FALSE as GLint {
            gl::GetProgramInfoLog(
                program,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::PROGRAM::LINKING_FAILED\n{:?}",
                // std::str::from_utf8(&info_log).unwrap()
                CString::from_vec_unchecked(info_log.clone()).to_str()
            );
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        let mut vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
        // Bind data
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        // Create vao
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        // Create vbo buffer

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as isize,
            vertices.as_mut_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        (program, vao)
    };
    while !window.should_close() {
        // Input
        process_input(&mut window);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Rendering code
            gl::UseProgram(program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // Check and call events and swap the buffers
        glfw.poll_events();
        window.swap_buffers();
    }
}
