fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("hpccg_src");
    let mut b = autocxx_build::Builder::new("src/cpp_ffi.rs", [&path]).build()?;

    let cpp_basename = vec!["waxpby", "ddot"];
    for basename in cpp_basename {
        b.file(format!("hpccg_src/{}.cpp", basename))
            .flag_if_supported("-std=c++14")
            .compile("autocxx-demo");
        println!("cargo:rerun-if-changed=hpccg_src/{}.cpp", basename);
        println!("cargo:rerun-if-changed=hpccg_src/{}.hpp", basename);
    }

    println!("cargo:rerun-if-changed=src/cpp_ffi.rs");
    Ok(())
}
