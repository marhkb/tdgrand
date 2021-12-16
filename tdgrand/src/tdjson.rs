// Copyright 2021 - developers of the `tdgrand` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use once_cell::sync::Lazy;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int};
use std::sync::Mutex;

static LOG_REDIRECT: Lazy<Mutex<Option<Box<dyn Fn(i32, &str) + Send>>>> =
    Lazy::new(|| Mutex::new(None));

static mut LOG_REDIRECT_UNSAFE: Option<Box<dyn Fn(i32, &str) + Send>> = None;

extern "C" fn log_message_callback(verbosity_level: i32, message: *const i8) {
    let message = unsafe { CStr::from_ptr(message) }.to_str().unwrap();
    unsafe { LOG_REDIRECT_UNSAFE.as_ref() }.unwrap()(verbosity_level, message);
    // LOG_REDIRECT.lock().unwrap().as_ref().unwrap()(verbosity_level, message);
}

#[link(name = "tdjson")]
extern "C" {
    fn td_create_client_id() -> c_int;
    fn td_send(client_id: c_int, request: *const c_char);
    fn td_receive(timeout: c_double) -> *const c_char;
    fn td_set_log_message_callback(
        max_verbosity_level: c_int,
        callback: Option<extern "C" fn(c_int, *const i8)>,
    );
}

pub(crate) fn create_client() -> i32 {
    unsafe { td_create_client_id() }
}

pub(crate) fn send(client_id: i32, request: String) {
    let cstring = CString::new(request).unwrap();
    unsafe { td_send(client_id, cstring.as_ptr()) }
}

pub(crate) fn receive(timeout: f64) -> Option<String> {
    unsafe {
        td_receive(timeout)
            .as_ref()
            .map(|response| CStr::from_ptr(response).to_string_lossy().into_owned())
    }
}

pub(crate) fn set_log_message_callback(
    max_verbosity_level: i32,
    callback: Option<Box<dyn Fn(i32, &str) + Send>>,
) {
    // *LOG_REDIRECT.lock().unwrap() = callback;
    unsafe {
        LOG_REDIRECT_UNSAFE = callback;
        td_set_log_message_callback(
            max_verbosity_level,
            // match *LOG_REDIRECT.lock().unwrap() {
            match LOG_REDIRECT_UNSAFE {
                None => None,
                Some(_) => Some(log_message_callback),
            },
        );
    }
}
