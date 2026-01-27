use std::ffi::CStr;
use std::os::raw::{c_char, c_uchar};
use std::ptr;
use std::slice;
extern crate libc;

use meika256_lib::{
    encrypt_file,
    decrypt_file,
    encrypt_stream,
    decrypt_stream,
};

/// Error codes
pub const MEIKA_OK: i32 = 0;
pub const MEIKA_INVALID_INPUT: i32 = 1;
pub const MEIKA_CRYPTO_ERROR: i32 = 2;
pub const MEIKA_IO_ERROR: i32 = 3;


/// ==========================
/// BUFFER MODE
/// ==========================

#[no_mangle]
pub extern "C" fn meika_encrypt_buffer(
    data: *const c_uchar,
    len: usize,
    password: *const c_char,
    out_ptr: *mut *mut c_uchar,
    out_len: *mut usize,
) -> i32 {
    if data.is_null() || password.is_null() || out_ptr.is_null() || out_len.is_null() {
        return MEIKA_INVALID_INPUT;
    }

    let input = unsafe { slice::from_raw_parts(data, len) };
    let password = unsafe { CStr::from_ptr(password) }
        .to_string_lossy()
        .to_string();

    let encrypted = match encrypt_file(input, &password) {
        Ok(v) => v,
        Err(_) => return MEIKA_CRYPTO_ERROR,
    };

    unsafe {
        *out_len = encrypted.len();
        *out_ptr = libc::malloc(encrypted.len()) as *mut c_uchar;
        if (*out_ptr).is_null() {
            return MEIKA_IO_ERROR;
        }
        ptr::copy_nonoverlapping(encrypted.as_ptr(), *out_ptr, encrypted.len());
    }

    MEIKA_OK
}

#[no_mangle]
pub extern "C" fn meika_decrypt_buffer(
    data: *const c_uchar,
    len: usize,
    password: *const c_char,
    out_ptr: *mut *mut c_uchar,
    out_len: *mut usize,
) -> i32 {
    if data.is_null() || password.is_null() || out_ptr.is_null() || out_len.is_null() {
        return MEIKA_INVALID_INPUT;
    }

    let input = unsafe { slice::from_raw_parts(data, len) };
    let password = unsafe { CStr::from_ptr(password) }
        .to_string_lossy()
        .to_string();

    let decrypted = match decrypt_file(input, &password) {
        Ok(v) => v,
        Err(_) => return MEIKA_CRYPTO_ERROR,
    };

    unsafe {
        *out_len = decrypted.len();
        *out_ptr = libc::malloc(decrypted.len()) as *mut c_uchar;
        if (*out_ptr).is_null() {
            return MEIKA_IO_ERROR;
        }
        ptr::copy_nonoverlapping(decrypted.as_ptr(), *out_ptr, decrypted.len());
    }

    MEIKA_OK
}

/// ==========================
/// FILE MODE (STREAMING)
/// ==========================

#[no_mangle]
pub extern "C" fn meika_encrypt_file(
    input: *const c_char,
    output: *const c_char,
    password: *const c_char,
) -> i32 {
    if input.is_null() || output.is_null() || password.is_null() {
        return MEIKA_INVALID_INPUT;
    }

    let input = unsafe { CStr::from_ptr(input) }.to_string_lossy();
    let output = unsafe { CStr::from_ptr(output) }.to_string_lossy();
    let password = unsafe { CStr::from_ptr(password) }.to_string_lossy();

    let reader = match std::fs::File::open(&*input) {
        Ok(f) => f,
        Err(_) => return MEIKA_IO_ERROR,
    };

    let writer = match std::fs::File::create(&*output) {
        Ok(f) => f,
        Err(_) => return MEIKA_IO_ERROR,
    };

    match encrypt_stream(reader, writer, &password, 64 * 1024) {
        Ok(_) => MEIKA_OK,
        Err(_) => MEIKA_CRYPTO_ERROR,
    }
}

#[no_mangle]
pub extern "C" fn meika_decrypt_file(
    input: *const c_char,
    output: *const c_char,
    password: *const c_char,
) -> i32 {
    if input.is_null() || output.is_null() || password.is_null() {
        return MEIKA_INVALID_INPUT;
    }

    let input = unsafe { CStr::from_ptr(input) }.to_string_lossy();
    let output = unsafe { CStr::from_ptr(output) }.to_string_lossy();
    let password = unsafe { CStr::from_ptr(password) }.to_string_lossy();

    let reader = match std::fs::File::open(&*input) {
        Ok(f) => f,
        Err(_) => return MEIKA_IO_ERROR,
    };

    let writer = match std::fs::File::create(&*output) {
        Ok(f) => f,
        Err(_) => return MEIKA_IO_ERROR,
    };

    match decrypt_stream(reader, writer, &password) {
        Ok(_) => MEIKA_OK,
        Err(_) => MEIKA_CRYPTO_ERROR,
    }
}

/// ==========================
/// MEMORY FREE
/// ==========================

#[no_mangle]
pub extern "C" fn meika_free(ptr: *mut c_uchar) {
    if !ptr.is_null() {
        unsafe { libc::free(ptr as *mut _) };
    }
}
