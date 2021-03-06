extern crate cmake;
extern crate sha2;

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use sha2::{Sha256, Digest};

fn main() {
    println!("cargo:rerun-if-changed=build/glslangValidator.exe");

    let target = env::var("TARGET").unwrap();
    let out_file = Path::new(&env::var("OUT_DIR").unwrap()).join("glslang_validator");

    let path = if target.contains("windows") {
        const SHA256SUM: &'static str = 
            "b6ebab5bd07dc7c0b54c59a2a303df58f7132e5fd6c177ffed92d20934dafa41";
        let path = Path::new("build/glslangValidator.exe").to_owned();
        let content = fs::read(&path).expect("failed to open executable");
        let mut hasher = Sha256::default();
        hasher.input(&content);
        let result = hasher.result();
        let sha256sum = format!("{:x}", result);
        assert_eq!(sha256sum, SHA256SUM, "glslangValidator.exe checksum failed");
        path

    } else {
        // Try to initialize submodules. Don't care if it fails, since this code also runs for
        // the crates.io package.
        let _ = Command::new("git")
            .arg("submodule")
            .arg("update")
            .arg("--init")
            .status();
        cmake::build("glslang");
        Path::new(&env::var("OUT_DIR").unwrap())
            .join("bin")
            .join("glslangValidator")
    };

    fs::copy(&path, &out_file).expect("failed to copy executable");
}
