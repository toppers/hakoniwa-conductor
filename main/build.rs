
fn main() {
    println!("cargo:rustc-link-search=native=./");
    println!("cargo:rustc-link-search=native=/usr/local/Cellar/gcc/11.2.0_3/lib/gcc/11/");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
}
