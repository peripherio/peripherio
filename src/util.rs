use config::global::GLOBAL_SCHEMA;
use config::Config;
use error::{InvalidJSONNumberError, InvalidNumberError, TypeNotFoundError, UnknownConfigError};

use failure::Error;
use linked_hash_map::LinkedHashMap;
use serde_json::value::{Number, Value};

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::{fmt, mem, ptr, slice};

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
                        .as_f64()
                        .ok_or(InvalidJSONNumberError { value: n.clone() })?
                        as i64;
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

pub fn value_to_c_struct(
    requires: &LinkedHashMap<String, Value>,
    value: &HashMap<String, Value>,
) -> Result<(*mut u8, usize), Error> {
    let types = requires
        .iter()
        .map(|(k, v)| {
            Ok((
                k,
                v.get("type")
                    .and_then(|v| v.as_str())
                    .ok_or({
                        Error::from(TypeNotFoundError {
                            field: k.to_string(),
                        })
                    })?
                    .to_string(),
            ))
        })
        .collect::<Result<LinkedHashMap<&String, String>, Error>>()?;
    let entire_size: usize = types.iter().fold(0, |sum, (_, v)| sum + size_of_type(v));
    let buf = unsafe { alloc(entire_size) };
    let mut filled_size: usize = 0;
    for (k, v) in types {
        let size = size_of_type(&v);
        if let Some(val) = value.get(k) {
            unsafe {
                let ptr = cast_to_ptr(&v, val)?;
                ptr::copy_nonoverlapping(ptr, buf.offset(filled_size as isize), size);
            }
            filled_size += size;
        } else {
            unsafe { ptr::write_bytes(buf.offset(filled_size as isize), 0, size) };
            filled_size += size;
        }
    }
    Ok((buf, entire_size))
}

pub fn c_struct_to_value(
    requires: &LinkedHashMap<String, Value>,
    value: *const u8,
) -> Result<HashMap<String, Value>, Error> {
    let mut newconf = Config::new();
    let mut retrieved_size: usize = 0;
    for (k, v) in requires {
        let type_str = v.get("type").and_then(|v| v.as_str()).ok_or(Error::from(
            TypeNotFoundError {
                field: k.to_string(),
            },
        ))?;
        let size = size_of_type(type_str);
        let val = unsafe {
            let buf = alloc(size);
            ptr::copy_nonoverlapping(value.offset(retrieved_size as isize), buf, size);
            let val = cast_from_ptr(type_str, buf)?.clone();
            free(buf, size);
            val
        };
        retrieved_size += size;
        newconf.insert(k.to_string(), val);
    }
    Ok(newconf)
}

pub fn merge_value(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge_value(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

pub fn merge_schema_with_global(
    requires: &Vec<String>,
    schemas: &LinkedHashMap<String, Value>,
) -> Result<LinkedHashMap<String, Value>, Error> {
    requires
        .iter()
        .map(|key| {
            Ok((
                key.clone(),
                match (schemas.get(key), GLOBAL_SCHEMA.get(key)) {
                    (Some(schema), Some(v)) => {
                        let mut new_val = v.clone();
                        merge_value(&mut new_val, &schema);
                        new_val
                    }
                    (Some(schema), None) => schema.clone(),
                    (None, Some(v)) => v.clone(),
                    (None, None) => return Err(UnknownConfigError { name: key.clone() }.into()),
                },
            ))
        })
        .collect::<Result<LinkedHashMap<_, _>, Error>>()
}
