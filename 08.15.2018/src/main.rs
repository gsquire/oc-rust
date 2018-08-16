extern crate libc;

use std::ffi::CStr;
use std::ptr;

use libc::c_char;

// We can include this in a separate module if we wanted to.
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        // The usage per the documentation is to first get the length of the path.
        let len = self::wai_getExecutablePath(ptr::null_mut(), 0, ptr::null_mut());

        // Allocate a vector to hold the path.
        let mut path: Vec<i8> = vec![0; (len + 1) as usize];
        let res =
            self::wai_getExecutablePath(path.as_mut_ptr() as *mut c_char, len, ptr::null_mut());

        // If our result is -1, then bail early.
        if res == -1 {
            println!("error getting executable path");
            return;
        }

        // Add a NULL terminator.
        path.push('\0' as i8);

        // Convert our buffer to a `CStr` and print it out.
        println!("path: {}", CStr::from_ptr(path.as_ptr()).to_str().unwrap());
    }
}
