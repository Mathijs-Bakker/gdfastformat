fn main() {
    // Tell Rust where to look for libraries
    println!("cargo:rustc-link-search=native=../tree-sitter-gdscript");

    // Tell Rust which library to link
    println!("cargo:rustc-link-lib=static=tree-sitter-gdscript");
}
