use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::{env, fs::create_dir, path::Path};

const RES: &'static str = "../res/";

fn main() {
    println!("cargo:rerun-if-changed={}*", RES);
    handle_res_folder();
}

fn handle_res_folder() {
    let path = Path::new(RES);

    if !path.exists() {
        create_dir(path).unwrap();
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = format!("{}/../../../", out_dir);

    let out_dir = Path::new(&out_dir);
    if !out_dir.exists() {
        create_dir(out_dir).unwrap();
    }

    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let mut paths_to_copy = Vec::new();
    paths_to_copy.push(RES);
    copy_items(&paths_to_copy, out_dir, &copy_options).unwrap();
}
