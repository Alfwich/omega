use std::env;
use std::fs::read_dir;
use std::path::Path;
use std::path::PathBuf;

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type);
    return PathBuf::from(path);
}

fn main() {
    let output_path = get_output_path();
    let paths = read_dir("extern/SFML-2.5.1/bin").unwrap();
    for p in paths {
        if p.as_ref()
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .ends_with(".dll")
        {
            let dst = Path::new(&output_path).join(p.as_ref().unwrap().path().file_name().unwrap());
            if !dst.exists() {
                let res = std::fs::copy(p.as_ref().unwrap().path(), dst);
                println!(
                    "cargo:warning=Copied dll to target directory: {:?}, res:{:?}",
                    p, res
                );
            }
        }
    }
}
