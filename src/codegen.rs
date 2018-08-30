extern crate rami;
extern crate serde_json;
extern crate linked_hash_map;

use linked_hash_map::LinkedHashMap;

use rami::driver::driver::DriverData;
use rami::util;
use rami::category::Signature;

use std::collections::HashMap;

fn ctype(type_str: &str) -> String {
    match type_str {
        "number" => "double",
        "integer" => "long",
        "string" => "const char*",
        _ => unimplemented!()
    }.to_string()
}

fn merge_map(map1: &LinkedHashMap<String, serde_json::Value>, map2: &LinkedHashMap<String, serde_json::Value>) -> LinkedHashMap<String, serde_json::Value>{
    map1.keys()
        .map(|key| {
            (
                key.clone(),
                match (map1.get(key), map2.get(key)) {
                    (Some(schema), Some(v)) => {
                        let mut new_val = v.clone();
                        util::merge_value(&mut new_val, &schema);
                        new_val
                    }
                    (Some(schema), None) => schema.clone(),
                    _ => unreachable!()
                },
            )
        }).collect()
}


fn main() {
    let drv = DriverData::new(".").unwrap();

    println!("typedef struct Config_ {{");
    for (key, val) in drv.schemas() {
        println!("  {} {};", ctype(val["type"].as_str().unwrap()), key.replace(".", "_"));
    }
    println!("}} __attribute__((__packed__)) Config;");

    let mut merged_signs: HashMap<String, Signature> = HashMap::new();
    for ctg in drv.category() {
        for (name, sign) in ctg.signatures().into_iter() {
            let sign: Signature = sign.clone();
            let gs = if let Some(existing_sign) = merged_signs.get(name) {
                let args = if let Some(ref v_args) = existing_sign.args {
                    merge_map(&v_args, &sign.args.unwrap_or_default())
                } else {
                    sign.args.unwrap_or_default()
                };
                let returns = if let Some(ref v_rets) = existing_sign.returns {
                    merge_map(&v_rets, &sign.returns.unwrap_or_default())
                } else {
                    sign.returns.unwrap_or_default()
                };
                Signature { args:Some(args), returns: Some(returns) }
            } else { sign };
            merged_signs.insert(name.clone(), gs);
        }
    }

    for (name, sign) in merged_signs {
        println!("typedef struct {}_args_ {{", name);
        for (key, val) in sign.args.clone().unwrap_or_default() {
            println!("  {} {};", ctype(val["type"].as_str().unwrap()), key.replace(".", "_"));
        }
        println!("}} __attribute__((__packed__)) {}_args;", name);

        println!("typedef struct {}_returns_ {{", name);
        for (key, val) in sign.returns.clone().unwrap_or_default() {
            println!("  {} {};", ctype(val["type"].as_str().unwrap()), key.replace(".", "_"));
        }
        println!("}} __attribute__((__packed__)) {}_returns;", name);
        println!("{0}_returns* {0}({0}* args, Config* conf);", name)
    }
}

