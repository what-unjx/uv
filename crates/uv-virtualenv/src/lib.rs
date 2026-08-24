use std::io;
use std::path::{Path, PathBuf};

use thiserror::Error;

use uv_fs::Simplified;
use uv_python::{Interpreter, PythonEnvironment};

pub use virtualenv::{ClearNonVirtualenv, OnExisting, RemovalReason, Seed};

mod virtualenv;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(
        "Could not find a suitable Python executable for the virtual environment based on the interpreter: {0}"
    )]
    NotFound(String),
    #[error("A {name} already exists at: {}", path.user_display())]
    Exists {
        /// The type of environment (e.g., "virtual environment" or "directory").
        name: &'static str,
        /// The path to the existing environment.
        path: PathBuf,
    },
    #[error("uv will not clear a directory that is not a virtual environment")]
    ClearNonVirtualenv {
        /// The non-virtual environment directory that would have been cleared.
        path: PathBuf,
    },
    #[error("Virtual environment path is not valid UTF-8: {}", path.user_display())]
    NonUtf8Path {
        /// The non-UTF-8 virtual environment path.
        path: PathBuf,
    },
}

impl uv_errors::Hint for Error {
    fn hints(&self) -> uv_errors::Hints<'_> {
        match self {
            Self::Exists { name, .. } => uv_errors::Hints::from(format!(
                "Use the `--clear` flag or set `UV_VENV_CLEAR=1` to replace the existing {name}",
            )),
            Self::ClearNonVirtualenv { .. } => uv_errors::Hints::from(
                "Use the `--force` flag to remove the existing directory anyway",
            ),
            _ => uv_errors::Hints::none(),
        }
    }
}

/// Create a virtualenv.
pub fn create_venv(
    location: &Path,
    interpreter: Interpreter,
    system_site_packages: bool,
    on_existing: OnExisting,
    relocatable: bool,
    seed: Seed,
) -> Result<PythonEnvironment, Error> {
    // Create the virtualenv at the given location.
    let virtualenv = virtualenv::create(
        location,
        &interpreter,
        system_site_packages,
        on_existing,
        relocatable,
        seed,
    )?;

    // Create the corresponding `PythonEnvironment`.
    let interpreter = interpreter.with_virtualenv(virtualenv);
    Ok(PythonEnvironment::from_interpreter(interpreter))
}
