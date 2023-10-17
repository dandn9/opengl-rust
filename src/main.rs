extern crate core;
use core::ffi::{c_int, c_void};

#[link(name = "glfw3", kind = "static")]
#[link(name = "shell32")]
#[link(name = "gdi32")]
extern "C" {
    fn glfwInit() -> c_void;
}

fn main() {
    println!("[rust] start");
    unsafe {
        glfwInit();
    };
}
