use std::env;
use std::path::{Path, PathBuf};
use std::fs::{read_dir, write};

fn main() {
    println!("cargo:rerun-if-changed=src/");
    create_asset_paths()
}

fn create_asset_paths() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("asset_paths.rs");

    let paths = get_asset_paths("./assets");
    let constant_string = paths_to_string(paths);

    write(
        &dest_path,
        constant_string
    ).unwrap();
}

fn get_asset_paths<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let mut paths = vec![];

    for path_result in read_dir(path).unwrap() {
        let path = path_result.unwrap().path();

        if path.is_file() {
            paths.push(path)
        } else {
            paths.extend(get_asset_paths(path))
        }
    }

    paths
}

fn paths_to_string(paths: Vec<PathBuf>) -> String {
    let joined_paths = paths.into_iter()
        .map(|p| p.to_str().unwrap().to_string())
        .map(|s| s.replace("\\", "/"))
        .map(|s| s.replace("./assets/", ""))
        .map(|s| format!(r#""{}""#, s))
        .collect::<Vec<_>>()
        .join(",");

    format!("[{}]", joined_paths)
}
