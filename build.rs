fn main() {
    println!("cargo:rerun-if-changed=grammars");
    println!("cargo:rerun-if-changed=xtask");
}
