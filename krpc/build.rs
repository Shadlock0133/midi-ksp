use std::{fs::{read_to_string, write}, path::PathBuf};

use protobuf_but_worse::{codegen, protobuf_parser};

fn main() {
    let file = read_to_string("krpc.proto").unwrap();
    let proto = protobuf_parser::FileDescriptor::parse(&file).unwrap();
    let code = codegen::gen_proto(&proto).unwrap();
    let out_file = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("krpc.rs");
    write(out_file, code).unwrap();
}
