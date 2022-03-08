use image::io::Reader;

pub fn load(path: &str) -> (image::DynamicImage, i32, i32) {
    let img = match Reader::open(&std::path::Path::new(path)) {
        Ok(value) => value,
        Err(error) => {
            println!("Error reading image {}", path);
            println!("{:?}", error);
            std::process::exit(0);
        }
    };

    let img_data = match img.decode() {
        Ok(value) => value,
        Err(error) => {
            println!("Error reading image {}", path);
            println!("{:?}", error);
            std::process::exit(0);
        }
    };

    let width = img_data.width().try_into().unwrap();
    let height = img_data.height().try_into().unwrap();

    return (img_data, width, height);
}
