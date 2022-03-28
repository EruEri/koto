use std::process::Command;

fn main(){
    let out_dir = "C/object";

    let _ = Command::new("cc")
    .args(vec!["-c", "C/date/date.c", "-o", format!("{}/date.o", out_dir).as_str()])
    .status();
    let status = Command::new("cc")
    .args(vec!["-c", "C/dir/dir.c", "-o", format!("{}/dir.o", out_dir).as_str()])
    .status();
    println!("Status : {:?}", status);
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib={}", "date.o");
    println!("cargo:rustc-link-lib={}", "dir.o");
    println!("cargo:rerun-if-changed=C/dir/dir.c");
    println!("cargo:rerun-if-changed=C/date/date.c");
}