extern crate cmake;
use cmake::*;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::vec;
use std::fs::hard_link;
use std::fs;
use std::path::PathBuf;
use std::path::Path;

fn degug_write_env_to_file() -> File {
    let mut file = File::create("foo.txt").unwrap();
    for e in env::vars() {
        let d = format!("{:?}\n", e);
        file.write(d.as_ref());
    }

    return file;
}

fn glob_folder<P: AsRef<Path>>(folder_path: &P, libs:&mut Vec<PathBuf>) {
    let files = fs::read_dir(folder_path).unwrap();
    for file in files {
        let ref entry = file.unwrap();
        let path = entry.path().clone();
        if path.is_file() && path.extension().unwrap() == "dll" ||  path.extension().unwrap() == "libc"{
            libs.push(path.clone());
        }
    }
}

fn cmake() {
//    let mut conf = Config::new(external_dir.clone() + "SDL2");
//    conf.define("CMAKE_BUILD_TYPE", "Release");
//    conf.define("CMAKE_RUNTIME_OUTPUT_DIRECTORY", &target_dir);

    //    let mut dst = conf.build();
    //sdl2
//    println!("cargo:rustc-link-search=native={}", dst.display());
//    println!("cargo:rustc-link-lib=dylib=SDL2");
//    println!("cargo:rustc-link-lib=dylib=libSDL2");
//    println!("cargo:rustc-link-lib=static=libSDL2d");
//    if is_gnu && is_windows {
//        conf.generator("MinGW Makefiles");
//    }
}

fn main() {
    let mut libs: Vec<PathBuf> = Vec::new();

    let mut out_file = degug_write_env_to_file();

    let out_dir = env::var( "OUT_DIR").unwrap();
    let profile = env::var( "PROFILE").unwrap();
    const TARGET_STR: &str = "target";
    let target_dir = out_dir[0..out_dir.rfind(TARGET_STR).unwrap() + TARGET_STR.len()].to_string() + &"/" + &profile ;
    println!("cargo:rustc-link-search=native={}", &target_dir);

    let is_gnu = env::var("CARGO_CFG_TARGET_ENV").unwrap() == "gnu";
    let is_windows = env::var("CARGO_CFG_TARGET_FAMILY").unwrap() == "windows";
    let external_dir =  "../external/".to_string();


    if is_gnu && is_windows {
        //sdl2 image
        let pp = external_dir + r#"SDL2\mingw_64"#;
        println!("cargo:rustc-link-search=native={}", &pp);
        glob_folder(&pp, &mut libs);
    } else if is_windows {
        //sdl2 image
        let pp = external_dir + r#"SDL2\vc_64"#;
        println!("cargo:rustc-link-search=native={}", &pp);
        glob_folder(&pp, &mut libs);
    }

    for ref lib in libs {
        let output = target_dir.clone() + &"/" + lib.file_name().unwrap().to_str().unwrap();
        out_file.write(format!("adding dll {:?} to {:?}\n", lib.display(), output).as_ref());
        hard_link(&lib, output);
    }
}
