#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::{
    env, fs,
    io::{self, BufWriter, Write},
    process::{Command, Stdio},
};

use quote::ToTokens;
use syn_serde::json;
use tempfile::Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let (input_path, output_path) = match &args[..] {
        [input] => (input, None),
        [input, output] => (input, Some(output)),
        _ => {
            println!("Usage: rust2pickle <input_path> [output_path]");
            std::process::exit(1);
        }
    };

    let json = fs::read_to_string(&input_path)?;
    let syntax: syn::File = json::from_str(&json)?;

    let outdir = Builder::new().prefix("json2rust").tempdir()?;
    let outfile_path = outdir.path().join("expanded");
    fs::write(&outfile_path, syntax.into_token_stream().to_string())?;

    // Run rustfmt
    let rustfmt_config_path = outdir.path().join(".rustfmt.toml");
    fs::write(rustfmt_config_path, "normalize_doc_attributes = true\n")?;
    // Ignore any errors.
    let _ = Command::new("rustfmt").arg(&outfile_path).stderr(Stdio::null()).status();

    let buf = fs::read(&outfile_path)?;
    if let Some(output_path) = output_path {
        fs::write(output_path, buf)?;
    } else {
        let writer = io::stdout();
        let mut writer = BufWriter::new(writer.lock());
        writer.write_all(&buf)?;
        writer.flush()?;
    }
    Ok(())
}
