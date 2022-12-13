
fn main() {
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-search=/Users/tmori/project/hakoniwa-master-rust/main");
    //println!("cargo:rustc-link-search=native=/usr/lib");
    //println!("cargo:rustc-link-search=native=/usr/local/lib");
}
