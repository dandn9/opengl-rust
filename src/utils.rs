use crate::gl::types::*;
use crate::{camera, gl, Camera};
use std::ffi::c_void;

// Helps with all the nasty casts
pub struct ToCVoid<T>(pub T);
impl Into<*const c_void> for ToCVoid<usize> {
    fn into(self) -> *const c_void {
        self.0 as *const c_void
    }
}
impl Into<*const c_void> for ToCVoid<&Vec<f32>> {
    fn into(self) -> *const c_void {
        self.0.as_ptr() as *const c_void
    }
}

// Does this behave weirdly because it gets deallocated since it gets consumed ????
// impl Into<*const c_void> for ToCVoid<Vec<f32>> {
//     fn into(self) -> *const c_void {
//         let b = self.0.as_ptr();
//         println!("INTO WITHOUT & {:?} - {:?}", b, b as *const c_void);
//         b as *const c_void
//     }
// }
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

pub fn load_texture(path: &str) -> u32 {
    unsafe {
        let mut texture_id = 0;
        gl::GenTextures(1, &mut texture_id);

        let mut image = image::open(path).unwrap();
        let data = image.as_bytes().as_ptr();
        let format = match image {
            image::DynamicImage::ImageLuma8(_) => gl::RED,
            image::DynamicImage::ImageLumaA8(_) => gl::RG,
            image::DynamicImage::ImageRgb8(_) => gl::RGB,
            image::DynamicImage::ImageRgba8(_) => gl::RGBA,
            _ => panic!("Unsupported format"),
        };
        let width = image.width();
        let height = image.height();

        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as GLint,
            width as GLsizei,
            height as GLsizei,
            0,
            format,
            gl::UNSIGNED_BYTE,
            data as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as GLint,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

        return texture_id;
    }
}
pub fn framebuffer_size_callback(width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}
