# Fork 说明（what-unjx/uv）

本仓库是 [astral-sh/uv](https://github.com/astral-sh/uv) 的定制分叉，维护本地增强功能。
上游版本基于 `upstream` 远程同步，定制发布以 `-unjx.N` 后缀的标签区分。

## 定制点

1. **UV_HOME 收敛**：将 uv 的各类数据/缓存目录收敛到统一的 `UV_HOME` 根目录管理，
   便于便携化部署（相关环境变量见 `crates/uv-static/src/env_vars.rs`）。
2. **移除托管 Python 下载功能**（进行中）：去掉内置的 Python 托管下载机制，
   Python 查找改为依赖系统/外部安装（详见 git log 中 WIP 提交）。
3. **uv-keyring 修复**：为仅被 `native-auth` 测试使用的内部条目补齐
   `#[cfg(feature = "native-auth")]` 门控，消除默认构建的 dead_code 警告。

## 构建

`build\` 目录下提供构建与打包脚本：

```powershell
.\build\build-uv.ps1                          # 默认特性: performance, self-update
.\build\build-uv.ps1 -Features performance    # 自定义特性列表
```

在标准 release 配置（`strip` + `lto = "fat"` + `panic = "abort"`，Windows 下启用
mimalloc v2 分配器）基础上，追加 `codegen-units = 1` 以获得更好的优化效果。
产物位于 `target\release\uv.exe` 与 `target\release\uvx.exe`。

注意：首次构建或与普通 `cargo build --release` 交替时会触发较长的全量重编，
属正常现象（profile 指纹不同）。

## 发布流程

1. 提交全部变更，确保工作区干净；
2. 打标签：`git tag -a v<上游版本>-unjx.<序号> -m "<说明>"` 并推送；
3. 本地打包（Bandizip CLI，输出 7z 到 `dist\`，按体积自动选择压缩级别）：

   ```powershell
   .\build\pack-release.ps1 -Tag v0.12.5-unjx.2
   ```

   分档规则：内容 <16 MiB 仅存储；16~32 MiB 正常压缩；>32 MiB 极限压缩。

4. 创建 Release：

   ```powershell
   gh release create v0.12.5-unjx.2 --repo what-unjx/uv `
       --title "uv v0.12.5-unjx.2" --latest `
       --notes-file <notes.md> dist\uv-v0.12.5-unjx.2-x86_64-pc-windows-msvc.7z
   ```

   注意：本仓库 `gh` 默认解析到上游 astral-sh/uv，命令必须带
   `--repo what-unjx/uv`。
