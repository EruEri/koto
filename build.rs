use std::process::Command;

fn main(){
    let out_dir = "external/product";

    let _ = Command::new("cc")
    .args(vec!["-c", "external/date/date.c", "-o", format!("{}/date.o", out_dir).as_str()])
    .status()
    .expect("CC Date")
    ;
    let status = Command::new("cc")
    .args(vec!["-c", "external/dir/dir.c", "-o", format!("{}/dir.o", out_dir).as_str()])
    .status()
    .expect("CC Directory")
    ;
    let _ = Command::new("make")
    .args(vec!["-C", "external/libcuesheetmaker"])
    .status()
    .expect("Failed to make");
    println!("Status : {:?}", status);
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-search=native={}", "external/libcuesheetmaker/product");
    println!("cargo:rustc-link-lib={}", "date.o");
    println!("cargo:rustc-link-lib={}", "dir.o");
    println!("cargo:rustc-link-lib={}", "cuesheetmaker");
    println!("cargo:rerun-if-changed=external/libcuesheetmaker/cue_sheet_maker.c");
    println!("cargo:rerun-if-changed=external/dir/dir.c");
    println!("cargo:rerun-if-changed=external/date/date.c");
    println!("cargo:rerun-if-changed=build.rs");
}