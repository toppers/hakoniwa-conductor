
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rustc-link-search=/usr/lib");
    if cfg!(target_arch = "x86_64") {
        println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");
    } else if cfg!(target_arch = "aarch64") {
        println!("cargo:rustc-link-search=/usr/lib/aarch64-linux-gnu");
    }
    println!("cargo:rustc-link-search=/usr/local/lib/hakoniwa");
    tonic_build::compile_protos("spec/hakoniwa_core.proto")?;
    Ok(())
}
