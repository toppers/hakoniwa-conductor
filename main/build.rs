
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-search=/usr/local/lib/hakoniwa");
    tonic_build::compile_protos("spec/hakoniwa_core.proto")?;
    Ok(())
}
