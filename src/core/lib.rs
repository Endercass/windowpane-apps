include!(concat!(env!("OUT_DIR"), "/manifest/mod.rs"));
use crate::manifest::Manifest;
use protobuf::Message;
use protobuf_json_mapping::{parse_from_str_with_options, ParseOptions};
use std::fs::{self};
use std::path::PathBuf;
use std::str;

pub fn read_manifest_json(file: PathBuf) -> Manifest {
    let in_bytes = fs::read(file).expect("Could not read input manifest.");
    let s = match str::from_utf8(&in_bytes) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    parse_from_str_with_options::<Manifest>(
        s,
        &ParseOptions {
            ignore_unknown_fields: true,
            ..Default::default()
        },
    )
    .expect("Could not parse input manifest.")
}

pub fn read_manifest_wpmf(file: PathBuf) -> Manifest {
    let in_bytes = fs::read(file).expect("Could not read input manifest.");
    Manifest::parse_from_bytes(&in_bytes).expect("Could not parse input manifest.")
}
