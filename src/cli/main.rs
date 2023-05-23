include!(concat!(env!("OUT_DIR"), "/manifest/mod.rs"));
use dircpy::copy_dir;

use sha256::digest;
use std::env::temp_dir;
use std::fs::{self, remove_dir_all, File};
use std::path::{Path, PathBuf};

use tar::Builder;
use uuid::Uuid;

use protobuf::Message;

fn main() {
    let input_path: PathBuf =
        Path::new("/home/endercass/windowpane-apps/examples/HelloWorld").to_path_buf();
    let mut manifest_path: PathBuf = input_path.clone();
    manifest_path.push("manifest.json");
    let output: wpapps::manifest::Manifest = wpapps::read_manifest_json(manifest_path);
    let out_bytes: Vec<u8> = output.write_to_bytes().unwrap();
    let mut dir: PathBuf = temp_dir();
    dir.push(format!("wpapp-work-{}", Uuid::new_v4()));
    println!("Creating temp dir: \"{}\".", dir.to_str().unwrap());
    fs::create_dir(dir.clone()).expect("Could not create directory");
    let mut manifest: PathBuf = dir.clone();
    manifest.push("manifest.wpmf");
    println!("Writing file: \"{}\".", manifest.to_str().unwrap());
    fs::write(manifest, out_bytes.clone()).expect("Could not write file.");
    let mut bin: PathBuf = dir.clone();
    bin.push("bin");
    println!("Creating dir: \"{}\".", bin.to_str().unwrap());
    fs::create_dir(bin.clone()).expect("Could not create directory");
    let mut bin_path: PathBuf = input_path.clone();
    bin_path.push(output.bin_dir);

    let mut output_tarball: PathBuf = input_path;
    output_tarball.push(format!("{}.wpapp", digest(out_bytes.as_slice())));

    println!(
        "Copying files from binpath to dir: \"{}\".",
        bin.to_str().unwrap()
    );
    copy_dir(bin_path, bin).expect("Could not copy files from binpath.");
    let file = File::create(output_tarball).unwrap();
    let mut builder: Builder<File> = Builder::new(file);
    builder
        .append_dir_all(".", dir.to_str().unwrap())
        .expect("Could not add file to archive.");
    remove_dir_all(dir).expect("Could not remove directory.");
}
