use std::ffi;

extern "C" {
    pub fn blur_hash_for_file(
        x_components: ffi::c_int,
        y_components: ffi::c_int,
        filename: *const ffi::c_char,
    );
}

fn main() {}
