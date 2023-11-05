extern crate gl;
extern crate nalgebra_glm as glm;

mod camera;
pub mod shader;
pub mod utils;

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
    let (mut VBO, mut cubeVAO): (GLuint, GLuint) = (0, 0);
    let mut lightCubeVAO: GLuint = 0;
    // Load the textures
    let diffuse_map = load_texture("assets/container2.png");
    let specular_map = load_texture("assets/container2_specular.png");

    // ---------------------- DATA
    let point_light_positions: &[glm::Vec3] = &[
        glm::vec3(0.7, 0.2, 2.0),
        glm::vec3(2.3, -3.3, -4.0),
        glm::vec3(-4.0, 2.0, -12.0),
        glm::vec3(0.0, 0.0, -3.0),
    ];
    let cube_position: &[glm::Vec3] = &[
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
    println!(
        "{} - {} {}  - len {}",
        // size_of_val(vertices.as_slice()),
        size_of_val(&vertices),
        size_of_val(&vertices.as_ptr()),
        size_of_val(vertices.iter().as_ref()),
        vertices.len()
    );

    unsafe {
        // Configure global opengl state
        gl::Enable(gl::DEPTH_TEST);
        // Build and compile our shaders programs
        // First, configure the cube's vao and vbo
        gl::GenVertexArrays(1, &mut cubeVAO);
        gl::GenBuffers(1, &mut VBO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(vertices) as GLsizeiptr,
            ToCVoid(vertices).into(),
            gl::STATIC_DRAW,
        );
        gl::BindVertexArray(cubeVAO);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * size_of::<f32>()) as GLsizei,
            ToCVoid(0).into(),
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * size_of::<f32>()) as GLsizei,
            ToCVoid(3 * size_of::<GLfloat>()).into(),
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            (8 * size_of::<f32>()) as GLsizei,
            ToCVoid(6 * size_of::<GLfloat>()).into(),
        );
        gl::EnableVertexAttribArray(2);

        // Second, configure the light's vao (vbo stays the same since the vertices are the same)
        gl::GenVertexArrays(1, &mut lightCubeVAO);
        gl::BindVertexArray(lightCubeVAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * size_of::<f32>()) as GLsizei,
            ToCVoid(0).into(),
        );
        gl::EnableVertexAttribArray(0);
    };

    // Shader configuration
    // --------------------
    // Material properties
    lighting_shader.use_shader();
    lighting_shader.set_int("material.diffuse", 0);
    lighting_shader.set_int("material.specular", 1);
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

        // Directional light
        lighting_shader.set_vec3_f("dirLight.direction", -0.2, -1.0, -0.3);
        lighting_shader.set_vec3_f("dirLight.ambient", 0.05, 0.05, 0.05);
        lighting_shader.set_vec3_f("dirLight.diffuse", 0.4, 0.4, 0.4);
        lighting_shader.set_vec3_f("dirLight.specular", 0.5, 0.5, 0.5);
        // Point light 1
        lighting_shader.set_vec3_g("pointLights[0].position", &point_light_positions[0]);
        lighting_shader.set_vec3_f("pointLights[0].ambient", 0.05, 0.05, 0.05);
        lighting_shader.set_vec3_f("pointLights[0].diffuse", 0.8, 0.8, 0.8);
        lighting_shader.set_vec3_f("pointLights[0].specular", 1.0, 1.0, 1.0);
        lighting_shader.set_float("pointLights[0].constant", 1.0);
        lighting_shader.set_float("pointLights[0].linear", 0.09);
        lighting_shader.set_float("pointLights[0].quadratic", 0.032);
        // Point light 2
        lighting_shader.set_vec3_g("pointLights[1].position", &point_light_positions[1]);
        lighting_shader.set_vec3_f("pointLights[1].ambient", 0.05, 0.05, 0.05);
        lighting_shader.set_vec3_f("pointLights[1].diffuse", 0.8, 0.8, 0.8);
        lighting_shader.set_vec3_f("pointLights[1].specular", 1.0, 1.0, 1.0);
        lighting_shader.set_float("pointLights[1].constant", 1.0);
        lighting_shader.set_float("pointLights[1].linear", 0.09);
        lighting_shader.set_float("pointLights[1].quadratic", 0.032);
        // Point light 3
        lighting_shader.set_vec3_g("pointLights[2].position", &point_light_positions[2]);
        lighting_shader.set_vec3_f("pointLights[2].ambient", 0.05, 0.05, 0.05);
        lighting_shader.set_vec3_f("pointLights[2].diffuse", 0.8, 0.8, 0.8);
        lighting_shader.set_vec3_f("pointLights[2].specular", 1.0, 1.0, 1.0);
        lighting_shader.set_float("pointLights[2].constant", 1.0);
        lighting_shader.set_float("pointLights[2].linear", 0.09);
        lighting_shader.set_float("pointLights[2].quadratic", 0.032);
        // Point light 4
        lighting_shader.set_vec3_g("pointLights[3].position", &point_light_positions[3]);
        lighting_shader.set_vec3_f("pointLights[3].ambient", 0.05, 0.05, 0.05);
        lighting_shader.set_vec3_f("pointLights[3].diffuse", 0.8, 0.8, 0.8);
        lighting_shader.set_vec3_f("pointLights[3].specular", 1.0, 1.0, 1.0);
        lighting_shader.set_float("pointLights[3].constant", 1.0);
        lighting_shader.set_float("pointLights[3].linear", 0.09);
        lighting_shader.set_float("pointLights[3].quadratic", 0.032);

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

        // bind diffuse map
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, diffuse_map);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, specular_map);
        }
        // render containers
        unsafe {
            gl::BindVertexArray(cubeVAO);
        }
        for i in 0..cube_position.len() {
            let mut model = glm::Mat4::identity();
            model = glm::translate(&model, &cube_position[i]);
            let angle = 20. * i as f32;
            model = glm::rotate(&model, f32::to_radians(angle), &glm::vec3(1.0, 0.3, 0.5));
            lighting_shader.set_mat4("model", &model);

            unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 36) }
        }

        // Light cubes
        // -------------------
        lightcube_shader.use_shader();
        lightcube_shader.set_mat4("projection", &projection);
        lightcube_shader.set_mat4("view", &view);
        unsafe {
            gl::BindVertexArray(lightCubeVAO);
        }
        for position in point_light_positions {
            let mut model = glm::Mat4::identity();
            model = glm::translate(&model, &position);
            model = glm::scale(&model, &glm::Vec3::from_element(0.2));
            lightcube_shader.set_mat4("model", &model);
            unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 36) }
        }
        window.swap_buffers();
        glfw.poll_events();
    }
    unsafe {
        gl::DeleteVertexArrays(1, &cubeVAO);
        gl::DeleteVertexArrays(1, &lightCubeVAO);
        gl::DeleteBuffers(1, &VBO);
    }
}
