use image;
use image::io::Reader;
use std::fs::File;
use std::io::prelude::Read;

pub fn load(path: &str) -> (*mut u8, i32, i32) {
    let mut file = File::open(path).expect(&format!("Error loading file {}", path));
    let mut contents = vec![];
    match file.read_to_end(&mut contents) {
        Ok(_) => {}
        Err(error) => {
            println!("Error reading file {}", path);
            println!("{:?}", error);
        }
    }

    let mut x = 0;
    let mut y = 0;
    let mut comp = 0;
    let img: *mut u8;

    unsafe {
        stb_image_rust::stbi_set_flip_vertically_on_load(1);
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

// pub fn load(path: &str) -> (*mut u8, i32, i32) {
// // pub fn load(path: &str) -> (image::DynamicImage, i32, i32) {
//     let img = match Reader::open(&std::path::Path::new(path)) {
//         Ok(value) => value,
//         Err(error) => {
//             println!("Error reading image {}", path);
//             println!("{:?}", error);
//             std::process::exit(0);
//         }
//     };

//     let img_data = match img.decode() {
//         Ok(value) => value,
//         Err(error) => {
//             println!("Error reading image {}", path);
//             println!("{:?}", error);
//             std::process::exit(0);
//         },
//     };

//     // let mut img_data = img.to_rgba8();
//     let width = img_data.width().try_into().unwrap();
//     let height = img_data.height().try_into().unwrap();

//     return (
//         img_data.to_rgba8().as_mut_ptr(),
//         width,
//         height,
//     );
// }
