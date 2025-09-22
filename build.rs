use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(
        env::var("OUT_DIR").expect("OUT_DIR was not defined by the cargo environment!"),
    );

    // Ensure rebuild when any capnp file changes
    println!("cargo:rerun-if-changed=capnp");

    // Also trigger rebuild if build.rs itself changes
    println!("cargo:rerun-if-changed=build.rs");

    capnpc::CompilerCommand::new()
        .src_prefix("capnp")
        .file("capnp/common.capnp")
        .file("capnp/init.capnp")
        .file("capnp/mining.capnp")
        .file("capnp/proxy.capnp")
        .file("capnp/echo.capnp")
        .output_path(out_path)
        .run()
        .expect("capnpc compilation failed");
}
