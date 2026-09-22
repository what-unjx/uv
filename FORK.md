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

## 构建（Windows）

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

推荐顺序：先推 tag（自动触发 Linux CI 并创建 Release），再本地打包 Windows 产物并上传。

1. 提交全部变更，确保工作区干净；
2. 打标签并推送：

   ```powershell
   git tag -a v<上游版本>-unjx.<序号> -m "<说明>"
   git push origin v<上游版本>-unjx.<序号>
   ```

   ⚠️ 推送 `v*-unjx.*` tag 会**自动触发** `.github/workflows/build-linux.yml`，
   构建 Linux musl 静态产物并在该 tag 上创建 Release（详见下节）。

3. Linux 侧无需手工操作，等待 Actions 完成即可（约 11 分钟）。
4. 本地打包 Windows 产物（Bandizip CLI，输出 7z 到 `dist\`，按体积自动选择压缩级别）：

   ```powershell
   .\build\pack-release.ps1 -Tag v0.12.5-unjx.3
   ```

   分档规则：内容 <16 MiB 仅存储；16~32 MiB 正常压缩；>32 MiB 极限压缩。

5. 把 7z 挂到同一个 Release：

   ```powershell
   # Release 通常已由 Linux CI 创建，直接覆盖上传
   gh release upload v0.12.5-unjx.3 --repo what-unjx/uv `
       dist\uv-v0.12.5-unjx.3-x86_64-pc-windows-msvc.7z --clobber

   # 若 Release 尚不存在（例如 Linux CI 失败），再自行创建
   gh release create v0.12.5-unjx.3 --repo what-unjx/uv `
       --title "uv v0.12.5-unjx.3" --latest `
       --notes-file <notes.md> `
       dist\uv-v0.12.5-unjx.3-x86_64-pc-windows-msvc.7z
   ```

   注意：本仓库 `gh` 默认解析到上游 astral-sh/uv，命令必须带
   `--repo what-unjx/uv`。

## 发布流程（Linux / Ubuntu 24.04，GitHub CI）

由 `.github/workflows/build-linux.yml` 全自动完成，**只需推送 tag**：

- **触发**：推送形如 `v<上游版本>-unjx.<序号>` 的 tag（glob `v*-unjx.*`）。
  该 tag 所指提交必须已包含该 workflow 文件。
- **构建**：在 `ubuntu-24.04` 上原生构建 `x86_64-unknown-linux-musl`，
  产物为 `static-pie` 全静态二进制，不依赖目标机 libc，可在 Ubuntu 24.04
  及更新的发行版上直接运行（无需 `musl` 或交叉工具链）。
- **产物**：`uv-<tag>-x86_64-unknown-linux-musl.zip`，平铺内含 `uv` 与 `uvx`
  两个文件（Linux 侧打包，自带可执行权限位，解压后无需 `chmod +x`），
  另附同名 `.sha256`。校验方式：

  ```shell
  sha256sum -c uv-<tag>-x86_64-unknown-linux-musl.zip.sha256
  unzip uv-<tag>-x86_64-unknown-linux-musl.zip
  ./uv --version
  ```

  `uvx` 需要与 `uv` 位于同一目录（`uvx` 会查找同目录的 `uv`，见
  `crates/uv/src/bin/uvx.rs`）。

- **发布**：自动创建 Release（标题 `uv <tag>`，`--latest`）；若该 tag 的 Release
  已存在，则改为覆盖上传资产，因此重跑同一个 run 是幂等的。
- **手动触发不可用**：workflow 只监听 tag（`workflow_dispatch` 要求 workflow 文件
  位于仓库默认分支，而本仓库默认分支为 `main`）。需要重新构建时，从 Actions
  页面 Re-run 同一个 run。
