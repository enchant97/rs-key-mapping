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
    pub prefix: String,
}

impl KeyCode {
    pub fn prefix_as_pascal(&self) -> String {
        let p = self.prefix.to_lowercase();
        let parts = p.split("_");
        parts
            .map(|v| {
                let mut c = v.chars();
                let f = c.next().unwrap();
                f.to_uppercase().collect::<String>() + c.as_str()
            })
            .collect::<Vec<_>>()
            .join("")
    }
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

    // build usage-id enum

    write!(
        &mut mappings_fs,
        "/// Keyboard keys as enum values, with usage-id representation.\n"
    )
    .unwrap();
    write!(&mut mappings_fs, "#[repr(u8)]\n").unwrap();
    write!(
        &mut mappings_fs,
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = \"serde\", derive(Serialize, Deserialize))]\n"
    )
    .unwrap();
    write!(&mut mappings_fs, "pub enum Keys {{\n").unwrap();
    for key_code in &key_codes {
        write!(
            &mut mappings_fs,
            "    {} = {},\n",
            key_code.prefix_as_pascal(),
            key_code.usage_id
        )
        .unwrap();
    }
    write!(&mut mappings_fs, "}}\n").unwrap();

    // build static mappings for mapped keys
    let mut builder = phf_codegen::Map::new();
    for key in &key_codes {
        builder.entry(
            &key.usage_id,
            &format!(
                "MappedKey{{
    usage_id: {},
    dom_key: \"{}\",
    prefix: \"{}\",
}}",
                key.usage_id, key.key_code, key.prefix,
            ),
        );
    }
    write!(&mut mappings_fs, "/// usage-id to mapped key info.\n").unwrap();
    write!(
        &mut mappings_fs,
        "pub static MAPPED_KEYS: phf::Map<&'static str, MappedKey> = \n{};\n",
        builder.build(),
    )
    .unwrap();

    // build static mappings for US layout
    let mut builder = phf_codegen::Map::new();
    for key_code in &keycodes_us {
        builder.entry(&key_code.key_code, &key_code.usage_id);
    }
    write!(
        &mut mappings_fs,
        "/// DOM key to usage-id map for the US layout.\n"
    )
    .unwrap();
    write!(
        &mut mappings_fs,
        "pub static DOM_KEYS_US: phf::Map<&'static str, u8> = \n{};\n",
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
        "/// DOM key to usage-id map for the UK layout.\n"
    )
    .unwrap();
    write!(
        &mut mappings_fs,
        "pub static DOM_KEYS_UK: phf::Map<&'static str, u8> = \n{};\n",
        builder.build(),
    )
    .unwrap();
}
