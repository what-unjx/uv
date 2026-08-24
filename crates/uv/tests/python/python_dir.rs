use uv_test::uv_snapshot;

#[test]
fn python_dir() {
    let context = uv_test::test_context!("3.12");

    // [第3次修正] 托管 Python 目录固定为 UV_HOME/data/python
    uv_snapshot!(context.filters(), context.python_dir(), @"
    exit_code: 0 (success)
    ----- stdout -----
    [TEMP_DIR]/data/python
    ");
}
