use std::env;
use std::fs::create_dir_all;
use std::{
    fs::{copy, read_dir},
    path::Path,
};

static ASSET_PATH: &str = "assets/";

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed={}", ASSET_PATH);

    let asset_path = Path::new(ASSET_PATH);
    let out_dir = env::var("OUT_DIR").unwrap();
    dbg!(&out_dir);

    copy_dir(
        asset_path,
        Path::new(format!("{}/{}", out_dir, ASSET_PATH).as_str()),
    );
}

fn copy_dir(from: &Path, to: &Path) {
    let paths = read_dir(from).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap();

        let dest = to.join(path.file_name().unwrap());

        if path.is_dir() {
            match create_dir_all(&dest) {
                Ok(_) => copy_dir(&path, &dest),
                Err(why) => {
                    if why.kind() != std::io::ErrorKind::AlreadyExists {
                        panic!("Couldn't create directory: {}", why);
                    }
                }
            };
            copy_dir(&path, &dest);
        } else {
            println!("cargo:rerun-if-changed={}", path_str);
            dbg!(&path);
            dbg!(&dest);
            copy(path, dest).unwrap();
        }
    }
}
