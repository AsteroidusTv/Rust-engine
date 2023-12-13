use window::graphics::window::Window;

fn main() {
    let mut window = Window::new(1080, 720, "Hello Window!");
    window.init_gl();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.5, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.update();
    }
}

