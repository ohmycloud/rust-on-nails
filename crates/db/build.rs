use std::{env, path::Path};

fn cornucopia() {
    let queries_path = "queries";
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("cornucopia.rs");

    let db_url = env::var("DATABASE_URL").unwrap();

    // Return this build script if the queries or migrations change.
    println!("cargo:rerun-if-change={queries_path}");

    // Call cornucopia. Use whatever CLI you need.
    // cargo install cornucopia first.
    let output = std::process::Command::new("cornucopia")
        .arg("-q")
        .arg(queries_path)
        .arg("--serialize")
        .arg("-d")
        .arg(&file_path)
        .arg("live")
        .arg(db_url)
        .output()
        .unwrap();

    // If Cornucopia couldn't run properly, try to display the error.
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }
}

fn main() {
    // Compile our SQL.
    cornucopia();
}
