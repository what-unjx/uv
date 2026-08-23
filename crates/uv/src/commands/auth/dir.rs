use std::path::PathBuf;

use owo_colors::OwoColorize;
use std::fmt::Write;

use uv_auth::{PyxTokenStore, Service, TextCredentialStore, is_default_pyx_domain};
use uv_fs::Simplified;
use uv_static::EnvVars;

use crate::printer::Printer;

/// Show the credentials directory.
pub(crate) fn dir(service: Option<&Service>, printer: Printer) -> anyhow::Result<()> {
    if let Some(service) = service {
        let pyx_store = PyxTokenStore::from_settings()?;
        if pyx_store.is_known_domain(service.url()) || is_default_pyx_domain(service.url()) {
            writeln!(
                printer.stdout(),
                "{}",
                pyx_store.root().simplified_display().cyan()
            )?;
            return Ok(());
        }
    }

    // [第1次试飞后修正] 从环境变量读取 UV_HOME，传递给 directory_path
    let uv_home = std::env::var_os(EnvVars::UV_HOME)
        .filter(|s| !s.is_empty())
        .map(PathBuf::from);
    let root = TextCredentialStore::directory_path(uv_home)?;
    writeln!(printer.stdout(), "{}", root.simplified_display().cyan())?;
    Ok(())
}
