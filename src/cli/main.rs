include!(concat!(env!("OUT_DIR"), "/manifest/mod.rs"));
use clap::{Parser, ValueHint};
use dircpy::copy_dir;

use sha256::digest;
use std::env::temp_dir;
use std::fs::{self, copy, remove_dir_all, File};
use std::path::PathBuf;

use tar::Builder;
use uuid::Uuid;

use protobuf::Message;

#[derive(Parser, Debug)]
struct Arguments {
    #[arg(value_name = "manifest", value_hint = ValueHint::DirPath, help = "Manifest file that will be read")]
    manifest: std::path::PathBuf,
    #[arg(short = 'o', long = "out", value_name = "output", value_hint = ValueHint::DirPath, help = "Folder in which the package will be placed")]
    out: std::path::PathBuf,
    #[arg(short = 'i', long = "include", value_name = "include_path", value_hint = ValueHint::DirPath, help = "Folder to include in package, should contain source files (html for embedding, image files, etc)\nNOTE: Do not place your launch.js here.")]
    include_path: std::path::PathBuf,
}

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);

    let manifest_value: wpapps::manifest::Manifest =
        wpapps::read_manifest_json(args.manifest.clone());

    let out_bytes: Vec<u8> = manifest_value.write_to_bytes().unwrap();
    let mut tempfs: PathBuf = temp_dir();
    tempfs.push(format!("wpapp-work-{}", Uuid::new_v4()));
    println!("Creating work directory: \"{}\".", tempfs.to_str().unwrap());
    fs::create_dir(tempfs.clone()).expect("Could not create directory");
    let mut manifest: PathBuf = tempfs.clone();
    manifest.push("manifest.wpmf");
    println!("Writing manifest...");
    fs::write(manifest, out_bytes.clone()).expect("Could not write file.");
    let mut bin: PathBuf = tempfs.clone();
    bin.push("bin");
    println!("Creating binary directory...");
    fs::create_dir(bin.clone()).expect("Could not create directory");

    let mut output_tarball: PathBuf = args.out;
    output_tarball.push(format!("{}.wpapp", digest(out_bytes.as_slice())));

    println!("Copying resources...");
    copy_dir(args.include_path, bin).expect("Could not copy files from binpath.");

    let mut launch_script: PathBuf = args
        .manifest
        .parent()
        .expect("Could not get parent of manifest.")
        .to_path_buf();
    launch_script.push("launch.js");

    let mut launch_script_out: PathBuf = tempfs.clone();
    launch_script_out.push("launch.js");

    println!("Copying launch script...");
    copy(launch_script, launch_script_out).expect("Could not copy files from binpath.");

    let file = File::create(output_tarball).unwrap();
    let mut builder: Builder<File> = Builder::new(file);
    builder
        .append_dir_all(".", tempfs.to_str().unwrap())
        .expect("Could not add file to archive.");
    println!("Cleaning up...");
    remove_dir_all(tempfs).expect("Could not remove directory.");
}
