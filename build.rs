use std::process::Command;

fn main(){
    let out_dir = "src/C";

    let status = Command::new("clang")
    .args(vec!["-c", "src/C/date.c", "-o", format!("{}/date.o", out_dir).as_str()])
    .status();
    println!("Status : {:?}", status);
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib={}", "date.o");
    println!("cargo:rerun-if-changed=src/C/date.c");
}