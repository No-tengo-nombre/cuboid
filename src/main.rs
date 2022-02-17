mod core;

use beryllium::*;
use ogl33::*;

use crate::core::buffers;
use crate::core::utils::{
    types,
    init,
};


const WINDOW_TITLE: &str = "Triangle: Draw Arrays";

const VERTICES: [types::Vertex; 3] =
    [
        [-0.5, -0.5, 0.0],
        [0.5, -0.5, 0.0],
        [0.0, 0.5, 0.0],
    ];

const VERT_SHADER: &str = r#"#version 330 core
    layout (location = 0) in vec3 pos;
    void main() {
        gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
    out vec4 final_color;
    void main() {
        final_color = vec4(1.0, 0.5, 0.2, 1.0);
    }
"#;


// fn set_up_window() -> (beryllium::SDL, GlWindow) {
//     let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");
//     sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
//     sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
//     sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();
//     #[cfg(target_os = "macos")]
//     {
//         sdl
//         .gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
//         .unwrap();
//     }

//     let win = sdl
//         .create_gl_window(
//             WINDOW_TITLE,
//             WindowPosition::Centered,
//             800,
//             600,
//             WindowFlags::Shown,
//         )
//         .expect("couldn't make a window and context");
//     win.set_swap_interval(SwapInterval::Vsync);
//     return (sdl, win);
// }


// fn load_gl(window: &GlWindow) {
//     unsafe {
//         load_gl_with(|f_name| window.get_proc_address(f_name));
//     }
// }


fn main() {
    let (sdl, win) = init::set_up_window(WINDOW_TITLE);
    init::load_gl(&win);

    let vao = buffers::vao::new();
    let vbo = buffers::vbo::new(&VERTICES);
    vao.link_vbo(vbo, 0);
    
    unsafe {
        glClearColor(0.2, 0.3, 0.3, 1.0);
            
        let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
        glShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
        );
        glCompileShader(vertex_shader);
        let mut success = 0;
        glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetShaderInfoLog(
                vertex_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }

        let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);
        glShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
        );
        glCompileShader(fragment_shader);
        let mut success = 0;
        glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetShaderInfoLog(
                fragment_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }

        let shader_program = glCreateProgram();
        assert_ne!(shader_program, 0);
        glAttachShader(shader_program, vertex_shader);
        glAttachShader(shader_program, fragment_shader);
        glLinkProgram(shader_program);
        let mut success = 0;
        glGetProgramiv(shader_program, GL_LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetProgramInfoLog(
                shader_program,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }
        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);

        glUseProgram(shader_program);
    }

    'main_loop: loop {
        // handle events this frame
        while let Some(event) = sdl.poll_events().and_then(Result::ok) {
        match event {
            Event::Quit(_) => break 'main_loop,
            _ => (),
        }
        }
        // now the events are clear.

        // here's where we could change the world state if we had some.

        // and then draw!
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
            glDrawArrays(GL_TRIANGLES, 0, 3);
        }
        win.swap_window();
    }
}