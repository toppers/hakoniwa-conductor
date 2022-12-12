
fn main() {
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-search=native=./");
    //println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
}
