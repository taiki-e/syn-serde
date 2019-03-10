use failure::{bail, Error};
use quote::ToTokens;
use serde_syn::json;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{self, Command, Stdio};
use tempfile::Builder;

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{}", error);
        process::exit(1);
    }
}

fn try_main() -> Result<(), Error> {
    let mut args = env::args_os();
    let _ = args.next(); // executable name

    let filepath = match (args.next(), args.next()) {
        (Some(arg1), None) => PathBuf::from(arg1),
        _ => bail!("Usage: json2rust path/to/filename.json"),
    };

    let json = fs::read_to_string(&filepath)?;
    let syntax = json::from_str(&json)?;

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

    writeln!(io::stdout(), "{}", fs::read_to_string(&outfile_path)?)?;
    Ok(())
}
