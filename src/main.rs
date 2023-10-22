extern crate nalgebra_glm as glm;

pub mod gl;
pub mod shader;
pub mod utils;

use gl::types::*;
use glfw::Context;
use image::GenericImageView;
use std::os::raw::c_void;
use utils::to_c_str;

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

// const fragment_shader_source: &str = r#

fn main() {
    let mut trans = glm::Mat4::identity();
    trans = glm::rotate(&trans, 0.5 * glm::pi::<f32>(), &glm::vec3(0., 0., 1.));
    trans = glm::scale(&trans, &glm::vec3(0.5, 0.5, 0.5));

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
        let shader = Shader::new("src/shaders/vertex.glsl", "src/shaders/fragment.glsl");

        #[rustfmt::skip]
        let mut vertices: Vec<f32> = vec![
    // positions          // colors           // texture coords
     0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0,   // top right
     0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0,   // bottom right
    -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0,   // bottom left
    -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0    // top left 
            ];
        let mut indices: Vec<u32> = vec![
            0, 1, 3, // First triangle
            3, 1, 2, // Second triangle
        ];

        // ebo = ElementBufferObject, vao = VertexAttributeObject, vbo = VertexBufferObject
        let (mut vao, mut vbo, mut ebo): (GLuint, GLuint, GLuint) = (0, 0, 0);

        // Create vao
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

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
            8 * std::mem::size_of::<f32>() as GLsizei,
            0 as *const c_void,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            8 * std::mem::size_of::<f32>() as GLsizei,
            (std::mem::size_of::<GLfloat>() * 3) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            8 * std::mem::size_of::<f32>() as GLsizei,
            (std::mem::size_of::<GLfloat>() * 6) as *const c_void,
        );
        gl::EnableVertexAttribArray(2);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(indices.as_slice()) as isize,
            indices.as_mut_slice().as_mut_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // note that this is allowed, the call to glVertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::BindVertexArray(0);

        // // Wireframe mode
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        // Set texture parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // Load the texture
        let img = image::open("assets/container.jpg").unwrap();
        let mut texture1: GLuint = 0;
        let mut texture2: GLuint = 0;

        // Texture 1
        gl::GenTextures(1, &mut texture1);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture1);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as GLint,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            img.as_bytes().as_ptr() as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        std::mem::drop(img);

        // Texture 2
        let img = image::open("assets/awesomeface.png").unwrap().flipv();

        gl::GenTextures(1, &mut texture2);
        gl::ActiveTexture(gl::TEXTURE1);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as GLint,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img.as_bytes().as_ptr() as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        shader.use_shader();
        shader.set_int("texture1", 0);
        shader.set_int("texture2", 1);

        (shader, vao)
    };
    while !window.should_close() {
        // Input
        process_input(&mut window);

        let mut trans = glm::Mat4::identity();
        trans = glm::translate(&trans, &glm::vec3(0.5, -0.5, 0.0));
        trans = glm::rotate(&trans, glfw.get_time() as f32, &glm::vec3(0.0, 0.0, 1.0));

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Rendering code
            shader.use_shader();

            gl::UniformMatrix4fv(
                gl::GetUniformLocation(shader.id, to_c_str("transform").as_ptr()),
                1,
                gl::FALSE,
                glm::value_ptr(&trans).as_ptr() as *const f32,
            );

            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);
        }

        // Check and call events and swap the buffers
        glfw.poll_events();
        window.swap_buffers();
    }
}
