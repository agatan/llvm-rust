use std::process::Command;
use std::env;

fn main() {
    let llvm_config_path = env::var("LLVM_CONFIG_PATH").unwrap_or("llvm-config".to_string());

    let llvm_link_dir = Command::new(&llvm_config_path).arg("--ldflags").output().unwrap_or_else(|e| {
        panic!("failed to execute `llvm-config --ldflags`: {}", e);
    }).stdout;
    println!("cargo:rustc-flags=-l dylib=stdc++");
    println!("cargo:rustc-flags=-l tinfo");
    println!("cargo:rustc-link-search={}", String::from_utf8_lossy(&llvm_link_dir).replace("-L", ""));
}
