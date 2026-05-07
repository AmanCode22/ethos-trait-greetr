use core::ffi::c_char;
use core::slice;
use core::str;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn greet(name: *mut c_char) {
    if name.is_null() { return; }

    let mut len = 0;
    unsafe {
        while *name.offset(len) != 0 {
            len += 1;
        }

        let bytes = slice::from_raw_parts(name as *const u8, len as usize);

        if let Ok(name_str) = str::from_utf8(bytes) {
            println!("Hello, {}", name_str);
        }
    }
}
