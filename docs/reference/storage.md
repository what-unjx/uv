# Storage

## Storage directories

uv uses the following high-level directories for storage.

For each location, uv checks for the existence of environment variables in the given order and uses
the first path found.

The paths of storage directories are platform-specific. uv follows the
[XDG](https://specifications.freedesktop.org/basedir-spec/latest/) conventions on Linux and macOS
and the [Known Folder](https://learn.microsoft.com/en-us/windows/win32/shell/known-folders) scheme
on Windows.

### Unified storage root (`UV_HOME`)

Instead of configuring each storage directory individually, uv supports a unified storage root
directory via the `UV_HOME` environment variable or the `home` setting in `uv.toml`. When set, all
uv storage subdirectories are created under this single root, providing a self-contained and
consistent layout.

**Configuration.** The `UV_HOME` environment variable takes highest precedence. Alternatively, the
same value can be set persistently in the user-level configuration file
(`%APPDATA%\uv\uv.toml` on Windows, `~/.config/uv/uv.toml` on Unix):

```toml title="uv.toml"
home = "/path/to/uv/home"
```

uv requires one of these two to be configured. If neither `UV_HOME` nor the `home` setting is
present, uv will exit with an error instructing the user to set one of them.

**Directory layout.** Under the unified root, uv creates the following structure:

```
UV_HOME/
├── cache/                      # Package cache (equivalent to UV_CACHE_DIR)
│   └── python/                 # Python download cache (equivalent to UV_PYTHON_CACHE_DIR)
├── data/                       # Persistent state root
│   ├── tools/                  # Installed tools (equivalent to UV_TOOL_DIR)
│   ├── python/                 # Managed Python installations (equivalent to UV_PYTHON_INSTALL_DIR)
│   └── credentials/            # Credential storage (equivalent to UV_CREDENTIALS_DIR)
└── bin/                        # Executables (equivalent to UV_TOOL_BIN_DIR and UV_PYTHON_BIN_DIR)
```

**Interaction with individual directory settings.** When `UV_HOME` is set, all individual directory
environment variables (`UV_CACHE_DIR`, `UV_TOOL_DIR`, `UV_TOOL_BIN_DIR`, `UV_PYTHON_INSTALL_DIR`,
`UV_PYTHON_BIN_DIR`, `UV_PYTHON_CACHE_DIR`, `UV_CREDENTIALS_DIR`) are ignored in favor of the
corresponding subdirectory under `UV_HOME`. Command-line flags take precedence over `UV_HOME`: for
the cache, the resolution order is `--no-cache` (use a temporary directory) > `--cache-dir` >
`UV_HOME/cache/` > `UV_CACHE_DIR` > the `cache-dir` setting in `uv.toml` > the platform default.

**Design rationale.** This approach mirrors the convention used by Rust's `cargo` toolchain
(`~/.cargo/` containing `bin/`, `registry/`, and `git/`) and provides several benefits:

- **Self-contained installation.** All uv-managed data lives under a single directory, making it
  easy to relocate, back up, or remove.
- **Consistent PATH setup.** Adding `UV_HOME/bin/` to the system `PATH` gives access to both
  uv-installed tools (`uv tool install`) and managed Python executables (`uv python install`).
- **Simplified configuration.** One setting replaces up to seven individual environment variables.

### Temporary directory

The temporary directory is used for ephemeral data.

=== "Unix"

    1. `$TMPDIR`
    1. `/tmp`

=== "Windows"

    1. `%TMP%`
    1. `%TEMP%`
    1. `%USERPROFILE%`

### Cache directory

The cache directory is used for data that is disposable, but is useful to be long-lived.

=== "Unix"

    1. `$XDG_CACHE_HOME/uv`
    1. `$HOME/.cache/uv`

=== "Windows"

    1. `%LOCALAPPDATA%\uv\cache`
    1. `uv\cache` within [`FOLDERID_LocalAppData`](https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_LocalAppData)

### Persistent data directory

The persistent data directory is used for non-disposable data.

=== "Unix"

    1. `$XDG_DATA_HOME/uv`
    1. `$HOME/.local/share/uv`
    1. `$CWD/.uv`

=== "Windows"

    1. `%APPDATA%\uv\data`
    1. `.\.uv`

### Configuration directories

The configuration directories are used to store changes to uv's settings.

User-level configuration

=== "Unix"

    1. `$XDG_CONFIG_HOME/uv`
    1. `$HOME/.config/uv`

=== "Windows"

    1. `%APPDATA%\uv`
    1. `uv` within [`FOLDERID_RoamingAppData`](https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_RoamingAppData)

System-level configuration

=== "Unix"

    1. `$XDG_CONFIG_DIRS/uv`
    1. `/etc/uv`

=== "Windows"

    1. `%PROGRAMDATA%\uv`
    1. `uv` within [`FOLDERID_AppDataProgramData`](https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid#FOLDERID_AppDataProgramData)

### Executable directory

The executable directory is used to store files that can be run by the user, i.e., a directory that
should be on the `PATH`.

=== "Unix"

    1. `$XDG_BIN_HOME`
    1. `$XDG_DATA_HOME/../bin`
    1. `$HOME/.local/bin`

=== "Windows"

    1. `%XDG_BIN_HOME%`
    1. `%XDG_DATA_HOME%\..\bin`
    1. `%USERPROFILE%\.local\bin`

## Types of data

### Dependency cache

uv uses a local cache to avoid re-downloading and re-building dependencies.

By default, the cache is stored in the [cache directory](#cache-directory) but it can be overridden
via command line arguments, environment variables, or settings as detailed in
[the cache documentation](../concepts/cache.md#cache-directory). When the cache is disabled, the
cache will be stored in a [temporary directory](#temporary-directory).

When the [unified storage root](#unified-storage-root-uv_home) is configured, the cache is placed
at `UV_HOME/cache/` instead, and `UV_CACHE_DIR` is ignored.

Use `uv cache dir` to show the current cache directory path.

!!! important

    For optimal performance, the cache directory needs to be on the same filesystem as virtual
    environments.

### Python versions

uv can install managed [Python versions](../concepts/python-versions.md), e.g., with
`uv python install`.

By default, Python versions managed by uv are stored in a `python/` subdirectory of the
[persistent data directory](#persistent-data-directory), e.g., `~/.local/share/uv/python`.

When the [unified storage root](#unified-storage-root-uv_home) is configured, Python installations
are placed at `UV_HOME/data/python/` instead, and `UV_PYTHON_INSTALL_DIR` is ignored.

Use `uv python dir` to show the Python installation directory.

Use the `UV_PYTHON_INSTALL_DIR` environment variable to override the installation directory.

!!! note

    Changing where Python is installed will not be automatically reflected in existing virtual environments; they will keep referring to the old location, and will need to be updated manually (e.g. by re-creating them).

### Python executables

uv installs executables for [Python versions](#python-versions), e.g., `python3.13`.

By default, Python executables are stored in the [executable directory](#executable-directory).

When the [unified storage root](#unified-storage-root-uv_home) is configured, Python executables
are placed at `UV_HOME/bin/` instead, and both `UV_PYTHON_BIN_DIR` and `UV_TOOL_BIN_DIR` are
ignored.

Use `uv python dir --bin` to show the Python executable directory.

Use the `UV_PYTHON_BIN_DIR` environment variable to override the Python executable directory.

### Tools

uv can install Python packages as [command-line tools](../concepts/tools.md) using
`uv tool install`.

By default, tools are installed in a `tools/` subdirectory of the
[persistent data directory](#persistent-data-directory), e.g., `~/.local/share/uv/tools`.

When the [unified storage root](#unified-storage-root-uv_home) is configured, tools are placed at
`UV_HOME/data/tools/` instead, and `UV_TOOL_DIR` is ignored.

Use `uv tool dir` to show the tool installation directory.

Use the `UV_TOOL_DIR` environment variable to configure the installation directory.

### Tool executables

uv installs executables for installed [tools](#tools), e.g., `ruff`.

By default, tool executables are stored in the [executable directory](#executable-directory).

When the [unified storage root](#unified-storage-root-uv_home) is configured, tool executables are
placed at `UV_HOME/bin/` instead, and both `UV_TOOL_BIN_DIR` and `UV_PYTHON_BIN_DIR` are ignored.

Use `uv tool dir --bin` to show the tool executable directory.

Use the `UV_TOOL_BIN_DIR` environment variable to configure the tool executable directory.

### The uv executable

When using uv's [standalone installer](./installer.md) to install uv, the `uv` and `uvx` executables
are installed into the [executable directory](#executable-directory).

Use the `UV_INSTALL_DIR` environment variable to configure uv's installation directory.

### Configuration files

uv's behavior can be configured through TOML files.

Configuration files are discovered in the [configuration directories](#configuration-directories).

For more details, see the [configuration files documentation](../concepts/configuration-files.md).

### Project virtual environments

When working on [projects](../concepts/projects/index.md), uv creates a dedicated virtual
environment for each project.

By default, project virtual environments are created in `.venv` in the project or workspace root,
i.e., next to the `pyproject.toml`.

Use the `UV_PROJECT_ENVIRONMENT` environment variable to override this location. For more details,
see the
[projects environment documentation](../concepts/projects/config.md#project-environment-path).

With the [`centralized-project-envs` preview feature](../concepts/preview.md), uv stores default
project environments in the [cache directory](#cache-directory). They can be removed by
`uv cache clean` or `uv cache prune` and are recreated when next needed. See the
[centralized project environments](../concepts/projects/layout.md#centralized-project-environments)
documentation for details.

### Script virtual environments

When running [scripts with inline metadata](../guides/scripts.md), uv creates a dedicated virtual
environment for each script in the [cache directory](#cache-directory).
