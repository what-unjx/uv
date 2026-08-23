use std::io;
use std::path::{Path, PathBuf};
use uv_static::EnvVars;

use crate::Cache;
use clap::{Parser, ValueHint};
use tracing::{debug, warn};

#[derive(Parser, Debug, Clone)]
#[command(next_help_heading = "Cache options")]
pub struct CacheArgs {
    /// Avoid reading from or writing to the cache, instead using a temporary directory for the
    /// duration of the operation.
    #[arg(
        global = true,
        long,
        short,
        alias = "no-cache-dir",
        env = EnvVars::UV_NO_CACHE,
        value_parser = clap::builder::BoolishValueParser::new(),
    )]
    pub no_cache: bool,

    /// Path to the cache directory.
    ///
    /// Defaults to `$XDG_CACHE_HOME/uv` or `$HOME/.cache/uv` on macOS and Linux, and
    /// `%LOCALAPPDATA%\uv\cache` on Windows.
    ///
    /// When `UV_HOME` is set, this flag takes precedence over the `UV_HOME/cache/` subdirectory,
    /// while the `UV_CACHE_DIR` environment variable is ignored.
    ///
    /// To view the location of the cache directory, run `uv cache dir`.
    #[arg(global = true, long, value_hint = ValueHint::DirPath)]
    pub cache_dir: Option<PathBuf>,
}

impl Cache {
    /// Prefer, in order:
    ///
    /// 1. A temporary cache directory, if the user requested `--no-cache`.
    /// 2. The cache directory specified by the user via the `--cache-dir` command-line flag.
    /// 3. `UV_HOME/cache/` if `uv_home` is set.
    /// 4. The cache directory specified by the user via the `UV_CACHE_DIR` environment variable.
    /// 5. The cache directory configured in `uv.toml` via `cache-dir`.
    /// 6. The system-appropriate cache directory.
    /// 7. A `.uv_cache` directory in the current working directory.
    ///
    /// Returns an absolute cache dir.
    ///
    /// [第2次修正]
    /// 显式传入的 `--cache-dir` 优先于 `UV_HOME`；`UV_CACHE_DIR` 环境变量仅在未设置 `UV_HOME` 时生效
    pub fn from_settings(
        no_cache: bool,
        cache_dir: Option<PathBuf>,
        config_cache_dir: Option<PathBuf>,
        uv_home: Option<PathBuf>,
    ) -> Result<Self, io::Error> {
        if no_cache {
            Self::temp()
        } else if let Some(cache_dir) = cache_dir {
            Ok(Self::from_path(cache_dir))
        } else if let Some(uv_home) = uv_home {
            Ok(Self::from_path(uv_home.join("cache")))
        } else if let Some(cache_dir) = std::env::var_os(EnvVars::UV_CACHE_DIR)
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
        {
            Ok(Self::from_path(cache_dir))
        } else if let Some(cache_dir) = config_cache_dir {
            Ok(Self::from_path(cache_dir))
        } else if let Some(cache_dir) = uv_dirs::legacy_user_cache_dir().filter(|dir| dir.exists())
        {
            // If the user has an existing directory at (e.g.) `/Users/user/Library/Caches/uv`,
            // respect it for backwards compatibility. Otherwise, prefer the XDG strategy, even on
            // macOS.
            Ok(Self::from_path(cache_dir))
        } else if let Some(cache_dir) = uv_dirs::user_cache_dir() {
            if cfg!(windows) {
                // On Windows, we append `cache` to the LocalAppData directory, i.e., prefer
                // `C:\Users\User\AppData\Local\uv\cache` over `C:\Users\User\AppData\Local\uv`.
                //
                // Unfortunately, v0.3.0 and v0.3.1 used the latter, so we need to migrate the cache
                // for those users.
                let destination = cache_dir.join("cache");
                let source = cache_dir;
                if let Err(err) = migrate_windows_cache(&source, &destination) {
                    warn!(
                        "Failed to migrate cache from `{}` to `{}`: {err}",
                        source.display(),
                        destination.display()
                    );
                }

                Ok(Self::from_path(destination))
            } else {
                Ok(Self::from_path(cache_dir))
            }
        } else {
            Ok(Self::from_path(".uv_cache"))
        }
    }
}

