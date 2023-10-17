fn main() {
    println!("cargo:rustc-link-search=libs");
    println!("cargo:rustc-link-lib=static=glfw3");
    println!("cargo:rustc-link-lib=shell32");
    println!("cargo:rustc-link-lib=gdi32");
}
