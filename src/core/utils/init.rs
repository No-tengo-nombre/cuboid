use beryllium::{
    SDL,
    GlWindow,
    GlProfile,
    SdlGlAttr,
    InitFlags,
    SwapInterval,
    WindowFlags,
    WindowPosition,    
};
use ogl33::{
    load_gl_with,
};


pub fn set_up_window(title: &str) -> (SDL, GlWindow) {
    let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");
    sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();
    #[cfg(target_os = "macos")]
    {
        sdl
        .gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
        .unwrap();
    }

    let win = sdl
        .create_gl_window(
            title,
            WindowPosition::Centered,
            800,
            600,
            WindowFlags::Shown,
        )
        .expect("couldn't make a window and context");
    win.set_swap_interval(SwapInterval::Vsync);
    return (sdl, win);
}


pub fn load_gl(window: &GlWindow) {
    unsafe {
        load_gl_with(|f_name| window.get_proc_address(f_name));
    }
}