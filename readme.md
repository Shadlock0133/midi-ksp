# MIDI KSP Controller

A Rust program that allows to control Kerbal Space Program game (using KRPC mod)
by way of a MIDI controller (only works with M-Audio Axiom Air Mini 32).

`protobuf-but-worse` is opinionated WIP protobuf codegen crate, created because
exisiting crates weren't suitable for me. Doesn't support all of protobufs
features, I only implemented what needed for krpc.

Pure Rust, using crates:
- `protobuf-parser`, a nom 3 parser, for reading `.proto` files
- `syn` & co. for codegen

Caveats:
- doesn't handle unknown fields, currently they just get ignored
- doesn't support importing, at least for now, as I didn't need it
- non-required fields are wrapped in `Option`. Kinda annoying but correct-er
- because in protobuf default values don't have to be sent, you might/will get
None for default values like `false` bool, `0` int or `Foo = 0` enum variant,
possibly panic on required field
- contrary to above, we do send default values.
Specs don't say that's illegal so `¯\_(ツ)_/¯`

`krpc` wraps generated protobuf code and provides more rusty interface. Very WIP

`krpc-proto` crate contains generated code from mod-supplied `krpc.proto`

`procs.rs` is not a real rust file, it's just a easier-to-read dump of
available procedures from server. Methods' real names are `Class_Proc`, so
for `impl Control { fn get_Gear(&self) -> Bool; ]`,
real sig is `Control_get_Gear(this: Class) -> Bool`
