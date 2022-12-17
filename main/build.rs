
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-search=/usr/local/lib/hakoniwa");
    tonic_build::compile_protos("hakoniwa-core-spec/grpc/hakoniwa_core.proto")?;
    Ok(())
}
