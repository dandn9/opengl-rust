pub mod gl;
pub mod program;

use std::cell::RefCell;
use std::ffi;
use std::ffi::CString;
use std::os;
use std::os::raw::c_void;
use std::ptr;
use glfw::Context;
use gl::types::*;

use crate::program::Program;

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

        let program = Program::new(vertex_shader_source, fragment_shader_source, &mut info_log);

        #[rustfmt::skip]
        let mut vertices: [f32; 12] = 
                [   0.5, 0.5, 0.0, 
                    0.5, -0.5, 0.0,
                    -0.5, -0.5, 0.0,
                    -0.5, 0.5, 0.0
                ];
        #[rustfmt::skip]
        let mut indices: [u32; 6] = [
            0, 1, 3, // First triangle
            1, 2, 3
        ];
        // ebo = ElementBufferObject, vao = VertexAttributeObject, vbo = VertexBufferObject
        let (mut ebo, mut vao, mut vbo): (GLuint, GLuint, GLuint) = (0, 0, 0);

        // Create vao
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        // Copy vertex data to the buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as isize,
            vertices.as_mut_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // Copy indices data to the buffer
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, std::mem::size_of_val(&indices) as isize, indices.as_mut_ptr() as *const c_void, gl::STATIC_DRAW );

        // And create a pointer with size
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to glVertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::BindVertexArray(0);

        // // Wireframe mode
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);




        (program, vao)
    };
    while !window.should_close() {
        // Input
        process_input(&mut window);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Rendering code
            gl::UseProgram(program.program_id);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null_mut());
        }

        // Check and call events and swap the buffers
        glfw.poll_events();
        window.swap_buffers();
    }
}
