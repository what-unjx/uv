use std::path::PathBuf;

use anyhow::Result;

use uv_python::managed::python_executable_dir;
use uv_static::EnvVars;

use crate::commands::{ExitStatus, update_shell};
use crate::printer::Printer;

/// Ensure that the Python executable directory is in PATH.
pub(crate) async fn update_shell(printer: Printer) -> Result<ExitStatus> {
    // [第1次试飞后修正] 从环境变量读取 uv_home
    let uv_home = std::env::var_os(EnvVars::UV_HOME)
        .filter(|s| !s.is_empty())
        .map(PathBuf::from);
    update_shell::update_shell(&python_executable_dir(uv_home)?, printer).await
}
