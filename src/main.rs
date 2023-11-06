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

const SRC_WIDTH: u32 = 800;
const SRC_HEIGHT: u32 = 600;

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

    let lighting_shader = Shader::new("src/shaders/vertex.glsl", "src/shaders/fragment.glsl");
    let lightcube_shader = Shader::new(
        "src/shaders/vertex_light_cube.glsl",
        "src/shaders/fragment_light_cube.glsl",
    );

    #[rustfmt::skip]
        let vertices: &[f32] = &[
        // positions       // normals        // texture coords
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
        0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0,
        0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
        0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
        0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0,
        0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
        0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,

        0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
        0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0,
        0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
        0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
        0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0,
        0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
        0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0,  1.0,
        0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
        0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
        0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0,  1.0,
        0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
        0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0
    ];

    unsafe {
        // Configure global opengl state
        gl::Enable(gl::DEPTH_TEST);

        lighting_shader.set_float("material.shininess", 32.0);

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

        // Models
        // -------

        let our_model = Model::new("assets/models/backpack.obj");

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
            lighting_shader.use_shader();
            lighting_shader.set_vec3_g("viewPos", &camera.position);
            // --- Directional light
            lighting_shader.set_vec3_f("dirLight.direction", 0.5, -1.0, 0.5);
            lighting_shader.set_vec3_f("dirLight.ambient", 0.5, 0.5, 0.5);
            lighting_shader.set_vec3_f("dirLight.specular", 0.5, 0.5, 0.5);
            lighting_shader.set_vec3_f("dirLight.diffuse", 0.5, 0.5, 0.5);

            // View/projection transformations
            let projection = glm::perspective(
                SRC_WIDTH as f32 / SRC_HEIGHT as f32,
                f32::to_radians(camera.zoom),
                0.1,
                100.,
            );
            let view = camera.get_view_matrix();
            lighting_shader.set_mat4("projection", &projection);
            lighting_shader.set_mat4("view", &view);
            // World transformation
            let model = glm::Mat4::identity();
            lighting_shader.set_mat4("model", &model);

            let model = glm::Mat4::identity();
            lighting_shader.set_mat4("model", &model);
            our_model.draw(&lighting_shader);

            // Light cubes
            window.swap_buffers();
            glfw.poll_events();
        }
        for mesh in our_model.meshes.iter() {
            unsafe {
                gl::DeleteVertexArrays(1, &mesh.vao);
                gl::DeleteBuffers(1, &mesh.vbo);
                gl::DeleteBuffers(1, &mesh.ebo);
            }
        }
    }
}
