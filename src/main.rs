use glfw::Context;

pub mod gl;

fn framebuffer_size_callback(width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}

fn process_input(window: &mut glfw::Window) {
    if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
        window.set_should_close(true);
    }
}
fn main() {
    println!("[rust] start");
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    let (mut window, receiver) = glfw
        .create_window(800, 600, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create Glfw window");

    glfw.make_context_current(Some(&window));

    gl::load(|symbol| glfw.get_proc_address_raw(symbol));

    unsafe { gl::Viewport(0, 0, 800, 600) };
    window.set_framebuffer_size_callback(framebuffer_size_callback);

    // Render loop
    while !window.should_close() {
        // Input
        process_input(&mut window);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        // Check and call events and swap the buffers
        glfw.poll_events();
        window.swap_buffers();
    }
}
