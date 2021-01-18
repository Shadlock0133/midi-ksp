use std::{
    error::Error,
    io,
    path::{Path, PathBuf},
};

pub use protobuf_parser;

pub mod codegen;
pub mod encoding;

pub fn generate(proto_file: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let proto_file = proto_file.as_ref();
    let bytes = std::fs::read(proto_file).map_err(|e| {
        let msg = format!("Error reading {}: {}", proto_file.display(), e);
        io::Error::new(io::ErrorKind::Other, msg)
    })?;
    let proto = protobuf_parser::FileDescriptor::parse(bytes).map_err(|e| {
        let msg =
            format!("Error when parsing {}: {:?}", proto_file.display(), e);
        io::Error::new(io::ErrorKind::InvalidInput, msg)
    })?;
    let code = codegen::gen_proto(&proto).map_err(|e| {
        let msg = format!(
            "Error when generating code for {}: {:?}",
            proto_file.display(),
            e
        );
        io::Error::new(io::ErrorKind::InvalidInput, msg)
    })?;

    let rs_file = proto_file.with_extension("rs");
    let out_path = PathBuf::from(std::env::var("OUT_DIR")?).join(rs_file);
    std::fs::write(out_path, code)?;
    Ok(())
}
