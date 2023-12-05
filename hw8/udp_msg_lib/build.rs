extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/proto/thermo_msg.proto"], &["src/proto"]).unwrap();
}
