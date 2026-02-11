fn main() {
    // Tell Cargo to rerun this build script if the file changes
    // println!("cargo:rerun-if-changed=xtask/turtle.txt");

    // You can also watch a whole directory
    println!("cargo:rerun-if-changed=xtask");

    // Optional: do some work here
    // e.g. validate, generate code, preprocess files, etc.
}
