extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/protos/kmessages.proto"], &["src/protos"]).unwrap();
}
