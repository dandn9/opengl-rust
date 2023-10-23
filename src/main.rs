extern crate nalgebra_glm as glm;

mod camera;
pub mod gl;
pub mod shader;
pub mod utils;

use camera::Camera;
use gl::types::*;
use glfw::Context;
use image::{EncodableLayout, GenericImageView};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::os::raw::c_void;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use utils::{framebuffer_size_callback, process_input, process_mouse, to_c_str};

use crate::shader::Shader;

const SRC_WIDTH: u32 = 800;
const SRC_HEIGHT: u32 = 600;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    let (mut window, events) = glfw
        .create_window(800, 600, "LearnOpenGL-Rust", glfw::WindowMode::Windowed)
        .expect("Failed to create Glfw window");

    glfw.make_context_current(Some(&window));

    gl::load(|symbol| glfw.get_proc_address_raw(symbol));

    unsafe { gl::Viewport(0, 0, SRC_WIDTH as i32, SRC_HEIGHT as i32) };
    window.set_framebuffer_size_callback(framebuffer_size_callback);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.make_current();
    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_scroll_polling(true);

    // let vbo: *mut u32 = std::ptr::null();
    let mut camera = Camera::new(None, None, None, None);

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

    // Camera

    let mut last_frame: f32 = 0.0;
    let mut delta_time: f32 = 0.0;

    let mut last_x: f32 = 0.0;
    let mut last_y: f32 = 0.0;
    let mut first_mouse = true;

    while !window.should_close() {
        let time = glfw.get_time() as f32;

        delta_time = time - last_frame;
        last_frame = time;

        // Input
        // TODO: Make this a polling event, (just have to keep track of when it polled the PRESS event and when it polled the RELEASE event)
        process_input(&mut window, &mut camera, delta_time);
        for (_, event) in glfw::flush_messages(&events) {
            process_mouse(
                event,
                &mut camera,
                &mut first_mouse,
                &mut last_x,
                &mut last_y,
            );
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            // Rendering code
            shader.use_shader();

            let projection = glm::perspective(
                SRC_WIDTH as f32 / SRC_HEIGHT as f32,
                f32::to_radians(camera.zoom),
                0.1,
                100.,
            );
            let view = camera.get_view_matrix();
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
