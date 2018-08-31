extern crate linked_hash_map;
extern crate peripherio;
extern crate serde_json;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;

use linked_hash_map::LinkedHashMap;

use peripherio::category::{Category, Signature};
use peripherio::driver::driver::{DriverData, DRIVER_FILE};
use peripherio::util;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Read, Write};

fn ctype(type_str: &str) -> String {
    match type_str {
        "number" => "double",
        "integer" => "int64_t",
        "string" => "const char*",
        _ => unimplemented!(),
    }.to_string()
}

fn merge_map(
    map1: &LinkedHashMap<String, serde_json::Value>,
    map2: &LinkedHashMap<String, serde_json::Value>,
) -> LinkedHashMap<String, serde_json::Value> {
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
                    _ => unreachable!(),
                },
            )
        })
        .collect()
}

fn field_strs(fields: &LinkedHashMap<String, serde_json::Value>) -> String {
    fields.iter().fold(String::new(), |acc, (k, v)| {
        format!(
            "{}  {} {};\n",
            acc,
            ctype(v["type"].as_str().unwrap()),
            k.replace(".", "_")
        )
    })
}

#[derive(Deserialize)]
struct DriverFile {
    category: Vec<String>,
    requires: Option<Vec<String>>,
    schemas: Option<LinkedHashMap<String, serde_json::Value>>,
}

fn main() {
    let mut file = File::open(format!("./{}", DRIVER_FILE)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let data: DriverFile = serde_yaml::from_str(&contents).unwrap();

    let categories: Vec<Category> = data.category.iter().map(|c| c.parse().unwrap()).collect();

    let requires = data.requires.unwrap_or_default();
    let schemas = data.schemas.unwrap_or_default();
    let merged_schemas = util::merge_schema_with_global(&requires, &schemas).unwrap();

    let mut writer = BufWriter::new(File::create("peripherio.gen.h").unwrap());
    write!(&mut writer, "#include <stddef.h>\n\n"); // for size_t
    write!(&mut writer, "#include <stdint.h>\n\n"); // for int64_t

    let fields = field_strs(&merged_schemas);
    write!(
        &mut writer,
        "typedef struct Config_ {{\n{}}} __attribute__((__packed__)) Config;\n",
        fields
    );

    let mut merged_signs: HashMap<String, Signature> = HashMap::new();
    for ctg in categories {
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
                Signature {
                    args: Some(args),
                    returns: Some(returns),
                }
            } else {
                sign
            };
            merged_signs.insert(name.clone(), gs);
        }
    }

    for (name, sign) in merged_signs {
        let arg_fields = field_strs(&sign.args.unwrap_or_default());
        write!(
            &mut writer,
            "typedef struct {0}_args_ {{\n{1}}} __attribute__((__packed__)) {0}_args;\n",
            name, arg_fields
        );

        let return_fields = field_strs(&sign.returns.unwrap_or_default());
        write!(
            &mut writer,
            "typedef struct {0}_returns_ {{\n{1}}} __attribute__((__packed__)) {0}_returns;\n",
            name, return_fields
        );

        write!(
            &mut writer,
            "{0}_returns* {0}({0}_args* args, Config* conf);\n",
            name
        );
    }

    write!(&mut writer, "void init();\n");
    write!(&mut writer, "Config** detect(Config*, size_t*);\n");
}
