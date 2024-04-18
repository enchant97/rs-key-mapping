use std::{
    env, fs,
    io::{BufWriter, Write},
    path::Path,
};

use serde::Deserialize;

#[derive(Deserialize)]
struct KeyCode {
    pub usage_id: String,
    pub key_code: String,
    pub lang_code: Option<String>,
    //pub visual: String,
    //pub prefix: String,
}

fn main() {
    println!("cargo::rerun-if-changed=src/key_codes.json");

    let mut rdr = fs::File::open("src/key_codes.json").unwrap();
    let key_codes: Vec<KeyCode> = serde_json::from_reader(&mut rdr).unwrap();
    let keycodes_us = key_codes
        .iter()
        .filter(|v| {
            if let Some(code) = &v.lang_code {
                return code == "US";
            }
            true
        })
        .collect::<Vec<_>>();
    let keycodes_uk = key_codes
        .iter()
        .filter(|v| {
            if let Some(code) = &v.lang_code {
                return code == "UK";
            }
            true
        })
        .collect::<Vec<_>>();

    let mappings_path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut mappings_fs = BufWriter::new(fs::File::create(mappings_path).unwrap());

    // build static mappings for US layout
    let mut builder = phf_codegen::Map::new();
    for key_code in &keycodes_us {
        builder.entry(&key_code.key_code, &key_code.usage_id);
    }
    write!(
        &mut mappings_fs,
        "pub const KEY_CODES_US: phf::Map<&'static str, u8> = \n{};\n",
        builder.build(),
    )
    .unwrap();

    // build static mappings for UK layout
    let mut builder = phf_codegen::Map::new();
    for key_code in &keycodes_uk {
        builder.entry(&key_code.key_code, &key_code.usage_id);
    }
    write!(
        &mut mappings_fs,
        "pub const KEY_CODES_UK: phf::Map<&'static str, u8> = \n{};\n",
        builder.build(),
    )
    .unwrap();
}
