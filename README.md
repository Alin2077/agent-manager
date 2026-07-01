# Agent Manager

管理多个 AI Agent 产品的工具 —— 提供 **命令行** 和 **桌面 GUI** 两种使用方式。

## 功能

- 🔍 **扫描 Agent** — 自动检测电脑上已安装的 AI Agent（Claude Code、Codex CLI、Cursor、Copilot…）
- ⚙️ **多配置管理** — 为每个 Agent 保存多套 Profile（Base URL / 模型 / API Key），一键切换
- 📚 **Skills 管理** — 创建和管理可复用的 Skill prompt 模板
- 🚀 **一键启动** — 选择 Profile 直接启动 Agent，自动注入环境变量
- 🖥️ **桌面 GUI** — Tauri 原生图形界面（[下载](#安装)）

## 安装

### 方式一：下载预编译二进制（推荐）

从 [Releases](https://github.com/Alin2077/agent-manager/releases) 页面下载最新版本：

| 平台 | 文件 |
|------|------|
| Windows | `agent-manager-windows-amd64.exe` |
| Linux | `agent-manager-linux-amd64` |
| macOS Intel | `agent-manager-macos-amd64` |
| macOS Apple Silicon | `agent-manager-macos-arm64` |

#### Windows

1. 下载 `agent-manager-windows-amd64.exe`
2. 在 `C:\Program Files\` 下创建 `agent-manager` 文件夹，把 exe 放进去
3. 打开 **系统属性 → 环境变量**，在 `Path` 中新增 `C:\Program Files\agent-manager`
4. 重新打开终端，输入 `agent-manager --help` 验证

#### Linux

```bash
chmod +x agent-manager-linux-amd64
sudo mv agent-manager-linux-amd64 /usr/local/bin/agent-manager
agent-manager --help
```

#### macOS

```bash
chmod +x agent-manager-macos-arm64      # Apple Silicon 用这个
# 或 chmod +x agent-manager-macos-amd64  # Intel Mac 用这个
sudo mv agent-manager-macos-arm64 /usr/local/bin/agent-manager
xattr -d com.apple.quarantine /usr/local/bin/agent-manager 2>/dev/null  # 跳过 Gatekeeper
agent-manager --help
```

### 方式二：Cargo 安装

```bash
cargo install --git https://github.com/Alin2077/agent-manager.git
```

## 使用

```bash
# 扫描已安装的 Agent
agent-manager scan
agent-manager scan --verbose

# 管理 Profile
agent-manager profile list
agent-manager profile add my-openai
agent-manager profile show my-openai
agent-manager profile set-default my-openai

# 管理 Skills
agent-manager skill list
agent-manager skill add code-review

# 启动 Agent
agent-manager launch claude --profile my-openai
```

## Profile 配置示例

```toml
# ~/.agent-manager/profiles/my-openai.toml
name = "my-openai"
format = "openai"
base_url = "https://api.openai.com/v1"
model = "gpt-4o"
api_key = "$OPENAI_API_KEY"
max_tokens = 4096
```

API Key 支持直接填写或 `$ENV_VAR` 引用环境变量。

## 项目结构

```
agent-manager/
├── core/          # 核心库（CLI 和 GUI 共用）
├── cli/           # 命令行工具
├── gui/           # Tauri 桌面应用
└── .github/       # CI/CD
```

## 开发

```bash
# 编译 CLI
cargo build -p agent-manager --release

# 编译 GUI（需要系统安装 WebView2 / WebKitGTK）
cargo build -p agent-manager-gui --release

# 运行 CLI
cargo run -p agent-manager -- scan

# 运行 GUI
cargo run -p agent-manager-gui

# 打包 GUI 安装程序
cargo install tauri-cli
cargo tauri build
```

### 发布

```bash
.\release.ps1
```

## 许可

MIT
