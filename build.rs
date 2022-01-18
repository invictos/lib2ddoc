mod src_build;
use src_build as build;

use std::{env, path::PathBuf};




fn main(){
    println!("cargo:rerun-if-changed=json/s6.json");
    println!("cargo:rerun-if-changed=json/s7.json");
    println!("cargo:rerun-if-changed=json/s8.json");


    let base_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    build::Loader::new("json")
        .all(
            base_path.join("document_loader_inserts.rs").to_str().unwrap(),
            base_path.join("identifier_loader_inserts.rs").to_str().unwrap(),
        ).expect("Can't create specifications");
}