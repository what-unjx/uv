use serde::Serialize;
use std::collections::BTreeSet;
use std::fmt::Write;
use uv_cli::PythonListFormat;
use uv_pep440::Version;

use anyhow::Result;
use itertools::Either;
use owo_colors::OwoColorize;
use rustc_hash::FxHashSet;
use uv_cache::Cache;
use uv_fs::Simplified;
use uv_python::{
    EnvironmentPreference, PythonPreference, PythonRequest, find_all_python_installations,
};

use crate::commands::ExitStatus;
use crate::printer::Printer;

#[derive(Debug, Serialize)]
struct NamedVersionParts {
    major: u64,
    minor: u64,
    patch: u64,
}

#[derive(Debug, Serialize)]
struct PrintData {
    key: String,
    version: Version,
    version_parts: NamedVersionParts,
    path: Option<String>,
    symlink: Option<String>,
    os: String,
    variant: String,
    implementation: String,
    arch: String,
    libc: String,
}

/// List installed Python installations.
pub(crate) async fn list(
    request: Option<String>,
    all_versions: bool,
    output_format: PythonListFormat,
    python_preference: PythonPreference,
    cache: &Cache,
    printer: Printer,
) -> Result<ExitStatus> {
    let request = request.as_deref().map(PythonRequest::parse);

    let mut output = BTreeSet::new();
    let installations = find_all_python_installations(
        request.as_ref().unwrap_or(&PythonRequest::Any),
        EnvironmentPreference::OnlySystem,
        python_preference,
        cache,
    )?;

    for installation in installations {
        output.insert((
            installation.key(),
            Either::Left(installation.interpreter().real_executable().to_path_buf()),
        ));
    }

    let mut seen_minor = FxHashSet::default();
    let mut seen_patch = FxHashSet::default();
    let mut seen_paths = FxHashSet::default();
    let mut include = Vec::new();
    for (key, uri) in output.iter().rev() {
        // Do not show the same path more than once
        if !seen_paths.insert(uri) {
            continue;
        }

        // Only show the latest patch version for each minor version unless all were requested.
        if let [major, minor, ..] = *key.version().release()
            && !seen_minor.insert((
                major,
                minor,
                key.variant(),
                key.implementation(),
                *key.arch(),
                *key.libc(),
            ))
            && !all_versions
        {
            continue;
        }
        if let [major, minor, patch] = *key.version().release()
            && !seen_patch.insert((
                major,
                minor,
                patch,
                key.variant(),
                key.implementation(),
                *key.arch(),
                key.libc(),
            ))
            && !all_versions
        {
            continue;
        }
        include.push((key, uri));
    }

    match output_format {
        PythonListFormat::Json => {
            let data = include
                .iter()
                .map(|(key, uri)| -> Result<_> {
                    let mut path_or_none: Option<String> = None;
                    let mut symlink_or_none: Option<String> = None;
                    if let Either::Left(path) = uri {
                        path_or_none = Some(path.user_display().to_string());

                        let is_symlink = fs_err::symlink_metadata(path)?.is_symlink();
                        if is_symlink {
                            symlink_or_none =
                                Some(path.read_link()?.user_display().to_string());
                        }
                    }
                    let version = key.version();
                    let release = version.release();

                    Ok(PrintData {
                        key: key.to_string(),
                        version: version.version().clone(),
                        #[expect(clippy::get_first)]
                        version_parts: NamedVersionParts {
                            major: release.get(0).copied().unwrap_or(0),
                            minor: release.get(1).copied().unwrap_or(0),
                            patch: release.get(2).copied().unwrap_or(0),
                        },
                        path: path_or_none,
                        symlink: symlink_or_none,
                        arch: key.arch().to_string(),
                        implementation: key.implementation().to_string(),
                        os: key.os().to_string(),
                        variant: key.variant().to_string(),
                        libc: key.libc().to_string(),
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            writeln!(printer.stdout(), "{}", serde_json::to_string(&data)?)?;
        }
        PythonListFormat::Text => {
            // Compute the width of the first column.
            let width = include
                .iter()
                .fold(0usize, |acc, (key, _)| acc.max(key.to_string().len()));

            for (key, path) in include {
                let key = key.to_string();
                let is_symlink = fs_err::symlink_metadata(path)?.is_symlink();
                if is_symlink {
                    writeln!(
                        printer.stdout(),
                        "{key:width$}    {} -> {}",
                        path.user_display().cyan(),
                        path.read_link()?.user_display().cyan()
                    )?;
                } else {
                    writeln!(
                        printer.stdout(),
                        "{key:width$}    {}",
                        path.user_display().cyan()
                    )?;
                }
            }
        }
    }

    Ok(ExitStatus::Success)
}
