use std::process::Command;
use std::path::PathBuf;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=motors_raw");
    println!("cargo:rerun-if-changed=motors_build_config.yaml");
    println!("cargo:rerun-if-changed=build-motors.py");
    let here = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let output = Command::new("python")
        .args(["build-motors.py"])
        .current_dir(here)
        .output()
        .expect("Running rocket motor parser script");
    if !output.status.success() {
        println!("{}", format!("cargo:warning={:?}", output));
    }
}
