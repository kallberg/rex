use anyhow::Result;
use clap::Parser;
use nix::{
    errno::Errno,
    fcntl::{renameat2, RenameFlags},
    sys::stat,
    sys::stat::FileStat,
};
use std::{path::Path, path::PathBuf};
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    left: PathBuf,
    right: PathBuf,
}

impl Args {
    fn validate(&self) -> Result<()> {
        status(&self.left)?;
        status(&self.right)?;
        Ok(())
    }
}

#[derive(Debug, Error)]
enum RexError {
    #[error("could not get file status for {path:}")]
    Status {
        path: String,
        #[source]
        source: Errno,
    },
    #[error("could not do rename exchange")]
    Rename(#[from] Errno),
}

fn status(path: &Path) -> Result<FileStat> {
    let path_buf: PathBuf = path.clone().into();
    Ok(stat::stat(path).map_err(|errno| RexError::Status {
        path: path_buf.to_string_lossy().into(),
        source: errno,
    })?)
}

fn rex(args: Args) -> Result<()> {
    args.validate()?;

    Ok(renameat2(
        None,
        &args.left,
        None,
        &args.right,
        RenameFlags::RENAME_EXCHANGE,
    )
    .map_err(RexError::Rename)?)
}

fn main() -> Result<()> {
    rex(Args::parse())
}
