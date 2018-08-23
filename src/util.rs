use serde_json::value::Value;

use std::{slice, mem};

pub unsafe fn alloc(len: usize) -> *mut u8 {
    let mut vec = Vec::<u8>::with_capacity(len);
    vec.set_len(len);
    Box::into_raw(vec.into_boxed_slice()) as *mut u8
}

pub unsafe fn free(raw: *mut u8, len : usize) {
    let s = slice::from_raw_parts_mut(raw, len);
    let _ = Box::from_raw(s);
}

pub fn size_of_value(v: Value) -> usize {
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

pub unsafe fn cast_to_ptr(v: Value) -> *const u8 {
    match v {
        /*Value::Null => 0, Unsupported */
        Value::Bool(b) => mem::transmute::<&bool, *const u8>(&b),
        Value::Number(n) => {
            let via = n.as_f64();
            mem::transmute::<&f64, *const u8>(&via) // f64
        },
        Value::String(s) => mem::transmute::<&str, *const u8>(s.as_str()), // ptr(64bit)
        Value::Array(ary) => Box::into_raw(ary.into_boxed_slice()) as *const u8, // ptr
        /*Value::Object => 8, Write someday // ptr */
        _ => unimplemented!()
    }
}
