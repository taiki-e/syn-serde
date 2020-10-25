#![warn(rust_2018_idioms, single_use_lifetimes)]

use quote::ToTokens;
use std::{
    fs,
    io::{self, BufWriter, Write},
    path::Path,
    process::{Command, Stdio},
};
use structopt::{clap::AppSettings, StructOpt};
use syn_serde::json;
use tempfile::Builder;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[derive(StructOpt)]
#[structopt(setting = AppSettings::UnifiedHelpMessage)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input_path: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    output_path: Option<std::path::PathBuf>,
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let args = Cli::from_args();

    let json = fs::read_to_string(&args.input_path)?;
    let syntax: syn::File = json::from_str(&json)?;

    let outdir = Builder::new().prefix("json2rust").tempdir()?;
    let outfile_path = outdir.path().join("expanded");
    fs::write(&outfile_path, syntax.into_token_stream().to_string())?;

    // Run rustfmt
    write_rustfmt_config(outdir.path())?;
    // Ignore any errors.
    let _ = Command::new("rustfmt").arg(&outfile_path).stderr(Stdio::null()).status();

    let buf = fs::read(&outfile_path)?;
    if let Some(outpath) = args.output_path {
        fs::write(outpath, buf)?;
    } else {
        let writer = io::stdout();
        let mut writer = BufWriter::new(writer.lock());
        writer.write_all(&buf)?;
        writer.flush()?;
    }
    Ok(())
}

fn write_rustfmt_config(outdir: &Path) -> Result<()> {
    let rustfmt_config_path = outdir.join("rustfmt.toml");
    fs::write(rustfmt_config_path, "normalize_doc_attributes = true\n")?;
    Ok(())
}
