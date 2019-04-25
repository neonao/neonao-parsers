use std::process::Command;
use std::fs::File;
use std::io::prelude::*;

fn main() -> ::std::io::Result<()> {
    Command::new("wasm-pack")
        .args(&["build", "--dev"])
        .output()
        .expect("failed to wasm build");
    let path = "./pkg/neonao_parsers.d.ts";
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let new = contents.replace("markdown(source: string): any", "markdown(source: string): Segment[]");
    File::create(path)?.write_all(new.as_bytes())?;
    Ok(())
}
