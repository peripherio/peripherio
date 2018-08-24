use serde_json::value::{Value, Number};

use std::{slice, mem};
use std::ffi::{CStr, CString};


pub unsafe fn alloc(len: usize) -> *mut u8 {
    let mut vec = Vec::<u8>::with_capacity(len);
    vec.set_len(len);
    Box::into_raw(vec.into_boxed_slice()) as *mut u8
}

pub unsafe fn free(raw: *mut u8, len : usize) {
    let s = slice::from_raw_parts_mut(raw, len);
    let _ = Box::from_raw(s);
}

pub fn size_of_value(v: &Value) -> usize {
    match v {
        /*Value::Null => 0, Unsupported */
        Value::Bool(_) => 1, // u8
        Value::Number(_) => 8, // f64
        Value::String(_) => 8, // ptr(64bit)
        Value::Array(_) => 8, // ptr
        /*Value::Object(_) => 8, Write someday // ptr */
        _ => unimplemented!()
    }
}

pub fn size_of_type(typestr: &str) -> usize {
    match typestr {
        "string" | "number" | "integer" | "array" => 8,
        "bool" => 1,
        _ => unimplemented!()
    }
}

pub unsafe fn cast_to_ptr(v: &Value) -> *const u8 {
    match v {
        /*Value::Null => 0, Unsupported */
        Value::Bool(b) => mem::transmute::<&bool, *const u8>(&b),
        Value::Number(n) => {
            let via = n.as_f64().unwrap();
            mem::transmute::<&f64, *const u8>(&via) // f64
        },
        Value::String(s) => {
            let ptr = CString::new(s.clone()).unwrap().into_raw();
            mem::transmute::<&*mut i8, *const u8>(&ptr) // ptr
        }
        Value::Array(ary) => Box::into_raw(ary.clone().into_boxed_slice()) as *const u8, // ptr
        /*Value::Object => 8, Write someday // ptr */
        _ => unimplemented!()
    }
}

pub unsafe fn cast_from_ptr(type_str: &str, ptr: *const u8) -> Value {
    match type_str {
        /*"null" => 0, Unsupported */
        "bool" => Value::Bool(*mem::transmute::<*const u8, &bool>(ptr)),
        "integer" | "number" => {
            let f = mem::transmute::<*const u8, &f64>(ptr); // f64
            Value::Number(Number::from_f64(*f).unwrap())
        },
        "string" => {
            let sp = mem::transmute::<*const u8, &*mut i8>(ptr); // ptr
            let cstr = CStr::from_ptr(*sp);
            Value::String(cstr.to_str().unwrap().to_string())
        }
        // "array" => Box::into_raw(ary.clone().into_boxed_slice()) as *const u8, // ptr
        /*"object" => 8, Write someday // ptr */
        _ => unimplemented!()
    }
}
