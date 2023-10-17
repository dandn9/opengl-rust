extern crate core;
use std::{ffi::CString, os::raw::c_char};

use tutorial::*;
/// Replacement for `ToCStr::with_c_str`
pub fn with_c_str<F, T>(s: &str, f: F) -> T
where
    F: FnOnce(*const c_char) -> T,
{
    let c_str = CString::new(s.as_bytes());
    f(c_str.unwrap().as_bytes_with_nul().as_ptr() as *const _)
}
pub type GLProc = GLFWglproc;
pub fn get_proc_address_raw(procname: &str) -> GLProc {
    with_c_str(procname, |procname| unsafe { glfwGetProcAddress(procname) })
}
fn main() {
    println!("[rust] start");
    unsafe {
        glfwInit();
        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR.try_into().unwrap(), 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR.try_into().unwrap(), 3);
        glfwWindowHint(
            GLFW_OPENGL_PROFILE.try_into().unwrap(),
            GLFW_OPENGL_CORE_PROFILE.try_into().unwrap(),
        );

        let c_str = std::ffi::CString::new("LearnOpengl").unwrap();
        let window = glfwCreateWindow(
            800,
            600,
            c_str.as_ptr() as *const i8,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        glfwMakeContextCurrent(window);

        if window.is_null() {
            println!("[rust] Failed to create GLFW window");
            glfwTerminate();
            return;
        }
        gl::load(|e| {
            println!("load {}", e);
            let a = get_proc_address_raw(e) as *const std::os::raw::c_void;
            println!("load {} {:?}", e, a);
            return a;
        });
        gl::Viewport(0, 0, 800, 600);

        // glViewport(0, 0, 800, 600);

        while glfwWindowShouldClose(window) == 0 {
            println!("LOOPING");
            glfwSwapBuffers(window);
            glfwPollEvents();
        }
        // glfwTerminate();
    };
}
