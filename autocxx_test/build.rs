fn main() -> miette::Result<()> {
    let cpp_target_dir = std::path::PathBuf::from("hpccg_src");
    let mut builder = autocxx_build::Builder::new("src/cpp_ffi.rs", [&cpp_target_dir]).build()?;

    let cpp_basenames = vec![
        "compute_residual",
        "ddot",
        "exchange_externals",
        "generate_matrix",
        "HPC_Sparse_Matrix",
        "HPC_sparsemv",
        "HPCCG",
        "make_local_matrix",
        "waxpby",
    ];
    for basename in cpp_basenames {
        builder
            .file(format!("hpccg_src/{}.cpp", basename))
            .flag_if_supported("-std=c++14")
            .compile("autocxx-demo");
        println!("cargo:rerun-if-changed=hpccg_src/{}.cpp", basename);
        println!("cargo:rerun-if-changed=hpccg_src/{}.hpp", basename);
    }

    println!("cargo:rerun-if-changed=src/cpp_ffi.rs");
    Ok(())
}
