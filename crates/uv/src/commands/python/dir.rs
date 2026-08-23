use std::fmt::Write;
use std::path::PathBuf;

use anyhow::Context;
use owo_colors::OwoColorize;

use uv_fs::Simplified;
use uv_python::managed::{ManagedPythonInstallations, python_executable_dir};
// [第1次试飞后修正] 新增 EnvVars 导入
use uv_static::EnvVars;

use crate::printer::Printer;

/// Show the Python installation directory.
pub(crate) fn dir(bin: bool, printer: Printer) -> anyhow::Result<()> {
    // [第1次试飞后修正] 从环境变量读取 uv_home
    let uv_home = std::env::var_os(EnvVars::UV_HOME)
        .filter(|s| !s.is_empty())
        .map(PathBuf::from);
    if bin {
        let bin = python_executable_dir(uv_home.clone())?;
        writeln!(printer.stdout(), "{}", bin.simplified_display().cyan())?;
    } else {
        let installed_toolchains = ManagedPythonInstallations::from_settings(None, uv_home)
            .context("Failed to initialize toolchain settings")?;
        writeln!(
            printer.stdout(),
            "{}",
            installed_toolchains.root().simplified_display().cyan()
        )?;
    }

    Ok(())
}
