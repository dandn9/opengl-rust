#[allow(clippy::all)]
pub mod gl;

#[link(name = "glfw3", kind = "static")]
#[link(name = "shell32")]
#[link(name = "gdi32")]

include!(concat!("./bindings.rs"));
