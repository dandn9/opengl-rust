extern crate nalgebra_glm as glm;

mod camera;
pub mod gl;
pub mod shader;
pub mod utils;

use std::ops::Mul;
use camera::Camera;
use gl::types::*;
use glfw::Context;
use image::EncodableLayout;
use std::os::raw::c_void;
use glfw::ffi::glfwGetPrimaryMonitor;
use utils::{framebuffer_size_callback, process_input, process_mouse};

use crate::shader::Shader;

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
        .create_window(SRC_WIDTH * scale_x as u32, SRC_HEIGHT * scale_y as u32, "LearnOpenGL-Rust", glfw::WindowMode::Windowed)
        .expect("Failed to create Glfw window");

    glfw.make_context_current(Some(&window));

    gl::load(|symbol| glfw.get_proc_address_raw(symbol));

    unsafe { gl::Viewport(0, 0, (SRC_WIDTH as f32 * scale_x) as i32, (SRC_HEIGHT as f32 * scale_y) as i32) };
    window.set_framebuffer_size_callback(framebuffer_size_callback);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.make_current();
    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_scroll_polling(true);

    let mut camera = Camera::new(None, None, None, None);

    // Create shaders
    let (shader, vao, light_vao, light_shader) = unsafe {
        // Configure global opengl state
        // -------------------
        gl::Enable(gl::DEPTH_TEST);
        // Build and compile our shader program
        // -------------------
        let shader = Shader::new("src/shaders/vertex.glsl", "src/shaders/fragment.glsl");
        let light_shader = Shader::new(
            "src/shaders/vertex_light.glsl",
            "src/shaders/fragment_light.glsl",
        );

        #[rustfmt::skip]
            let mut vertices: Vec<f32> = vec![
  -0.5 , -0.5 , -0.5 ,  0.0 ,  0.0 , -1.0 ,
     0.5 , -0.5 , -0.5 ,  0.0 ,  0.0 , -1.0 ,
     0.5 ,  0.5 , -0.5 ,  0.0 ,  0.0 , -1.0 ,
     0.5 ,  0.5 , -0.5 ,  0.0 ,  0.0 , -1.0 ,
    -0.5 ,  0.5 , -0.5 ,  0.0 ,  0.0 , -1.0 ,
    -0.5 , -0.5 , -0.5 ,  0.0 ,  0.0 , -1.0 ,

    -0.5 , -0.5 ,  0.5 ,  0.0 ,  0.0 , 1.0 ,
     0.5 , -0.5 ,  0.5 ,  0.0 ,  0.0 , 1.0 ,
     0.5 ,  0.5 ,  0.5 ,  0.0 ,  0.0 , 1.0 ,
     0.5 ,  0.5 ,  0.5 ,  0.0 ,  0.0 , 1.0 ,
    -0.5 ,  0.5 ,  0.5 ,  0.0 ,  0.0 , 1.0 ,
    -0.5 , -0.5 ,  0.5 ,  0.0 ,  0.0 , 1.0 ,

    -0.5 ,  0.5 ,  0.5 , -1.0 ,  0.0 ,  0.0 ,
    -0.5 ,  0.5 , -0.5 , -1.0 ,  0.0 ,  0.0 ,
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
    -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,
    -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,

     0.5,  0.5,  0.5,  1.0,  0.0,  0.0,
     0.5,  0.5, -0.5,  1.0,  0.0,  0.0,
     0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
     0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
     0.5, -0.5,  0.5,  1.0,  0.0,  0.0,
     0.5,  0.5,  0.5,  1.0,  0.0,  0.0,

    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
     0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
     0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
     0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
    -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,

    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
     0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
     0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
     0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
    -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0];
        let mut indices: Vec<u32> = vec![
            0, 1, 3, // First triangle
            3, 1, 2, // Second triangle
        ];

        // ebo = ElementBufferObject, vao = VertexAttributeObject, vbo = VertexBufferObject
        let (mut vao, mut vbo, mut ebo): (GLuint, GLuint, GLuint) = (0, 0, 0);

        // Light vao
        let mut light_vao: GLuint = 0;

        gl::GenVertexArrays(1, &mut light_vao);
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
            (3 * std::mem::size_of::<f32>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(indices.as_slice()) as isize,
            indices.as_mut_slice().as_mut_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // note that this is allowed, the call to glVertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind

        gl::BindVertexArray(light_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, light_vao);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * std::mem::size_of::<f32>() as GLsizei,
            0 as *const c_void,
        );
        gl::EnableVertexAttribArray(0);
        gl::BindVertexArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::BindVertexArray(0);

        shader.use_shader();

        // // Wireframe mode
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shader, vao, light_vao, light_shader)
    };

    let mut projection = glm::perspective(f32::to_radians(45.), 800. / 600., 0.1, 100.);
    let light_pos = glm::vec3(1.2, 1.0, 2.0);

    let cube_position: Vec<glm::Vec3> = vec![glm::vec3(0.0, 0.0, 0.0)];

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
                shader.set_vec3_f("objectColor", 1.0, 0.5, 0.31);
                shader.set_vec3_f("lightColor", 1.0, 1.0, 1.0);
                shader.set_vec3_g("viewPos", &camera.position);
                // Light struct
                let mut light_color = glm::Vec3::identity();
                light_color.x = f32::sin(glfw.get_time() as f32 * 2.0);
                light_color.y = f32::sin(glfw.get_time() as f32 * 1.4);
                light_color.z = f32::sin(glfw.get_time() as f32 * 0.7);

                let light_ambient = glm::Vec3::from_element(0.1).component_mul(&light_color);
                let light_diffuse = glm::Vec3::from_element(0.5).component_mul(&light_color);
                let light_specular = glm::Vec3::from_element(1.0).component_mul(&light_color);

                shader.set_vec3_g("light.ambient", &light_ambient);
                shader.set_vec3_g("light.diffuse", &light_diffuse);
                shader.set_vec3_g("light.specular", &light_specular);
                shader.set_vec3_g("light.position", &light_pos);


                shader.set_vec3_f("material.ambient", 1.0, 0.5, 0.31);
                shader.set_vec3_f("material.diffuse", 1.0, 0.5, 0.31);
                shader.set_vec3_f("material.specular", 0.5, 0.5, 0.5);
                shader.set_float("material.shininess", 32.);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            light_shader.use_shader();
            light_shader.set_mat4("view", &view);
            light_shader.set_mat4("projection", &projection);
            let mut model = glm::Mat4::identity();
            model = glm::translate(&model, &light_pos);
            model = glm::scale(&model, &glm::Vec3::from_element(0.2));
            light_shader.set_mat4("model", &model);
            gl::BindVertexArray(light_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        // Check and call events and swap the buffers
        window.swap_buffers();
        glfw.poll_events();
    }
}
