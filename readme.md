# MIDI KSP Controller

A Rust program that allows to control Kerbal Space Program game (using KRPC mod)
by way of a MIDI controller (only works with M-Audio Axiom Air Mini 32).

`protobuf-but-worse` is WIP protobuf codegen crate, created because exisiting
crates weren't suitable for me. Pure Rust, for parsing `.proto` files 
uses protobuf-parser, a nom 3 parser; codegen using `syn` & co. crates.

Non-goal:
- Handling unknown fields, currently they just get ignored
- 

`krpc-proto` crate contains generated code from supplied `krpc.proto`
