use anyhow::{Ok, Result};
use clap::Parser;
use nix::{
    errno::Errno,
    fcntl::{renameat2, RenameFlags},
    sys::stat,
};
use std::{path::Path, path::PathBuf};
use thiserror::Error;

type RexResult = Result<()>;

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

fn status(path: &Path) -> RexResult {
    let path_buf: PathBuf = path.clone().into();
    stat::stat(path)
        .map_err(|errno| RexError::Status {
            path: path_buf.to_string_lossy().into(),
            source: errno,
        })
        .map(|_| ())?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about("atomically swap content of file arguments left and right"),
    long_about = None,
    arg_required_else_help(true),
    propagate_version(true)
)]
struct RexArgs {
    left: PathBuf,
    right: PathBuf,
}

impl RexArgs {
    fn validate(&self) -> RexResult {
        status(&self.left)?;
        status(&self.right)?;
        Ok(())
    }

    fn run(self) -> RexResult {
        self.validate()?;

        renameat2(
            None,
            &self.left,
            None,
            &self.right,
            RenameFlags::RENAME_EXCHANGE,
        )
        .map_err(RexError::Rename)?;

        Ok(())
    }
}

fn main() -> RexResult {
    RexArgs::parse().run()
}
