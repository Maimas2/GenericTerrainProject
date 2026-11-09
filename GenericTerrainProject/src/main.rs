// Snippets taken from https://github.com/rust-windowing/gl-rs/blob/master/gl/examples/triangle.rs

extern crate gl;
extern crate glfw;
extern crate gl_loader;

use glfw::{Action, Context, Key::{self}};
use gl::types::*;
use std::{f32::consts::PI, ffi::CString};

pub mod terrain_gen;

static VERTEX_DATA: [f32; 12] = [
    0.5, 0., 0.5,
    -0.5, 0., -0.5,
    0.5, 0., -0.5,
    -0.5, 0., 0.5,
];
const VERTICES: [GLuint; 6] = [0, 1, 2, 0, 1, 3];

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            let mut outlen = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            gl::GetShaderInfoLog(
                shader,
                len,
                &mut outlen, // Number of characters written excluding null terminator
                buf.as_mut_ptr() as *mut GLchar,
            );
            buf.set_len(outlen as usize);
            panic!(
                "{}",
                str::from_utf8(&buf).expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            let mut outlen = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            gl::GetProgramInfoLog(
                program,
                len,
                &mut outlen, // Number of characters written excluding null terminator
                buf.as_mut_ptr() as *mut GLchar,
            );
            buf.set_len(outlen as usize);
            panic!(
                "{}",
                str::from_utf8(&buf).expect("ProgramInfoLog not valid utf8")
            );
        }
        program
    }
}

fn main() {
    let vs_src: String = std::fs::read_to_string("./data/shaders/vs.glsl").unwrap();
    let fs_src: String = std::fs::read_to_string("./data/shaders/fs.glsl").unwrap();

    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

    let (mut window, events) = glfw.create_window(600, 600, "Yay GLFW", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    gl_loader::init_gl();

    gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);

    glfw.set_swap_interval(glfw::SwapInterval::None);

    let vertex_shader = compile_shader(&vs_src.into_boxed_str().to_owned(), gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(&fs_src.into_boxed_str().to_owned(), gl::FRAGMENT_SHADER);
    let program = link_program(vertex_shader, fragment_shader);

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    unsafe { // Returns the position index for `position` in the vertex shader
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, size_of_val(&VERTEX_DATA) as GLsizeiptr, VERTEX_DATA.as_ptr().cast(), gl::STATIC_DRAW);

        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size_of_val(&VERTICES) as isize, VERTICES.as_ptr().cast(), gl::STATIC_DRAW);

        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());
        
        let pos_atr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_atr as GLuint);
        gl::VertexAttribPointer(pos_atr as GLuint, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, std::ptr::null());

        gl::ClearColor(1.0, 0.0, 0.0, 1.0);
    }

    while !window.should_close() {
        let (wwidth, wheight) = window.get_framebuffer_size();

        let current_time = glfw.get_time() as f32;

        // let mut _temp_vec: Vec<f32> = vec![0., 0.5, 0.];

        // _temp_vec.as_ptr().cast::<f32>();

        unsafe { // Update opengl and transformation things
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Viewport(0, 0, wwidth, wheight);

            gl::UseProgram(program);

            let position_index: i32 = gl::GetUniformLocation(program, CString::new("rotate").unwrap().as_ptr());
            gl::Uniform1f(position_index, current_time);

            let perspective_matrix: glm::Matrix4<f32> = glm::ext::perspective(glm::radians(90.), 1., 0.1, 100.0);

            let perspective_atr: i32 = gl::GetUniformLocation(program, CString::new("perspective").unwrap().as_ptr());
            gl::UniformMatrix4fv(perspective_atr, 1, gl::FALSE, perspective_matrix.as_array().as_ptr().cast());

            let view_matrix: glm::Matrix4<f32> = glm::ext::look_at(
                glm::Vector3 { x: 4., y: 0., z: 0. },
                glm::Vector3 { x: 0., y: 0., z: 0. },
                glm::Vector3 { x: 0., y: 1., z: 0. }
            );

            let view_atr: i32 = gl::GetUniformLocation(program, CString::new("view").unwrap().as_ptr());
            gl::UniformMatrix4fv(view_atr, 1, gl::FALSE, view_matrix.as_array().as_ptr().cast());

            let transform_matrix: glm::Matrix4<f32> = glm::Matrix4::new(
                glm::Vector4::new(1., 0., 0., 0.),
                glm::Vector4::new(0., 1., 0., 0.),
                glm::Vector4::new(0., 0., 1., 0.),
                glm::Vector4::new(0., 0., 0., 1.)
            );
            let transform_matrix = glm::ext::rotate(&transform_matrix, current_time % (2.*PI), glm::Vector3::new(0., 0., 1.));

            let final_transform_atr: i32 = gl::GetUniformLocation(program, CString::new("final_transform").unwrap().as_ptr());
            gl::UniformMatrix4fv(final_transform_atr, 1, gl::FALSE, transform_matrix.as_array().as_ptr().cast());

            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // println!("{}", 1.0/(glfw.get_time() - lastTime));
        // last_time = glfw.get_time();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
        window.set_should_close(true);
    }

    if let glfw::WindowEvent::Key(_key, _scancode, action, _modifiers) = event
        && action == Action::Press {
            println!("{:?}", event)
        }
}