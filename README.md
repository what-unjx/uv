<div align="center">

# uv · unjx fork

[![Release](https://img.shields.io/github/v/release/what-unjx/uv?sort=semver)](https://github.com/what-unjx/uv/releases)
[![Based on](https://img.shields.io/badge/based%20on%20astral%2Fuv-0.12.5-blue)](https://github.com/astral-sh/uv)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](./LICENSE)

**一个 `UV_HOME` 目录管理 uv 的一切 —— cargo 风格的统一存储根目录**

*English: A personal fork of [astral-sh/uv](https://github.com/astral-sh/uv) that consolidates all of
uv's scattered storage directories into a single, cargo-style `UV_HOME` root.*

</div>

---

## 为什么 fork

官方 uv 极快、极好用,但它的数据存放相当分散:缓存、工具、托管 Python、凭据、可执行文件
分散在操作系统各个角落(Windows 下分属 `%LOCALAPPDATA%`、`%APPDATA%`、`%USERPROFILE%` 等多个不同目录),
备份、迁移、清理都要翻好几个地方。

本 fork 借鉴 Rust 工具链 `~/.cargo` 的设计,引入 **`UV_HOME` 统一存储根目录**:
所有 uv 数据收敛到一个自包含的目录树下,配置、备份、迁移、PATH 设置全部化繁为简。

> 本 fork 长期通过 rebase 跟随官方 main,持续吸收上游全部更新与修复。

## 特色功能

### 1️⃣ `UV_HOME` 统一存储根目录(核心)

**必选配置**:未设置 `UV_HOME` 时 uv 会拒绝运行,并给出明确的设置指引。

**配置方式**(二选一,环境变量优先):

```bash
# 方式一:环境变量
export UV_HOME=~/.uv          # macOS / Linux
setx UV_HOME D:\uv            # Windows(永久)
```

```toml
# 方式二:用户级 uv.toml
# Windows: %APPDATA%\uv\uv.toml   Unix: ~/.config/uv/uv.toml
home = "D:/uv"
```

**目录布局** —— 一个根目录,接管一切:

```
UV_HOME/
├── cache/                      # 包缓存                        (≈ UV_CACHE_DIR)
│   └── python/                 # Python 下载缓存               (≈ UV_PYTHON_CACHE_DIR)
├── data/                       # 持久状态
│   ├── tools/                  # uv tool 安装的工具            (≈ UV_TOOL_DIR)
│   ├── python/                 # uv python 托管安装            (≈ UV_PYTHON_INSTALL_DIR)
│   └── credentials/            # 凭据存储                      (≈ UV_CREDENTIALS_DIR)
└── bin/                        # 可执行文件(tools + python)   (≈ UV_TOOL_BIN_DIR / UV_PYTHON_BIN_DIR)
```

**行为规则**:

- 设置 `UV_HOME` 后,上表中所有独立目录环境变量(`UV_CACHE_DIR`、`UV_TOOL_DIR` 等)即被忽略,
  统一使用 `UV_HOME` 下的对应子目录;命令行参数则优先于 `UV_HOME`——缓存目录的解析顺序为
  `--no-cache` > `--cache-dir` > `UV_HOME/cache` > `UV_CACHE_DIR` > uv.toml 的 `cache-dir` > 系统默认;
- `uv python update-shell` / `uv tool update-shell` 会自动把 `UV_HOME/bin` 写入 shell 配置。

**收益**:

| 场景 | 官方 uv | 本 fork |
| --- | --- | --- |
| 备份 / 迁移 | 需逐个找出散落各处的目录 | 复制 `UV_HOME` 一个目录即可 |
| 彻底卸载残留 | 手动清理多个系统目录 | 删除 `UV_HOME` 即可 |
| PATH 配置 | 工具、Python 各一条路径 | 只需 `UV_HOME/bin` 一条 |
| 存储位置配置 | 最多 7 个环境变量 | 1 个 `UV_HOME` |

### 2️⃣ 精简虚拟环境(破坏性变更)

- 移除 `uv venv --prompt` 与整个 venv prompt 机制,激活脚本更干净,
  `pyvenv.cfg` 不再写入 `prompt` 字段;
- `uv venv` 的目标路径从位置参数改为显式参数:`uv venv project` → `uv venv --path project`。

## 安装(Windows)

从 [Releases](https://github.com/what-unjx/uv/releases) 下载 `uv.exe`,然后:

```powershell
# 1. 放到 UV_HOME\bin 下(推荐,或任意 PATH 目录)
mkdir D:\uv\bin
move .\uv.exe D:\uv\bin\

# 2. 设置统一存储根目录并写入 PATH
setx UV_HOME D:\uv
setx PATH "$env:PATH;D:\uv\bin"
```

重开终端后即可使用,所有数据都会收敛在 `D:\uv` 下。

### 快速上手

```bash
$ uv venv                        # 未设置 UV_HOME 时会得到清晰的报错指引
$ export UV_HOME=~/.uv
$ uv venv                        # 创建虚拟环境(项目本地,不受 UV_HOME 影响)
$ uv pip install requests        # 缓存写入 ~/.uv/cache/
$ uv python install 3.13         # 安装到 ~/.uv/data/python/,解释器链接进 ~/.uv/bin/
$ uv tool install ruff           # 安装到 ~/.uv/data/tools/,可执行文件进 ~/.uv/bin/
$ uv python dir                  # 显示 ~/.uv/data/python
$ uv tool dir                    # 显示 ~/.uv/data/tools
$ uv cache dir                   # 显示 ~/.uv/cache
```

## 与官方 uv 的差异一览

| 方面 | 官方 uv | 本 fork |
| --- | --- | --- |
| 存储布局 | 分散在系统各标准目录 | 统一收敛于 `UV_HOME` |
| 未配置时 | 使用各平台默认目录 | **报错退出**,要求显式配置 |
| 目录类环境变量 | 各自生效 | 设置 `UV_HOME` 后被忽略 |
| `uv venv --prompt` | 支持 | 已移除 |
| `uv venv <path>`(位置参数) | 支持 | 改为 `uv venv --path <path>` |
| 其余全部功能 | — | 与官方一致(rebase 跟随 upstream) |

详细语义见仓库内文档:[`docs/reference/storage.md`](./docs/reference/storage.md)
与 [`docs/concepts/configuration-files.md`](./docs/concepts/configuration-files.md)。

## 与上游同步

本 fork 以单提交形式维护全部定制(见 `local-mods` 分支最新提交),通过
`git fetch upstream && git rebase upstream/main` 持续跟随官方 main,冲突极少且可复现解决。

## 致谢

本 fork 基于 [astral-sh/uv](https://github.com/astral-sh/uv) —— 由 [Astral](https://astral.sh)
打造的极快 Python 包与项目管理器。全部上游功劳归 Astral 与 uv 社区;本仓库仅保留个人定制,
许可证(MIT OR Apache-2.0)与上游一致。
