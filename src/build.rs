extern crate capnpc; // Ensure this crate is included in your Cargo.toml

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let src_prefix = "cereal/";
    let capnp_files = ["log.capnp", "car.capnp", "custom.capnp", "legacy.capnp", "maptile.capnp"];
    let out_dir = Path::new("src/cereal");

    // Create CompilerCommand using a loop to add all files
    let mut command = capnpc::CompilerCommand::new();
    command.src_prefix(src_prefix).output_path(out_dir);
    for file in &capnp_files {
        command.file(format!("{}{}", src_prefix, file));
    }
    command.default_parent_module(vec!["cereal".into()]); // Added semicolon for clarity
    command.run(); // Propagate errors using `?`

    // Create or truncate the mod.rs file
    let mod_rs_path = out_dir.join("mod.rs");
    let mut mod_rs_file = File::create(&mod_rs_path)?;

    // Generate module declarations and write them to mod.rs
    for file in &capnp_files {
        let module_name = file.strip_suffix(".capnp").unwrap_or(file);
        writeln!(mod_rs_file, "pub mod {}_capnp;", module_name)?;
    }

    Ok(())
}
