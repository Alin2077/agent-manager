# Agent Manager

管理多个 AI Agent 产品的命令行工具 —— 扫描、配置、启动，一站式管理。

## 功能

- 🔍 **扫描 Agent** — 自动检测电脑上已安装的 AI Agent（Claude Code、Codex CLI、Cursor、Copilot…）
- ⚙️ **多配置管理** — 为每个 Agent 保存多套 Profile（Base URL / 模型 / API Key），一键切换
- 📚 **Skills 管理** — 创建和管理可复用的 Skill prompt 模板
- 🚀 **一键启动** — 选择 Profile 直接启动 Agent，自动注入环境变量

## 安装

从 [Releases](https://github.com/Alin2077/agent-manager/releases) 下载对应平台的二进制文件：

| 平台 | 文件 |
|------|------|
| Windows | `agent-manager-windows-amd64.exe` |
| Linux | `agent-manager-linux-amd64` |
| macOS Intel | `agent-manager-macos-amd64` |
| macOS Apple Silicon | `agent-manager-macos-arm64` |

下载后放到 PATH 目录即可使用。

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

## 开发

```bash
# 编译
cargo build --release

# 运行
cargo run -- scan

# 发布
.\release.ps1
```

## 许可

MIT
