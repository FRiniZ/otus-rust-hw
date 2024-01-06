extern crate prost_build;

use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["src/proto/smart_message.proto"], &["src/proto"])?;
    prost_build::compile_protos(&["src/proto/cmd_message.proto"], &["src/proto"])?;
    Ok(())
}
