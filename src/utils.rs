use crate::{camera, gl, Camera};

pub fn to_c_str(str: &str) -> std::ffi::CString {
    std::ffi::CString::new(str.as_bytes()).unwrap()
}

pub fn process_input(window: &mut glfw::Window, camera: &mut Camera, delta_time: f32) {
    if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
        window.set_should_close(true)
    }
    if window.get_key(glfw::Key::W) == glfw::Action::Press {
        camera.process_keyboard(camera::CameraMovement::FORWARD, delta_time)
    }
    if window.get_key(glfw::Key::S) == glfw::Action::Press {
        camera.process_keyboard(camera::CameraMovement::BACKWARD, delta_time)
    }
    if window.get_key(glfw::Key::A) == glfw::Action::Press {
        camera.process_keyboard(camera::CameraMovement::LEFT, delta_time)
    }
    if window.get_key(glfw::Key::D) == glfw::Action::Press {
        camera.process_keyboard(camera::CameraMovement::RIGHT, delta_time)
    }
}

pub fn process_mouse(
    event: glfw::WindowEvent,
    camera: &mut Camera,
    first_mouse: &mut bool,
    last_x: &mut f32,
    last_y: &mut f32,
) {
    match event {
        glfw::WindowEvent::CursorPos(xpos, ypos) => {
            if *first_mouse {
                *last_x = xpos as f32;
                *last_y = ypos as f32;
                *first_mouse = false;
            }
            let x_offset = xpos as f32 - *last_x;
            let y_offset = *last_y - ypos as f32; // Reversed coordinates since y go from bottom to top

            *last_x = xpos as f32;
            *last_y = ypos as f32;

            camera.process_mouse_movement(x_offset, y_offset, None);
        }
        glfw::WindowEvent::Scroll(_, yoffset) => camera.process_mouse_scroll(yoffset as f32),

        _ => {}
    }
}

pub fn framebuffer_size_callback(width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}
