# ccs-cli — Claude Code Switcher

[cc-switch](https://github.com/farion1231/cc-switch) 的终端伴侣工具。

[![LDO](https://ldo.betax.dev/badge/community)](https://linux.do/)

cc-switch（60k+ stars）提供了强大的 GUI 来管理 Claude Code 的 API 供应商，但每次切换供应商都要打开桌面应用。**ccs 把这个选择搬进了终端**——读取 cc-switch 的数据库，TUI 快速选供应商，一键启动 `claude`。不用离开键盘。

## 与 cc-switch 的关系

```
cc-switch（GUI）        →  管理供应商、MCP、会话、用量统计
ccs（终端 TUI）         →  读取 cc-switch 的数据库，选供应商启动 claude
```

- ccs **不管理**供应商，供应商在 cc-switch 里配置
- ccs **只做一件事**：选供应商 → 启动 claude
- ccs **依赖** cc-switch 的 `~/.cc-switch/cc-switch.db`

## 为什么用 ccs

cc-switch 本身可以从 GUI 启动 claude，但每次都要：

1. 打开 cc-switch 桌面应用
2. 找到供应商
3. 点启动

ccs 只需要：终端里敲 `ccs`，按数字键选择，回车即启动。**从输入 `ccs` 到进入 claude 不超过 2 秒。**

更重要的是，**每个终端窗口可以独立选择不同的供应商**——窗口 A 用 Claude Opus 处理架构设计，窗口 B 用 GLM 写前端，窗口 C 用 DeepSeek 做代码审查。每个窗口启动时各自 `ccs` 选择对应的供应商即可，互不干扰。

## 安装

### 前置条件

- 安装 [Rust](https://rustup.rs/)（含 `cargo`）
- 安装 [cc-switch](https://github.com/farion1231/cc-switch) 并配置好供应商
- 安装 [Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code)

### crates.io（推荐）

```bash
cargo install ccs-cli
```

### macOS / Linux

```bash
cargo install --git https://github.com/Lily369/ccs-cli.git
```

或本地编译：

```bash
git clone https://github.com/Lily369/ccs-cli.git
cd ccs-cli
cargo install --path .
```

确保 `~/.cargo/bin` 在 `PATH` 中：

```bash
export PATH="$HOME/.cargo/bin:$PATH"
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc   # 永久生效
```

### Windows

```powershell
cargo install --git https://github.com/Lily369/ccs-cli.git
```

无需额外配置。`rustup` 安装时会自动将 `%USERPROFILE%\.cargo\bin` 加入 PATH，装完直接就能在终端用 `ccs`。

唯一需要注意的是 Python、Node 等编译依赖——如果 `cargo install` 报错提示缺少 linker，安装 [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)（勾选 "Desktop development with C++"）即可。纯 Rust 依赖不涉及这些，主要是 `rusqlite` 的 bundled 模式需要 C 编译器。

## 用法

```bash
ccs                # TUI 选择供应商 → 启动 claude（默认跳过权限确认）
ccs -r             # 恢复上次会话
ccs -r <id>        # 恢复指定 session ID
ccs --safe         # 普通模式，不跳过权限确认弹窗
ccs --print        # 只打印将注入的 env / argv，不启动 claude（调试用）
ccs -v             # 显示版本
ccs -h             # 显示帮助
```

TUI 内操作：`1-9` 选择供应商，`u/i` 翻页，`q` 退出。

## 原理

1. 读取 `~/.cc-switch/cc-switch.db`（SQLite）中 `app_type = 'claude'` 的供应商
2. 终端 TUI 列出供应商，用户数字键选择
3. 将该供应商的 `settings_config`（剔除 cc-switch 内部字段）写入 `/tmp/ccs-{id}.json`
4. 执行 `claude --settings /tmp/ccs-{id}.json`，用 `exec()` 替换当前进程

macOS/Linux 上 `exec()` 让 claude 直接接管终端，ccs 进程不残留。
