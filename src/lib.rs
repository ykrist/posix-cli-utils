use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

use anyhow::Error;
pub use anyhow::{anyhow, bail, Context, Result};

pub use clap::{self, ArgEnum, Args, Parser, Subcommand};

pub fn reset_sigpipe() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

pub trait IoContext<T, E>: Context<T, E> + Sized {
    fn context_read<P: AsRef<Path>>(self, path: P) -> Result<T, Error> {
        self.with_context(|| format!("failed to read {}", path.as_ref().display()))
    }

    fn context_write<P: AsRef<Path>>(self, path: P) -> Result<T, Error> {
        self.with_context(|| format!("failed to write {}", path.as_ref().display()))
    }

    fn context_append<P: AsRef<Path>>(self, path: P) -> Result<T, Error> {
        self.with_context(|| format!("failed to append to {}", path.as_ref().display()))
    }
}

impl<R, T, E> IoContext<T, E> for R where R: Context<T, E> {}

pub enum Input<R> {
    File(R),
    Stdin(io::Stdin),
}

impl Input<File> {
    pub fn buffered(self) -> Input<BufReader<File>> {
        match self {
            Input::File(f) => Input::File(BufReader::new(f)),
            Input::Stdin(i) => Input::Stdin(i),
        }
    }

    pub fn default_stdin<P: AsRef<Path>>(path: Option<P>) -> Result<Self> {
        match path {
            Some(ref path) => File::open(path).context_read(path).map(Input::File),
            None => Ok(Input::Stdin(std::io::stdin())),
        }
    }
}

impl<R: Read + 'static> Input<R> {
    pub fn into_dyn_read(self) -> Box<dyn Read> {
        match self {
            Input::File(f) => Box::new(f),
            Input::Stdin(i) => Box::new(i),
        }
    }
}

