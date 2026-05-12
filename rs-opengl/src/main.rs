use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let mut m_height = 0;
    let mut m_width = 0;

    let (mut window, events) = glfw
        .with_primary_monitor(|glfw, monitors| {
            for monitor in monitors.iter() {
                let mode = monitor.get_video_mode().unwrap();
                m_height = mode.height;
                m_width = mode.width;
            }
            glfw.create_window(
                m_width,
                m_height,
                "Learn OpenGL",
                glfw::WindowMode::Windowed,
            )
        })
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| match window.get_proc_address(symbol) {
        Some(ptr) => ptr as *const _,
        None => std::ptr::null(),
    });

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        glfw.poll_events();
        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height)
        },
        _ => {}
    }
}
