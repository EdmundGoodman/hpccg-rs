fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("hpccg_src");
    let mut b = autocxx_build::Builder::new("src/hpccg/ddot.rs", [&path]).build()?;
    b.file("hpccg_src/ddot.cpp").flag_if_supported("-std=c++14").compile("autocxx-demo");

    println!("cargo:rerun-if-changed=src/hpccg/ddot.rs");
    println!("cargo:rerun-if-changed=hpccg_src/ddot.cpp");
    println!("cargo:rerun-if-changed=hpccg_src/ddot.hpp");
    Ok(())
}
