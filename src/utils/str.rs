pub fn name_to_ptr(name: &str) -> *const i8 {
    return append_null(name).as_ptr() as *const i8;
}

pub fn append_null(name: &str) -> String {
    return format!("{name}\0");
}