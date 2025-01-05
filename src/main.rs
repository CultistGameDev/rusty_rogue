extern crate gl;
extern crate glfw;

use std::ffi::{c_void, CString};

use glfw::{Action, Context, Key};

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330
layout (location = 0) in vec3 aPos;
void main() {
    gl_Position = vec4(aPos, 1.0);
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330
out vec4 FragColor;
void main() {
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
"#;

fn main() {
    let window_width = 800;
    let window_height = 600;
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw
        .create_window(
            window_width,
            window_height,
            "Rusty Rogue",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let vertices: Vec<f32> = vec![
        // top
        0.0, 0.5, 0.0, // bottom right
        0.5, -0.5, 0.0, // bottom left
        -0.5, -0.5, 0.0,
    ];
    let size =
        (std::mem::size_of::<Vec<f32>>() + vertices.len() * std::mem::size_of::<f32>()) as isize;

    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    let program: u32;
    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let vertex_source = CString::new(VERTEX_SHADER_SOURCE).unwrap();
        gl::ShaderSource(
            vertex_shader,
            1,
            &vertex_source.as_ptr() as *const *const i8,
            std::ptr::null(),
        );
        gl::CompileShader(vertex_shader);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let fragment_source = CString::new(FRAGMENT_SHADER_SOURCE).unwrap();
        gl::ShaderSource(
            fragment_shader,
            1,
            &fragment_source.as_ptr() as *const *const i8,
            std::ptr::null(),
        );
        gl::CompileShader(fragment_shader);

        program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        gl::GenVertexArrays(1, &mut vao as *mut u32);
        gl::GenBuffers(1, &mut vbo as *mut u32);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        gl::BindVertexArray(0);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::Viewport(0, 0, window_width as i32, window_height as i32);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }

        window.swap_buffers();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &vao as *const u32);
        gl::DeleteBuffers(1, &vbo as *const u32);
        gl::DeleteProgram(program);
    }
}
fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
