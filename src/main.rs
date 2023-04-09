use std::env;
use std::ffi;

extern "C" {
    pub fn blur_hash_for_file(
        x_components: ffi::c_int,
        y_components: ffi::c_int,
        filename: *const ffi::c_char,
    ) -> *const ffi::c_char;
}

pub fn blur_hash_from_file(x_comp: i32, y_comp: i32, file_name: &str) -> String {
    unsafe {
        ffi::CStr::from_ptr(blur_hash_for_file(
            x_comp,
            y_comp,
            file_name.as_ptr() as *const ffi::c_char,
        ))
    }
    .to_str()
    .unwrap()
    .to_owned()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <x_comp> <y_comp> <file_name>", args[0]);
        return;
    }
    let x_comp: i32 = args[1].parse().unwrap();
    let y_comp: i32 = args[2].parse().unwrap();
    let file_name = &args[3];
    let blur_hash = blur_hash_from_file(x_comp, y_comp, file_name);

    println!("{blur_hash}");
}
