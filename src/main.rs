extern crate gl;
extern crate nalgebra_glm as glm;
extern crate russimp;

mod camera;
pub mod macros;
mod mesh;
mod model;
pub mod shader;
pub mod utils;

use crate::model::Model;
use camera::Camera;
use gl::types::*;
use glfw::Context;
use image::EncodableLayout;
use std::mem::{size_of, size_of_val};
use std::ops::Mul;
use std::os::raw::c_void;
use utils::{framebuffer_size_callback, process_input, process_mouse};

use crate::shader::Shader;
use crate::utils::{load_texture, ToCVoid};

const SRC_WIDTH: u32 = 1280;
const SRC_HEIGHT: u32 = 720;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    let monitor = glfw::Monitor::from_primary();
    let (scale_x, scale_y) = monitor.get_content_scale();
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(
            SRC_WIDTH * scale_x as u32,
            SRC_HEIGHT * scale_y as u32,
            "LearnOpenGL-Rust",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create Glfw window");

    glfw.make_context_current(Some(&window));
    // TODO: Make this a polled event
    window.set_framebuffer_size_callback(framebuffer_size_callback);
    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    gl::load_with(|symbol| glfw.get_proc_address_raw(symbol));
    window.make_current();

    let shader = Shader::new("src/shaders/vertex.glsl", "src/shaders/fragment.glsl");

    #[rustfmt::skip]
        let plane_vertices: &[f32] = &[
        // positions          // texture Coords (note we set these higher than 1 (together with GL_REPEAT as texture wrapping mode). this will cause the floor texture to repeat)
        5.0, -0.5,  5.0,  2.0, 0.0,
        -5.0, -0.5,  5.0,  0.0, 0.0,
        -5.0, -0.5, -5.0,  0.0, 2.0,

        5.0, -0.5,  5.0,  2.0, 0.0,
        -5.0, -0.5, -5.0,  0.0, 2.0,
        5.0, -0.5, -5.0,  2.0, 2.0
    ];
    #[rustfmt::skip]
        let cube_vertices: &[f32] = &[
// positions          // texture Coords
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
    let mut cubeVAO: u32 = 0;
    let mut cubeVBO: u32 = 0;
    let mut planeVAO: u32 = 0;
    let mut planeVBO: u32 = 0;

    unsafe {
        // Configure global opengl state
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);

        gl::GenVertexArrays(1, &mut cubeVAO);
        gl::GenBuffers(1, &mut cubeVBO);
        gl::BindVertexArray(cubeVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, cubeVBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(cube_vertices) as GLsizeiptr,
            cube_vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (5 * size_of::<f32>()) as GLsizei,
            ToCVoid(0).into(),
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * size_of::<f32>()) as GLsizei,
            ToCVoid(3 * size_of::<f32>()).into(),
        );

        gl::GenVertexArrays(1, &mut planeVAO);
        gl::GenBuffers(1, &mut planeVBO);
        gl::BindVertexArray(planeVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, planeVBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(plane_vertices) as GLsizeiptr,
            plane_vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (5 * size_of::<f32>()) as GLsizei,
            ToCVoid(0).into(),
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * size_of::<f32>()) as GLsizei,
            ToCVoid(3 * size_of::<f32>()).into(),
        );
    }

    // Load textures
    let cube_texture = load_texture("assets/textures/marble.jpg");
    let floor_texture = load_texture("assets/textures/metal.png");

    // Shader config
    // -------
    shader.use_shader();
    shader.set_int("texture1", 0);

    // Camera
    // ------------------
    let mut camera = Camera {
        position: glm::Vec3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };
    let mut last_x: f32 = 0.0;
    let mut last_y: f32 = 0.0;
    let mut first_mouse = true;

    // Time
    // -----------
    let mut last_frame: f32 = 0.0;
    let mut delta_time: f32 = 0.0;

    // Render loop
    // ---------------------------
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
        }
        shader.use_shader();
        // Model/View/projection
        let mut model = glm::Mat4::identity();
        let view = camera.get_view_matrix();
        let projection = glm::perspective(
            SRC_WIDTH as f32 / SRC_HEIGHT as f32,
            f32::to_radians(camera.zoom),
            0.1,
            100.,
        );
        shader.set_mat4("view", &view);
        shader.set_mat4("projection", &projection);

        unsafe {
            // Cubes
            gl::BindVertexArray(cubeVAO);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, cube_texture);
            model = glm::translate(&model, &glm::vec3(-1., 0., -1.));
            shader.set_mat4("model", &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            model = glm::identity();
            model = glm::translate(&model, &glm::vec3(2., 0., 0.));
            shader.set_mat4("model", &model);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            // Floor
            gl::BindVertexArray(planeVAO);
            gl::BindTexture(gl::TEXTURE_2D, floor_texture);
            shader.set_mat4("model", &glm::Mat4::identity());
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }
        window.swap_buffers();
        glfw.poll_events();
    }
    // for mesh in our_model.meshes.iter() {
    //     unsafe {
    //         gl::DeleteVertexArrays(1, &mesh.vao);
    //         gl::DeleteBuffers(1, &mesh.vbo);
    //         gl::DeleteBuffers(1, &mesh.ebo);
    //     }
    // }
    // }
}
