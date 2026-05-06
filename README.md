# ccs - Claude Code Switcher

从 [cc-switch](https://github.com/farion1231/cc-switch) 的数据库中读取 Claude 供应商配置，终端 TUI 快速选择并启动 `claude`。

## 用法

```bash
ccs              # 选择供应商并启动 claude
ccs -dsp         # 选择后以 --dangerously-skip-permissions 模式启动
ccs -v           # 显示版本
ccs -h           # 显示帮助
```

TUI 操作：`1-9` 选择，`u/i` 翻页，`q` 退出。

## 前置条件

- 已安装 [cc-switch](https://github.com/farion1231/cc-switch) 并配置了供应商
- 已安装 [Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code)
- 已安装 [Rust](https://rustup.rs/)（编译用）

## macOS / Linux

```bash
# 编译安装
cargo install --path /Users/lily/job/software/ccs

# 确保 ~/.cargo/bin 在 PATH 中
export PATH="$HOME/.cargo/bin:$PATH"

# 使用
ccs
```

如需永久生效，将 `export PATH="$HOME/.cargo/bin:$PATH"` 添加到 `~/.zshrc` 或 `~/.bashrc`。

## Windows

```powershell
# 编译安装
cargo install --path C:\path\to\ccs

# 使用
ccs
```

Windows 上临时配置文件写入 `%TEMP%\ccs-{id}.json`，退出 claude 后自动回到终端。

## 原理

1. 读取 `~/.cc-switch/cc-switch.db`（SQLite）中的 Claude 供应商
2. 用户在 TUI 中选择供应商
3. 将该供应商的 `settings_config` 写入临时文件
4. 用 `claude --settings <临时文件>` 启动 Claude Code

macOS/Linux 上使用 `exec` 替换当前进程（无残留），Windows 上使用 `spawn` 等待子进程退出。