impl TryFrom<CacheArgs> for Cache {
    type Error = io::Error;

    fn try_from(value: CacheArgs) -> Result<Self, Self::Error> {
        // [第1次试飞后修正] uv_home 从全局配置解析，这里通过 CacheArgs 无法获取，传 None
        Self::from_settings(value.no_cache, value.cache_dir, None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [第2次修正] 验证缓存目录优先级：`--no-cache` > `--cache-dir` > `UV_HOME` > `UV_CACHE_DIR`
    #[test]
    fn cache_dir_flag_takes_precedence_over_uv_home() {
        let cache = Cache::from_settings(
            false,
            Some(PathBuf::from("flag-cache")),
            None,
            Some(PathBuf::from("uv-home")),
        )
        .unwrap();
        assert_eq!(cache.root(), Path::new("flag-cache"));
    }

    #[test]
    fn uv_home_takes_precedence_over_uv_cache_dir_env() {
        temp_env::with_var(EnvVars::UV_CACHE_DIR, Some("env-cache"), || {
            let cache =
                Cache::from_settings(false, None, None, Some(PathBuf::from("uv-home"))).unwrap();
            assert_eq!(cache.root(), PathBuf::from("uv-home").join("cache"));
        });
    }

    #[test]
    fn uv_cache_dir_env_used_without_uv_home() {
        temp_env::with_var(EnvVars::UV_CACHE_DIR, Some("env-cache"), || {
            let cache = Cache::from_settings(false, None, None, None).unwrap();
            assert_eq!(cache.root(), Path::new("env-cache"));
        });
    }

    #[test]
    fn no_cache_takes_precedence_over_all() {
        temp_env::with_var(EnvVars::UV_CACHE_DIR, Some("env-cache"), || {
            let cache = Cache::from_settings(
                true,
                Some(PathBuf::from("flag-cache")),
                None,
                Some(PathBuf::from("uv-home")),
            )
            .unwrap();
            assert_ne!(cache.root(), Path::new("flag-cache"));
            assert_ne!(cache.root(), PathBuf::from("uv-home").join("cache"));
        });
    }
}

/// Migrate the Windows cache from `C:\Users\User\AppData\Local\uv` to `C:\Users\User\AppData\Local\uv\cache`.
fn migrate_windows_cache(source: &Path, destination: &Path) -> Result<(), io::Error> {
    // The list of expected cache buckets in v0.3.0.
    for directory in [
        "built-wheels-v3",
        "flat-index-v0",
        "git-v0",
        "interpreter-v2",
        "simple-v12",
        "wheels-v1",
        "archive-v0",
        "builds-v0",
        "environments-v1",
    ] {
        let source = source.join(directory);
        let destination = destination.join(directory);

        // Migrate the cache bucket.
        if source.exists() {
            debug!(
                "Migrating cache bucket from {} to {}",
                source.display(),
                destination.display()
            );
            if let Some(parent) = destination.parent() {
                fs_err::create_dir_all(parent)?;
            }
            fs_err::rename(&source, &destination)?;
        }
    }

    // The list of expected cache files in v0.3.0.
    for file in [".gitignore", "CACHEDIR.TAG"] {
        let source = source.join(file);
        let destination = destination.join(file);

        // Migrate the cache file.
        if source.exists() {
            debug!(
                "Migrating cache file from {} to {}",
                source.display(),
                destination.display()
            );
            if let Some(parent) = destination.parent() {
                fs_err::create_dir_all(parent)?;
            }
            fs_err::rename(&source, &destination)?;
        }
    }

    Ok(())
}
