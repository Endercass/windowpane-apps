include!(concat!(env!("OUT_DIR"), "/manifest/mod.rs"));
use clap::{Parser, ValueHint};
use dircpy::copy_dir;

use sha256::digest;
use std::env::temp_dir;
use std::fs::{self, remove_dir_all, File};
use std::io::Error;
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
    let mut bin_path: PathBuf = args
        .manifest
        .parent()
        .expect("Could not get parent of manifest.")
        .to_path_buf();
    bin_path.push(manifest_value.bin_dir);

    let mut output_tarball: PathBuf = args.out;
    output_tarball.push(format!("{}.wpapp", digest(out_bytes.as_slice())));

    println!("Copying binaries...");
    copy_dir(bin_path, bin).expect("Could not copy files from binpath.");
    let file = File::create(output_tarball).unwrap();
    let mut builder: Builder<File> = Builder::new(file);
    builder
        .append_dir_all(".", tempfs.to_str().unwrap())
        .expect("Could not add file to archive.");
    println!("Cleaning up...");
    remove_dir_all(tempfs).expect("Could not remove directory.");
}
