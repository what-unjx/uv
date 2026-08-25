use uv_test::uv_snapshot;

#[test]
fn cert_is_limited_to_pip() {
    let context = uv_test::test_context_with_versions!(&[]);

    uv_snapshot!(context.filters(), context.command()
        .arg("sync")
        .arg("--cert")
        .arg("ca-bundle.pem"), @"
    exit_code: 2 (failure)
    ----- stderr -----
    error: unexpected argument '--cert' found

      tip: a similar argument exists: '--script'

    Usage: uv sync --script <SCRIPT>

    For more information, try '--help'.
    ");
}

#[test]
fn help() {
    let context = uv_test::test_context_with_versions!(&[]);

    // The `uv help` command should show the long help message
    uv_snapshot!(context.filters(), context.help(), @"
    exit_code: 0 (success)
    ----- stdout -----
    An extremely fast Python package manager.

    Usage: uv [OPTIONS] <COMMAND>

    Commands:
      auth                       Manage authentication
      run                        Run a command or script
      init                       Create a new project
      add                        Add dependencies to the project
      remove                     Remove dependencies from the project
      version                    Read or update the project's version
      sync                       Update the project's environment
      lock                       Update the project's lockfile
      export                     Export the project's lockfile to an alternate format
      tree                       Display the project's dependency tree
      format                     Format Python code in the project
      check                      Run checks on the project
      audit                      Audit the project's dependencies
      tool                       Run and install commands provided by Python packages
      python                     Manage Python versions and installations
      pip                        Manage Python packages with a pip-compatible interface
      venv                       Create a virtual environment
      build                      Build Python packages into source distributions and wheels
      publish                    Upload distributions to an index
      workspace                  Inspect uv workspaces
      cache                      Manage uv's cache
      self                       Manage the uv executable
      generate-shell-completion  Generate shell completion
      help                       Display documentation for a command

    Cache options:
      -n, --no-cache               Avoid reading from or writing to the cache, instead using a temporary
                                   directory for the duration of the operation [env: UV_NO_CACHE=]
          --cache-dir [CACHE_DIR]  Path to the cache directory

    Global options:
      -q, --quiet...
              Use quiet output
      -v, --verbose...
              Use verbose output
          --color <COLOR_CHOICE>
              Control the use of color in output [possible values: auto, always, never]
          --system-certs
              Whether to load TLS certificates from the platform's native certificate store [env:
              UV_SYSTEM_CERTS=]
          --offline
              Disable network access [env: UV_OFFLINE=]
          --allow-insecure-host <ALLOW_INSECURE_HOST>
              Allow insecure connections to a host [env: UV_INSECURE_HOST=]
          --no-progress
              Hide all progress outputs [env: UV_NO_PROGRESS=]
          --directory <DIRECTORY>
              Change to the given directory prior to running the command [env: UV_WORKING_DIR=]
          --project <PROJECT>
              Discover a project in the given directory [env: UV_PROJECT=]
          --config-file <CONFIG_FILE>
              The path to a `uv.toml` file to use for configuration [env: UV_CONFIG_FILE=]
          --no-config
              Avoid discovering configuration files (`pyproject.toml`, `uv.toml`) [env: UV_NO_CONFIG=]
      -h, --help
              Display the concise help for this command
      -V, --version
              Display the uv version

    Use `uv help <command>` for more information on a specific command.
    ");
}

#[test]
fn help_flag() {
    let context = uv_test::test_context_with_versions!(&[]);

    uv_snapshot!(context.filters(), context.command().arg("--help"), @"
    exit_code: 0 (success)
    ----- stdout -----
    An extremely fast Python package manager.

    Usage: uv [OPTIONS] <COMMAND>

    Commands:
      auth       Manage authentication
      run        Run a command or script
      init       Create a new project
      add        Add dependencies to the project
      remove     Remove dependencies from the project
      version    Read or update the project's version
      sync       Update the project's environment
      lock       Update the project's lockfile
      export     Export the project's lockfile to an alternate format
      tree       Display the project's dependency tree
      format     Format Python code in the project
      check      Run checks on the project
      audit      Audit the project's dependencies
      tool       Run and install commands provided by Python packages
      python     Manage Python versions and installations
      pip        Manage Python packages with a pip-compatible interface
      venv       Create a virtual environment
      build      Build Python packages into source distributions and wheels
      publish    Upload distributions to an index
      workspace  Inspect uv workspaces
      cache      Manage uv's cache
      self       Manage the uv executable
      help       Display documentation for a command

    Cache options:
      -n, --no-cache               Avoid reading from or writing to the cache, instead using a temporary
                                   directory for the duration of the operation [env: UV_NO_CACHE=]
          --cache-dir [CACHE_DIR]  Path to the cache directory

    Global options:
      -q, --quiet...
              Use quiet output
      -v, --verbose...
              Use verbose output
          --color <COLOR_CHOICE>
              Control the use of color in output [possible values: auto, always, never]
          --system-certs
              Whether to load TLS certificates from the platform's native certificate store [env:
              UV_SYSTEM_CERTS=]
          --offline
              Disable network access [env: UV_OFFLINE=]
          --allow-insecure-host <ALLOW_INSECURE_HOST>
              Allow insecure connections to a host [env: UV_INSECURE_HOST=]
          --no-progress
              Hide all progress outputs [env: UV_NO_PROGRESS=]
          --directory <DIRECTORY>
              Change to the given directory prior to running the command [env: UV_WORKING_DIR=]
          --project <PROJECT>
              Discover a project in the given directory [env: UV_PROJECT=]
          --config-file <CONFIG_FILE>
              The path to a `uv.toml` file to use for configuration [env: UV_CONFIG_FILE=]
          --no-config
              Avoid discovering configuration files (`pyproject.toml`, `uv.toml`) [env: UV_NO_CONFIG=]
      -h, --help
              Display the concise help for this command
      -V, --version
              Display the uv version

    Use `uv help` for more details.
    ");
}

#[test]
fn help_short_flag() {
    let context = uv_test::test_context_with_versions!(&[]);

    uv_snapshot!(context.filters(), context.command().arg("-h"), @"
    exit_code: 0 (success)
    ----- stdout -----
    An extremely fast Python package manager.

    Usage: uv [OPTIONS] <COMMAND>

    Commands:
      auth       Manage authentication
      run        Run a command or script
      init       Create a new project
      add        Add dependencies to the project
      remove     Remove dependencies from the project
      version    Read or update the project's version
      sync       Update the project's environment
      lock       Update the project's lockfile
      export     Export the project's lockfile to an alternate format
      tree       Display the project's dependency tree
      format     Format Python code in the project
      check      Run checks on the project
      audit      Audit the project's dependencies
      tool       Run and install commands provided by Python packages
      python     Manage Python versions and installations
      pip        Manage Python packages with a pip-compatible interface
      venv       Create a virtual environment
      build      Build Python packages into source distributions and wheels
      publish    Upload distributions to an index
      workspace  Inspect uv workspaces
      cache      Manage uv's cache
      self       Manage the uv executable
      help       Display documentation for a command

    Cache options:
      -n, --no-cache               Avoid reading from or writing to the cache, instead using a temporary
                                   directory for the duration of the operation [env: UV_NO_CACHE=]
          --cache-dir [CACHE_DIR]  Path to the cache directory

    Global options:
      -q, --quiet...
              Use quiet output
      -v, --verbose...
              Use verbose output
          --color <COLOR_CHOICE>
              Control the use of color in output [possible values: auto, always, never]
          --system-certs
              Whether to load TLS certificates from the platform's native certificate store [env:
              UV_SYSTEM_CERTS=]
          --offline
              Disable network access [env: UV_OFFLINE=]
          --allow-insecure-host <ALLOW_INSECURE_HOST>
              Allow insecure connections to a host [env: UV_INSECURE_HOST=]
          --no-progress
              Hide all progress outputs [env: UV_NO_PROGRESS=]
          --directory <DIRECTORY>
              Change to the given directory prior to running the command [env: UV_WORKING_DIR=]
          --project <PROJECT>
              Discover a project in the given directory [env: UV_PROJECT=]
          --config-file <CONFIG_FILE>
              The path to a `uv.toml` file to use for configuration [env: UV_CONFIG_FILE=]
          --no-config
              Avoid discovering configuration files (`pyproject.toml`, `uv.toml`) [env: UV_NO_CONFIG=]
      -h, --help
              Display the concise help for this command
      -V, --version
              Display the uv version

    Use `uv help` for more details.
    ");
}

#[test]
fn help_flag_workspace() {
    let context = uv_test::test_context_with_versions!(&[]);

    uv_snapshot!(context.filters(), context.command().arg("workspace").arg("--help"), @"
    exit_code: 0 (success)
    ----- stdout -----
    Inspect uv workspaces

    Usage: uv workspace [OPTIONS] <COMMAND>

    Commands:
      metadata  View metadata about the current workspace
      dir       Display the path of a workspace member
      list      List the members of a workspace

    Cache options:
      -n, --no-cache               Avoid reading from or writing to the cache, instead using a temporary
                                   directory for the duration of the operation [env: UV_NO_CACHE=]
          --cache-dir [CACHE_DIR]  Path to the cache directory

    Global options:
      -q, --quiet...
              Use quiet output
      -v, --verbose...
              Use verbose output
          --color <COLOR_CHOICE>
              Control the use of color in output [possible values: auto, always, never]
          --system-certs
              Whether to load TLS certificates from the platform's native certificate store [env:
              UV_SYSTEM_CERTS=]
          --offline
              Disable network access [env: UV_OFFLINE=]
          --allow-insecure-host <ALLOW_INSECURE_HOST>
              Allow insecure connections to a host [env: UV_INSECURE_HOST=]
          --no-progress
              Hide all progress outputs [env: UV_NO_PROGRESS=]
          --directory <DIRECTORY>
              Change to the given directory prior to running the command [env: UV_WORKING_DIR=]
          --project <PROJECT>
              Discover a project in the given directory [env: UV_PROJECT=]
          --config-file <CONFIG_FILE>
              The path to a `uv.toml` file to use for configuration [env: UV_CONFIG_FILE=]
          --no-config
              Avoid discovering configuration files (`pyproject.toml`, `uv.toml`) [env: UV_NO_CONFIG=]
      -h, --help
              Display the concise help for this command

    Use `uv help workspace` for more details.
    ");
}

#[test]
fn help_subcommand() {
    let context = uv_test::test_context_with_versions!(&[]);

    uv_snapshot!(context.filters(), context.help().arg("python"), @"
    exit_code: 0 (success)
    ----- stdout -----
    Manage Python versions and installations

    Generally, uv first searches for Python in a virtual environment, either active or in a
    `.venv` directory in the current working directory or any parent directory. If a virtual
    environment is not required, uv will then search for a Python interpreter. Python
    interpreters are found by searching for Python executables in the `PATH` environment
    variable.

    On Windows, the registry is also searched for Python executables.


    The `--python` option allows requesting a different interpreter.

    The following Python version request formats are supported:

    - `<version>` e.g. `3`, `3.12`, `3.12.3`
    - `<version-specifier>` e.g. `>=3.12,<3.13`
    - `<version><short-variant>` (e.g., `3.13t`, `3.12.0d`)
    - `<version>+<variant>` (e.g., `3.13+freethreaded`, `3.12.0+debug`)
    - `<implementation>` e.g. `cpython` or `cp`
    - `<implementation>@<version>` e.g. `cpython@3.12`
    - `<implementation><version>` e.g. `cpython3.12` or `cp312`
    - `<implementation><version-specifier>` e.g. `cpython>=3.12,<3.13`
    - `<implementation>-<version>-<os>-<arch>-<libc>` e.g. `cpython-3.12.3-macos-aarch64-none`

    Additionally, a specific system Python interpreter can often be requested with:

    - `<executable-path>` e.g. `/opt/homebrew/bin/python3`
    - `<executable-name>` e.g. `mypython3`
    - `<install-dir>` e.g. `/some/environment/`

    When the `--python` option is used, normal discovery rules apply but discovered interpreters
    are checked for compatibility with the request, e.g., if `pypy` is requested, uv will first
    check if the virtual environment contains a PyPy interpreter then check if each executable
    in the path is a PyPy interpreter.

    uv supports discovering CPython, PyPy, and GraalPy interpreters. Unsupported interpreters
    will be skipped during discovery. If an unsupported interpreter implementation is requested,
    uv will exit with an error.

    Usage: uv python [OPTIONS] <COMMAND>

    Commands:
      list  List the available Python installations
      find  Search for a Python installation
      pin   Pin to a specific Python version

    Cache options:
      -n, --no-cache
              Avoid reading from or writing to the cache, instead using a temporary directory for the
              duration of the operation

              [env: UV_NO_CACHE=]

          --cache-dir [CACHE_DIR]
              Path to the cache directory.

              Defaults to `UV_HOME/cache/`.

              When `UV_HOME` is set, this flag takes precedence over the `UV_HOME/cache/` subdirectory.

              To view the location of the cache directory, run `uv cache dir`.

    Global options:
      -q, --quiet...
              Use quiet output.

              Repeating this option, e.g., `-qq`, will enable a silent mode in which uv will write no
              output to stdout.

      -v, --verbose...
              Use verbose output.

              You can configure fine-grained logging using the `RUST_LOG` environment variable.
              (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)

          --color <COLOR_CHOICE>
              Control the use of color in output.

              By default, uv will automatically detect support for colors when writing to a terminal.

              Possible values:
              - auto:   Enables colored output only when the output is going to a terminal or TTY with
                support
              - always: Enables colored output regardless of the detected environment
              - never:  Disables colored output

          --system-certs
              Whether to load TLS certificates from the platform's native certificate store [env:
              UV_SYSTEM_CERTS=]

              By default, uv uses bundled Mozilla root certificates, which improves portability and
              performance (especially on macOS).

              However, in some cases, you may want to use the platform's native certificate store,
              especially if you're relying on a corporate trust root (e.g., for a mandatory proxy)
              that's included in your system's certificate store.

          --offline
              Disable network access.

              When disabled, uv will only use locally cached data and locally available files.

              [env: UV_OFFLINE=]

          --allow-insecure-host <ALLOW_INSECURE_HOST>
              Allow insecure connections to a host.

              Can be provided multiple times.

              Expects to receive either a hostname (e.g., `localhost`), a host-port pair (e.g.,
              `localhost:8080`), or a URL (e.g., `https://localhost`).

              WARNING: Hosts included in this list will not be verified against the system's certificate
              store. Only use `--allow-insecure-host` in a secure network with verified sources, as it
              bypasses SSL verification and could expose you to MITM attacks.

              [env: UV_INSECURE_HOST=]

          --no-progress
              Hide all progress outputs.

              For example, spinners or progress bars.

              [env: UV_NO_PROGRESS=]

          --directory <DIRECTORY>
              Change to the given directory prior to running the command.

              Relative paths are resolved with the given directory as the base.

              See `--project` to only change the project root directory.

              [env: UV_WORKING_DIR=]

          --project <PROJECT>
              Discover a project in the given directory.

              All `pyproject.toml`, `uv.toml`, and `.python-version` files will be discovered by walking
              up the directory tree from the project root, as will the project's virtual environment
              (`.venv`).

              Other command-line arguments (such as relative paths) will be resolved relative to the
              current working directory.

              See `--directory` to change the working directory entirely.

              This setting has no effect when used in the `uv pip` interface.

              [env: UV_PROJECT=]

          --config-file <CONFIG_FILE>
              The path to a `uv.toml` file to use for configuration.

              While uv configuration can be included in a `pyproject.toml` file, it is not allowed in
              this context.

              [env: UV_CONFIG_FILE=]

          --no-config
              Avoid discovering configuration files (`pyproject.toml`, `uv.toml`).

              Normally, configuration files are discovered in the current directory, parent directories,
              or user configuration directories.

              [env: UV_NO_CONFIG=]

      -h, --help
              Display the concise help for this command

    Use `uv help python <command>` for more information on a specific command.
    ");
}

#[test]
fn help_subsubcommand() {
    let context = uv_test::test_context_with_versions!(&[]);

    uv_snapshot!(context.filters(), context.help().arg("python").arg("install"), @"
    exit_code: 2 (failure)
    ----- stderr -----
    error: There is no command `install` for `uv python`. Did you mean one of:
        list
        find
        pin
    ");
}

#[test]
fn help_flag_subcommand() {
    let context = uv_test::test_context_with_versions!(&[]);

    uv_snapshot!(context.filters(), context.command().arg("python").arg("--help"), @"
    exit_code: 0 (success)
    ----- stdout -----
    Manage Python versions and installations

    Usage: uv python [OPTIONS] <COMMAND>

    Commands:
      list  List the available Python installations
      find  Search for a Python installation
      pin   Pin to a specific Python version

    Cache options:
      -n, --no-cache               Avoid reading from or writing to the cache, instead using a temporary
                                   directory for the duration of the operation [env: UV_NO_CACHE=]
          --cache-dir [CACHE_DIR]  Path to the cache directory

    Global options:
      -q, --quiet...
              Use quiet output
      -v, --verbose...
              Use verbose output
          --color <COLOR_CHOICE>
              Control the use of color in output [possible values: auto, always, never]
          --system-certs
              Whether to load TLS certificates from the platform's native certificate store [env:
              UV_SYSTEM_CERTS=]
          --offline
              Disable network access [env: UV_OFFLINE=]
          --allow-insecure-host <ALLOW_INSECURE_HOST>
              Allow insecure connections to a host [env: UV_INSECURE_HOST=]
          --no-progress
              Hide all progress outputs [env: UV_NO_PROGRESS=]
          --directory <DIRECTORY>
              Change to the given directory prior to running the command [env: UV_WORKING_DIR=]
          --project <PROJECT>
              Discover a project in the given directory [env: UV_PROJECT=]
          --config-file <CONFIG_FILE>
              The path to a `uv.toml` file to use for configuration [env: UV_CONFIG_FILE=]
          --no-config
              Avoid discovering configuration files (`pyproject.toml`, `uv.toml`) [env: UV_NO_CONFIG=]
      -h, --help
              Display the concise help for this command

    Use `uv help python` for more details.
    ");
}

#[test]
fn help_flag_subsubcommand() {
    let context = uv_test::test_context_with_versions!(&[]);

    uv_snapshot!(context.filters(), context.command().arg("python").arg("install").arg("--help"), @"
    exit_code: 2 (failure)
    ----- stderr -----
    error: unrecognized subcommand 'install'

      tip: a similar subcommand exists: 'uv pip install'

    Usage: uv python [OPTIONS] <COMMAND>

    For more information, try '--help'.
    ");
}

#[test]
fn help_unknown_subcommand() {
    let context = uv_test::test_context_with_versions!(&[]);

    uv_snapshot!(context.filters(), context.help().arg("foobar"), @"
    exit_code: 2 (failure)
    ----- stderr -----
    error: There is no command `foobar` for `uv`. Did you mean one of:
        auth
        run
        init
        add
        remove
        version
        sync
        lock
        export
        tree
        format
        check
        audit
        tool
        python
        pip
        venv
        build
        publish
        workspace
        cache
        self
        generate-shell-completion
    ");

    uv_snapshot!(context.filters(), context.help().arg("foo").arg("bar"), @"
    exit_code: 2 (failure)
    ----- stderr -----
    error: There is no command `foo bar` for `uv`. Did you mean one of:
        auth
        run
        init
        add
        remove
        version
        sync
        lock
        export
        tree
        format
        check
        audit
        tool
        python
        pip
        venv
        build
        publish
        workspace
        cache
        self
        generate-shell-completion
    ");
}

#[test]
fn help_unknown_subsubcommand() {
    let context = uv_test::test_context_with_versions!(&[]);

    uv_snapshot!(context.filters(), context.help().arg("python").arg("foobar"), @"
    exit_code: 2 (failure)
    ----- stderr -----
    error: There is no command `foobar` for `uv python`. Did you mean one of:
        list
        find
        pin
    ");
}

#[test]
fn help_with_global_option() {
    let context = uv_test::test_context_with_versions!(&[]);

    uv_snapshot!(context.filters(), context.help().arg("--no-cache"), @"
    exit_code: 0 (success)
    ----- stdout -----
    An extremely fast Python package manager.

    Usage: uv [OPTIONS] <COMMAND>

    Commands:
      auth                       Manage authentication
      run                        Run a command or script
      init                       Create a new project
      add                        Add dependencies to the project
      remove                     Remove dependencies from the project
      version                    Read or update the project's version
      sync                       Update the project's environment
      lock                       Update the project's lockfile
      export                     Export the project's lockfile to an alternate format
      tree                       Display the project's dependency tree
      format                     Format Python code in the project
      check                      Run checks on the project
      audit                      Audit the project's dependencies
      tool                       Run and install commands provided by Python packages
      python                     Manage Python versions and installations
      pip                        Manage Python packages with a pip-compatible interface
      venv                       Create a virtual environment
      build                      Build Python packages into source distributions and wheels
      publish                    Upload distributions to an index
      workspace                  Inspect uv workspaces
      cache                      Manage uv's cache
      self                       Manage the uv executable
      generate-shell-completion  Generate shell completion
      help                       Display documentation for a command

    Cache options:
      -n, --no-cache               Avoid reading from or writing to the cache, instead using a temporary
                                   directory for the duration of the operation [env: UV_NO_CACHE=]
          --cache-dir [CACHE_DIR]  Path to the cache directory

    Global options:
      -q, --quiet...
              Use quiet output
      -v, --verbose...
              Use verbose output
          --color <COLOR_CHOICE>
              Control the use of color in output [possible values: auto, always, never]
          --system-certs
              Whether to load TLS certificates from the platform's native certificate store [env:
              UV_SYSTEM_CERTS=]
          --offline
              Disable network access [env: UV_OFFLINE=]
          --allow-insecure-host <ALLOW_INSECURE_HOST>
              Allow insecure connections to a host [env: UV_INSECURE_HOST=]
          --no-progress
              Hide all progress outputs [env: UV_NO_PROGRESS=]
          --directory <DIRECTORY>
              Change to the given directory prior to running the command [env: UV_WORKING_DIR=]
          --project <PROJECT>
              Discover a project in the given directory [env: UV_PROJECT=]
          --config-file <CONFIG_FILE>
              The path to a `uv.toml` file to use for configuration [env: UV_CONFIG_FILE=]
          --no-config
              Avoid discovering configuration files (`pyproject.toml`, `uv.toml`) [env: UV_NO_CONFIG=]
      -h, --help
              Display the concise help for this command
      -V, --version
              Display the uv version

    Use `uv help <command>` for more information on a specific command.
    ");
}

#[test]
fn help_with_help() {
    let context = uv_test::test_context_with_versions!(&[]);

    uv_snapshot!(context.filters(), context.help().arg("--help"), @"
    exit_code: 0 (success)
    ----- stdout -----
    Display documentation for a command

    Usage: uv help [OPTIONS] [COMMAND]...

    Options:
      --no-pager Disable pager when printing help
    ");
}

#[test]
fn help_with_version() {
    let context = uv_test::test_context_with_versions!(&[]);

    uv_snapshot!(context.filters(), context.help().arg("--version"), @"
    exit_code: 2 (failure)
    ----- stderr -----
    error: unexpected argument '--version' found

      tip: a similar argument exists: '--verbose'

    Usage: uv help --verbose... [COMMAND]...

    For more information, try '--help'.
    ");
}

#[test]
fn help_with_no_pager() {
    let context = uv_test::test_context_with_versions!(&[]);

    // We can't really test whether the --no-pager option works with a snapshot test.
    // It's still nice to have a test for the option to confirm the option exists.
    uv_snapshot!(context.filters(), context.help().arg("--no-pager"), @"
    exit_code: 0 (success)
    ----- stdout -----
    An extremely fast Python package manager.

    Usage: uv [OPTIONS] <COMMAND>

    Commands:
      auth                       Manage authentication
      run                        Run a command or script
      init                       Create a new project
      add                        Add dependencies to the project
      remove                     Remove dependencies from the project
      version                    Read or update the project's version
      sync                       Update the project's environment
      lock                       Update the project's lockfile
      export                     Export the project's lockfile to an alternate format
      tree                       Display the project's dependency tree
      format                     Format Python code in the project
      check                      Run checks on the project
      audit                      Audit the project's dependencies
      tool                       Run and install commands provided by Python packages
      python                     Manage Python versions and installations
      pip                        Manage Python packages with a pip-compatible interface
      venv                       Create a virtual environment
      build                      Build Python packages into source distributions and wheels
      publish                    Upload distributions to an index
      workspace                  Inspect uv workspaces
      cache                      Manage uv's cache
      self                       Manage the uv executable
      generate-shell-completion  Generate shell completion
      help                       Display documentation for a command

    Cache options:
      -n, --no-cache               Avoid reading from or writing to the cache, instead using a temporary
                                   directory for the duration of the operation [env: UV_NO_CACHE=]
          --cache-dir [CACHE_DIR]  Path to the cache directory

    Global options:
      -q, --quiet...
              Use quiet output
      -v, --verbose...
              Use verbose output
          --color <COLOR_CHOICE>
              Control the use of color in output [possible values: auto, always, never]
          --system-certs
              Whether to load TLS certificates from the platform's native certificate store [env:
              UV_SYSTEM_CERTS=]
          --offline
              Disable network access [env: UV_OFFLINE=]
          --allow-insecure-host <ALLOW_INSECURE_HOST>
              Allow insecure connections to a host [env: UV_INSECURE_HOST=]
          --no-progress
              Hide all progress outputs [env: UV_NO_PROGRESS=]
          --directory <DIRECTORY>
              Change to the given directory prior to running the command [env: UV_WORKING_DIR=]
          --project <PROJECT>
              Discover a project in the given directory [env: UV_PROJECT=]
          --config-file <CONFIG_FILE>
              The path to a `uv.toml` file to use for configuration [env: UV_CONFIG_FILE=]
          --no-config
              Avoid discovering configuration files (`pyproject.toml`, `uv.toml`) [env: UV_NO_CONFIG=]
      -h, --help
              Display the concise help for this command
      -V, --version
              Display the uv version

    Use `uv help <command>` for more information on a specific command.
    ");
}
