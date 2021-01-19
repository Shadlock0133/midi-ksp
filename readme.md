# MIDI KSP Controller

A Rust program that allows to control Kerbal Space Program game (using KRPC mod)
by way of a MIDI controller (only works with M-Audio Axiom Air Mini 32).

`protobuf-but-worse` is WIP protobuf codegen crate, created because exisiting
crates weren't suitable for me. Doesn't support all of protobufs features,
I only implemented what needed for krpc. Pure Rust, for parsing `.proto` files 
uses protobuf-parser, a nom 3 parser; codegen using `syn` & co. crates.

Caveats:
- doesn't handling unknown fields, currently they just get ignored
- doesn't support importing, at least for now, as I didn't need it

`krpc-proto` crate contains generated code from mod-supplied `krpc.proto`

`procs.rs` is not a real rust file, it's just a easier-to-read dump of
available procedures from server
