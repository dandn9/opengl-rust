pub mod gl;
pub mod shader;
pub mod utils;

use gl::types::*;
use glfw::Context;
use std::cell::RefCell;
use std::ffi;
use std::ffi::CString;
use std::os;
use std::os::raw::c_void;
use std::ptr;

use crate::shader::Shader;

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
"#;

const fragment_shader_source: &str = r#"
"#;

// const fragment_shader_source: &str = r#

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    let (mut window, _) = glfw
        .create_window(800, 600, "LearnOpenGL-Rust", glfw::WindowMode::Windowed)
        .expect("Failed to create Glfw window");

    glfw.make_context_current(Some(&window));

    gl::load(|symbol| glfw.get_proc_address_raw(symbol));

    unsafe { gl::Viewport(0, 0, 800, 600) };
    window.set_framebuffer_size_callback(framebuffer_size_callback);

    // let vbo: *mut u32 = std::ptr::null();

    // Create shaders
    let (shader, vao) = unsafe {
        let our_shader = Shader::new("src/shaders/vertex.glsl", "src/shaders/fragment.glsl");

        #[rustfmt::skip]
        let mut vertices: Vec<f32> = vec![
    // positions      // colors
     0.5, -0.5, 0.0,  1.0, 0.0, 0.0,   // bottom right
    -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom let
     0.0,  0.5, 0.0,  0.0, 0.0, 1.0    // top 
            ];
        // ebo = ElementBufferObject, vao = VertexAttributeObject, vbo = VertexBufferObject
        let (mut vao, mut vbo): (GLuint, GLuint) = (0, 0);

        // Create vao
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        // Copy vertex data to the buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(vertices.as_slice()) as isize,
            vertices.as_mut_slice().as_mut_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // And create a pointer with size
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * std::mem::size_of::<f32>() as GLsizei,
            0 as *const c_void,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * std::mem::size_of::<f32>() as GLsizei,
            (std::mem::size_of::<GLfloat>() * 3) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        // note that this is allowed, the call to glVertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::BindVertexArray(0);

        // // Wireframe mode
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (our_shader, vao)
    };
    while !window.should_close() {
        // Input
        process_input(&mut window);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Rendering code
            shader.use_shader();

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // Check and call events and swap the buffers
        glfw.poll_events();
        window.swap_buffers();
    }
}
