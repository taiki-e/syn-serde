#![warn(rust_2018_idioms)]

use quote::ToTokens;
use std::{
    env, fs,
    io::{self, BufWriter, Write},
    path::PathBuf,
    process::{Command, Stdio},
};
use syn_serde::json;
use tempfile::Builder;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let mut args = env::args_os();
    let _ = args.next(); // executable name

    let filepath = match (args.next(), args.next()) {
        (Some(arg1), None) => PathBuf::from(arg1),
        _ => {
            println!("Usage: rust2json path/to/filename.rs");
            return Ok(());
        }
    };

    let json = fs::read_to_string(&filepath)?;
    let syntax: syn::File = json::from_str(&json)?;

    let outdir = Builder::new().prefix("json2rust").tempdir()?;
    let outfile_path = outdir.path().join("expanded");
    fs::write(&outfile_path, syntax.into_token_stream().to_string())?;

    // Run rustfmt
    // https://github.com/dtolnay/cargo-expand/blob/0.4.9/src/main.rs#L181-L182
    let rustfmt_config_path = outdir.path().join("rustfmt.toml");
    fs::write(rustfmt_config_path, "normalize_doc_attributes = true\n")?;

    // Ignore any errors.
    let _status = Command::new("rustfmt")
        .arg(&outfile_path)
        .stderr(Stdio::null())
        .status();

    let writer = io::stdout();
    let mut writer = BufWriter::new(writer.lock());
    writer.write_all(fs::read_to_string(&outfile_path)?.as_bytes())?;
    writer.flush()?;
    Ok(())
}
