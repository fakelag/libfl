fn main() {
    println!("cargo::rerun-if-changed=src/libfl.c");
    cc::Build::new().file("src/libfl.c").compile("libfl");
}
