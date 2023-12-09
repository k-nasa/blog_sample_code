fn main() {
    println!("cargo:rustc-link-arg-bin=code_for_blog=--script=src/link.ld");
}
