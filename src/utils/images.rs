use std::fs::File;
use std::io::prelude::Read;

pub fn load(path: &str) -> (*mut u8, i32, i32) {
    let mut file = File::open(path).expect(&format!("Error loading file {}", path));
    let mut contents = vec![];
    file.read_to_end(&mut contents);

    let mut x = 0;
    let mut y = 0;
    let mut comp = 0;
    let img: *mut u8;

    unsafe {
        img = stb_image_rust::stbi_load_from_memory(
            contents.as_mut_ptr(),
            contents.len() as i32,
            &mut x,
            &mut y,
            &mut comp,
            stb_image_rust::STBI_rgb_alpha,
        );
    }
    return (img, x, y);
}
