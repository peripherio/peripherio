use error::{InvalidJSONNumberError, InvalidNumberError};

use failure::Error;
use serde_json::value::{Number, Value};

use std::ffi::{CStr, CString};
use std::{mem, slice};

pub unsafe fn alloc(len: usize) -> *mut u8 {
    let mut vec = Vec::<u8>::with_capacity(len);
    vec.set_len(len);
    Box::into_raw(vec.into_boxed_slice()) as *mut u8
}

pub unsafe fn free(raw: *mut u8, len: usize) {
    let s = slice::from_raw_parts_mut(raw, len);
    let _ = Box::from_raw(s);
}

pub fn size_of_type(typestr: &str) -> usize {
    match typestr {
        "string" | "integer" | "number" | "array" => 8,
        "bool" => 1,
        _ => unimplemented!(),
    }
}

pub unsafe fn cast_to_ptr(type_str: &str, v: &Value) -> Result<*const u8, Error> {
    Ok(match v {
        /*Value::Null => 0, Unsupported */
        Value::Bool(b) => mem::transmute::<&bool, *const u8>(&b),
        Value::Number(n) => {
            match type_str {
                "number" => {
                    let via: f64 = n
                        .as_f64()
                        .ok_or(InvalidJSONNumberError { value: n.clone() })?;
                    mem::transmute::<&f64, *const u8>(&via) // f64
                }
                "integer" => {
                    let via: i64 = n
                        .as_i64()
                        .ok_or(InvalidJSONNumberError { value: n.clone() })?;
                    mem::transmute::<&i64, *const u8>(&via) // i64
                }
                _ => unimplemented!(),
            }
        }
        Value::String(s) => {
            let ptr = CString::new(s.clone())?.into_raw();
            mem::transmute::<&*mut i8, *const u8>(&ptr) // ptr
        }
        // Value::Array(ary) => Box::into_raw(ary.clone().into_boxed_slice()) as *const u8, // ptr
        /*Value::Object => 8, Write someday // ptr */
        _ => unimplemented!(),
    })
}

pub unsafe fn cast_from_ptr(type_str: &str, ptr: *const u8) -> Result<Value, Error> {
    Ok(match type_str {
        /*"null" => 0, Unsupported */
        "bool" => Value::Bool(*mem::transmute::<*const u8, &bool>(ptr)),
        "number" => {
            let f = mem::transmute::<*const u8, &f64>(ptr); // f64
            Value::Number(Number::from_f64(*f).ok_or(InvalidNumberError { value: *f })?)
        }
        "integer" => {
            let f = mem::transmute::<*const u8, &i64>(ptr); // i64
            Value::Number(
                Number::from_f64(*f as f64).ok_or(InvalidNumberError { value: *f as f64 })?,
            )
        }
        "string" => {
            let sp = mem::transmute::<*const u8, &*mut i8>(ptr); // ptr
            let cstr = CStr::from_ptr(*sp);
            Value::String(cstr.to_str()?.to_string())
        }
        // "array" => Box::into_raw(ary.clone().into_boxed_slice()) as *const u8, // ptr
        /*"object" => 8, Write someday // ptr */
        _ => unimplemented!(),
    })
}
