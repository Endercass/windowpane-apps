include!(concat!(env!("OUT_DIR"), "/manifest/mod.rs"));
use crate::manifest::{Manifest, Backend};

fn main() {
    let mut man = Manifest::new();
    man.app_name = "Hi".to_owned();
    man.start= "index.html".to_owned();
    man.backend= Backend::DOM.into();
    println!("Hello, {:#?}!", man);
}