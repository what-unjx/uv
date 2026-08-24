use std::io;
use std::path::PathBuf;
use uv_static::EnvVars;

use crate::Cache;
use clap::{Parser, ValueHint};

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
    /// Defaults to `UV_HOME/cache/`.
    ///
    /// When `UV_HOME` is set, this flag takes precedence over the `UV_HOME/cache/` subdirectory.
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
    /// 4. The cache directory configured in `uv.toml` via `cache-dir`.
    ///
    /// Errors if `uv_home` is not set and no explicit directory is provided.
    ///
    /// Returns an absolute cache dir.
    ///
    /// [第3次修正]
    /// 收敛到 UV_HOME：删除 `UV_CACHE_DIR` 环境变量与系统默认目录兜底，未配置 `uv_home` 时直接报错
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
        } else if let Some(cache_dir) = config_cache_dir {
            Ok(Self::from_path(cache_dir))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "UV_HOME is not set; set the `UV_HOME` environment variable or add `home` to `uv.toml`",
            ))
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
    use std::path::Path;

    /// [第3次修正] 验证缓存目录优先级：`--no-cache` > `--cache-dir` > `UV_HOME` > uv.toml `cache-dir`
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
    fn uv_home_used_when_no_flag() {
        let cache =
            Cache::from_settings(false, None, None, Some(PathBuf::from("uv-home"))).unwrap();
        assert_eq!(cache.root(), PathBuf::from("uv-home").join("cache"));
    }

    #[test]
    fn config_cache_dir_used_without_uv_home() {
        let cache =
            Cache::from_settings(false, None, Some(PathBuf::from("config-cache")), None).unwrap();
        assert_eq!(cache.root(), Path::new("config-cache"));
    }

    #[test]
    fn missing_uv_home_errors() {
        let err = Cache::from_settings(false, None, None, None).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn no_cache_takes_precedence_over_all() {
        let cache = Cache::from_settings(
            true,
            Some(PathBuf::from("flag-cache")),
            None,
            Some(PathBuf::from("uv-home")),
        )
        .unwrap();
        assert_ne!(cache.root(), Path::new("flag-cache"));
        assert_ne!(cache.root(), PathBuf::from("uv-home").join("cache"));
    }
}
