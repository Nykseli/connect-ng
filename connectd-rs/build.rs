use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("go")
        .args(&["build", "-v", "-buildmode=c-archive", "-o"])
        .arg(&format!("{}/libsuseconnect.a", out_dir))
        .arg("github.com/SUSE/connect-ng/libsuseconnect")
        .current_dir("..")
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=suseconnect");
    println!("cargo:rerun-if-changed=../libsuseconnect/libsuseconnect.go");
}
