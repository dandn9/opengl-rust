extern crate nalgebra_glm as glm;

pub mod gl;
pub mod shader;
pub mod utils;

use gl::types::*;
use glfw::Context;
use image::{EncodableLayout, GenericImageView};
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
    let (shader, vao, texture1, texture2) = unsafe {
        // Configure global opengl state
        // -------------------
        gl::Enable(gl::DEPTH_TEST);
        // Build and compile our shader program
        // -------------------
        let shader = Shader::new("src/shaders/vertex.glsl", "src/shaders/fragment.glsl");

        #[rustfmt::skip]
        let mut vertices: Vec<f32> = vec![
    -0.5, -0.5, -0.5,  0.0, 0.0,
     0.5, -0.5, -0.5,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5,  0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 0.0,

    -0.5, -0.5,  0.5,  0.0, 0.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 1.0,
    -0.5,  0.5,  0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,

    -0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5,  0.5,  1.0, 0.0,

     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5,  0.5,  0.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 0.0,

    -0.5, -0.5, -0.5,  0.0, 1.0,
     0.5, -0.5, -0.5,  1.0, 1.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
     0.5, -0.5,  0.5,  1.0, 0.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,

    -0.5,  0.5, -0.5,  0.0, 1.0,
     0.5,  0.5, -0.5,  1.0, 1.0,
     0.5,  0.5,  0.5,  1.0, 0.0,
     0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5, -0.5,  0.0, 1.0
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
            5 * std::mem::size_of::<f32>() as GLsizei,
            0 as *const c_void,
        );
        gl::EnableVertexAttribArray(0);
        // gl::VertexAttribPointer(
        //     1,
        //     3,
        //     gl::FLOAT,
        //     gl::FALSE,
        //     6 * std::mem::size_of::<f32>() as GLsizei,
        //     (std::mem::size_of::<GLfloat>() * 3) as *const c_void,
        // );
        // gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * std::mem::size_of::<f32>() as GLsizei,
            (std::mem::size_of::<GLfloat>() * 3) as *const c_void,
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

        let img = image::open("assets/container.jpg").unwrap();
        let mut texture1: GLuint = 0;
        let mut texture2: GLuint = 0;
        gl::GenTextures(1, &mut texture1);
        gl::BindTexture(gl::TEXTURE_2D, texture1);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // Load the texture

        // Texture 1
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
        gl::BindTexture(gl::TEXTURE_2D, texture2);
        // gl::ActiveTexture(gl::TEXTURE1);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img.into_bytes().as_bytes().as_ptr() as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
        // -------------------------------------------------------------------------------------------
        shader.use_shader();
        shader.set_int("texture1", 0);
        shader.set_int("texture2", 1);

        (shader, vao, texture1, texture2)
    };

    let mut view = glm::Mat4::identity();
    view = glm::translate(&view, &glm::vec3(0., 0., -3.));

    let mut projection = glm::perspective(f32::to_radians(45.), 800. / 600., 0.1, 100.);

    let cube_position: Vec<glm::Vec3> = vec![
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5),
    ];

    while !window.should_close() {
        // Input
        process_input(&mut window);

        let mut model = glm::Mat4::identity();
        model = glm::rotate(
            &model,
            glfw.get_time() as f32 * f32::to_radians(50.),
            &glm::vec3(0.5, 1.0, 0.0),
        );

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            // Rendering code
            shader.use_shader();

            // shader.set_mat4("transform", &trans);
            shader.set_mat4("model", &model);
            shader.set_mat4("view", &view);
            shader.set_mat4("projection", &projection);

            gl::BindVertexArray(vao);
            for i in 0..cube_position.len() {
                let mut model = glm::Mat4::identity();
                model = glm::translate(&model, &cube_position[i]);
                let angle = 20. * i as f32;
                model = glm::rotate(&model, f32::to_radians(angle), &glm::vec3(1., 0.3, 0.5));
                shader.set_mat4("model", &model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        // Check and call events and swap the buffers
        window.swap_buffers();
        glfw.poll_events();
    }
}
