use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    protobuf_codegen_pure::Codegen::new()
        .out_dir(out_dir)
        .include(".")
        .input("./krpc.proto")
        .customize(protobuf_codegen_pure::Customize {
            serde_derive: Some(true),
            expose_fields: Some(true),
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .run()
        .unwrap();
}
