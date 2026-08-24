use anyhow::Result;
use assert_fs::fixture::PathChild;
use insta::assert_snapshot;

use uv_test::uv_snapshot;

#[test]
fn update_shell_tool_and_python() -> Result<()> {
    let context = uv_test::test_context_with_versions!(&[]);

    // The tool updater creates both Bash startup files when neither already exists.
    // [第3次修正] 工具与托管 Python 可执行目录共用 `UV_HOME/bin`。
    uv_snapshot!(
        context.filters(),
        context.command().arg("tool").arg("update-shell"),
        @"
        exit_code: 0 (success)
        ----- stderr -----
        Created configuration file: [HOME]/.bash_profile
        Created configuration file: [HOME]/.bashrc
        Restart your shell to apply changes
        "
    );

    // The Python updater targets the same `UV_HOME/bin` directory, which is already present in
    // the startup files (but not in the current PATH), so it reports up-to-date files.
    uv_snapshot!(
        context.filters(),
        context.command().arg("python").arg("update-shell"),
        @"
        exit_code: 2 (failure)
        ----- stderr -----
        error: The executable directory [TEMP_DIR]/bin is not in PATH, but the Bash configuration files are already up-to-date
        "
    );

    let bash_profile = fs_err::read_to_string(context.home_dir.child(".bash_profile"))?;
    let bashrc = fs_err::read_to_string(context.home_dir.child(".bashrc"))?;
    assert_eq!(bash_profile, bashrc);

    // Both updaters share the single `UV_HOME/bin` executable directory.
    insta::with_settings!({ filters => context.filters() }, {
        assert_snapshot!(bash_profile, @r#"
        # uv
        export PATH="[TEMP_DIR]/bin:$PATH"
        "#);
    });

    Ok(())
}
