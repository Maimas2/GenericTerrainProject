// Snippets taken from https://github.com/rust-windowing/gl-rs/blob/master/gl/examples/triangle.rs

extern crate gl;
extern crate glfw;
extern crate gl_loader;

use glfw::{Action, Context, Key::{self, X}};
use gl::types::*;
use std::{f32::consts::PI, ffi::CString};

pub mod terrain_gen;

struct ButtonsHeld {
    w: bool,
    a: bool,
    s: bool,
    d: bool,
}

// Represents a facing direction in 3d space using the theta-phi orientation you learned in 259
struct ThetaPhi {
    theta: f32,
    phi: f32,
}
impl ThetaPhi {
    pub fn to_glm_vec3(&self) -> glm::Vector3<f32> {
        glm::Vector3 { x: self.theta.cos()*self.phi.sin(), y: self.theta.sin()*self.phi.sin(), z: self.phi.cos() }
    }
}

const CAMERA_SPEED: f32 = 0.0004;

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

fn handle_movement(camera_position: &mut glm::Vector3<f32>, camera_direction: &ThetaPhi, current_buttons_held: &ButtonsHeld) {
    if current_buttons_held.w {
        *camera_position = *camera_position + (*camera_direction).to_glm_vec3() * CAMERA_SPEED;
    }
    if current_buttons_held.s {
        *camera_position = *camera_position - (*camera_direction).to_glm_vec3() * CAMERA_SPEED;
    }
    if current_buttons_held.d {
        *camera_position = *camera_position + glm::cross((*camera_direction).to_glm_vec3(), glm::Vector3::new(0., 1., 0.)) * CAMERA_SPEED;
    }
    if current_buttons_held.a {
        *camera_position = *camera_position + glm::cross((*camera_direction).to_glm_vec3(), glm::Vector3::new(0., 1., 0.)) * CAMERA_SPEED;
    }
}

fn main() {
    let mut camera_position: glm::Vector3<f32> = glm::vec3(-2., 0., 0.);
    let mut camera_direction: ThetaPhi = ThetaPhi { theta: 0., phi: 0.5*PI };

    let mut last_mouse_pos: [f32; 2] = [0., 0.];

    let mut current_buttons_held: ButtonsHeld = ButtonsHeld { w: false, a: false, s: false, d: false };

    let vs_src: String = std::fs::read_to_string("./data/shaders/vs.glsl").unwrap();
    let fs_src: String = std::fs::read_to_string("./data/shaders/fs.glsl").unwrap();

    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(glfw::WindowHint::Samples(Some(4)));
    glfw.window_hint(glfw::WindowHint::Maximized(true));

    let (mut window, events) = glfw.create_window(600, 600, "Yay GLFW", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    window.set_cursor_pos_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    gl_loader::init_gl();

    gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);

    glfw.set_swap_interval(glfw::SwapInterval::None);

    let vertex_shader = compile_shader(&vs_src.into_boxed_str().to_owned(), gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(&fs_src.into_boxed_str().to_owned(), gl::FRAGMENT_SHADER);
    let program = link_program(vertex_shader, fragment_shader);

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    let gend_mesh: Vec<f32> = terrain_gen::generate_test_mesh();

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (size_of::<f32>() * gend_mesh.len()) as GLsizeiptr, gend_mesh.as_ptr().cast(), gl::STATIC_DRAW);

        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size_of_val(&VERTICES) as isize, VERTICES.as_ptr().cast(), gl::STATIC_DRAW);

        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());
        
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE as GLboolean, 6 * size_of::<GLfloat>() as i32,
            std::ptr::null::<GLvoid>());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE as GLboolean, 6 * size_of::<GLfloat>() as i32,
            (3 * size_of::<GLfloat>()) as *const GLvoid);
        gl::EnableVertexAttribArray(1);

        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    while !window.should_close() {
        let (wwidth, wheight) = window.get_framebuffer_size();

        let current_time = glfw.get_time() as f32;

        unsafe { // Update opengl and transformation matrices
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Viewport(0, 0, wwidth, wheight);

            gl::UseProgram(program);

            let position_index: i32 = gl::GetUniformLocation(program, CString::new("rotate").unwrap().as_ptr());
            gl::Uniform1f(position_index, current_time);

            let perspective_matrix: glm::Matrix4<f32> = glm::ext::perspective(glm::radians(90.), (wwidth as f32) / (wheight as f32), 0.1, 100.0);

            let perspective_atr: i32 = gl::GetUniformLocation(program, CString::new("perspective").unwrap().as_ptr());
            gl::UniformMatrix4fv(perspective_atr, 1, gl::FALSE, perspective_matrix.as_array().as_ptr().cast());

            let view_matrix: glm::Matrix4<f32> = glm::ext::look_at(
                camera_position,
                camera_position + camera_direction.to_glm_vec3(),
                glm::Vector3 { x: 0., y: 1., z: 0. }
            );

            let view_atr: i32 = gl::GetUniformLocation(program, CString::new("view").unwrap().as_ptr());
            gl::UniformMatrix4fv(view_atr, 1, gl::FALSE, view_matrix.as_array().as_ptr().cast());

            // transform_matrix held the rotation over time previous, but now is unneeded and so remains identity
            let transform_matrix: glm::Matrix4<f32> = glm::Matrix4::new(
                glm::Vector4::new(1., 0., 0., 0.),
                glm::Vector4::new(0., 1., 0., 0.),
                glm::Vector4::new(0., 0., 1., 0.),
                glm::Vector4::new(0., 0., 0., 1.)
            );
            let transform_matrix = glm::ext::rotate(&transform_matrix, 0., glm::Vector3::new(0., 0., 1.));

            let final_transform_atr: i32 = gl::GetUniformLocation(program, CString::new("final_transform").unwrap().as_ptr());
            gl::UniformMatrix4fv(final_transform_atr, 1, gl::FALSE, transform_matrix.as_array().as_ptr().cast());

            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut current_buttons_held, &mut camera_direction, &mut last_mouse_pos);
        }

        handle_movement(&mut camera_position, &camera_direction, &current_buttons_held);

        // println!("{}", 1.0/(glfw.get_time() - lastTime));
        // last_time = glfw.get_time();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, current_buttons_held: &mut ButtonsHeld, camera_direction: &mut ThetaPhi, last_mouse_pos: &mut [f32; 2]) {
    if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
        window.set_should_close(true);
    }

    if let glfw::WindowEvent::Key(_key, _scancode, action, _modifiers) = event {
        if _key == glfw::Key::W  {
            current_buttons_held.w = action != Action::Release;
        } else if _key == glfw::Key::A {
            current_buttons_held.a = action != Action::Release;
        } else if _key == glfw::Key::S {
            current_buttons_held.s = action != Action::Release;
        } else if _key == glfw::Key::D {
            current_buttons_held.d = action != Action::Release;
        }
    }

    if let glfw::WindowEvent::CursorPos(xposs, yposs) = event {
        let xpos: f32 = xposs as f32;
        let ypos: f32 = yposs as f32;
        
        if last_mouse_pos[0] == 0. && last_mouse_pos[1] == 0. {
            *last_mouse_pos = [xpos, ypos];
        } else {
            let dx = last_mouse_pos[0] - xpos;
            let dy = last_mouse_pos[1] - ypos;

            camera_direction.theta += dy * 0.001;
            camera_direction.phi += dx * 0.001;
            
            camera_direction.theta += 2.*PI;
            camera_direction.phi += PI;

            camera_direction.theta %= 2.*PI;
            camera_direction.phi %= PI;

            *last_mouse_pos = [xpos, ypos];
        }
    }
}