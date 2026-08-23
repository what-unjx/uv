use std::fmt::Write;
use std::path::PathBuf;

use anyhow::Context;
use owo_colors::OwoColorize;

use uv_fs::Simplified;
use uv_preview::Preview;
// [第1次试飞后修正] 新增 EnvVars 导入
use uv_static::EnvVars;
use uv_tool::{InstalledTools, tool_executable_dir};

use crate::printer::Printer;

/// Show the tool directory.
pub(crate) fn dir(bin: bool, _preview: Preview, printer: Printer) -> anyhow::Result<()> {
    // [第1次试飞后修正] 从环境变量读取 uv_home
    let uv_home = std::env::var_os(EnvVars::UV_HOME)
        .filter(|s| !s.is_empty())
        .map(PathBuf::from);
    if bin {
        let executable_directory = tool_executable_dir(uv_home.clone())?;
        writeln!(
            printer.stdout(),
            "{}",
            executable_directory.simplified_display().cyan()
        )?;
    } else {
        let installed_tools = InstalledTools::from_settings(uv_home)
            .context("Failed to initialize tools settings")?;
        writeln!(
            printer.stdout(),
            "{}",
            installed_tools.root().simplified_display().cyan()
        )?;
    }

    Ok(())
}
