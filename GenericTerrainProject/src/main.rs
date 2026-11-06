// Snippets taken from https://github.com/rust-windowing/gl-rs/blob/master/gl/examples/triangle.rs

extern crate gl;
extern crate glfw;
extern crate gl_loader;

use glfw::{Action, Context, Key::{self}};
use gl::types::*;
use std::{ffi::CString, os::raw::c_void};

static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];
const VERTICES: [GLuint; 6] = [0, 1, 2, 0, 1, 3];

static VS_SRC: &str = "
#version 150
in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}";

static FS_SRC: &str = "
#version 150
out vec4 out_color;

void main() {
    out_color = vec4(1.0, 1.0, 1.0, 1.0);
}";

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
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    gl_loader::init_gl();

    gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);

    let vertex_shader = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vertex_shader, fragment_shader);

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    unsafe {
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
        gl::VertexAttribPointer(pos_atr as GLuint, 2, gl::FLOAT, gl::FALSE as GLboolean, 0, std::ptr::null());

        gl::ClearColor(1.0, 0.0, 0.0, 1.0);
    }

    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::UseProgram(program);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
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