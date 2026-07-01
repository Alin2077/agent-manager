use crate::models::{AgentFormat, AgentInfo};
use std::process::Command;

/// 已知的 Agent 产品定义
const KNOWN_AGENTS: &[(&str, &[&str], &[AgentFormat])] = &[
    ("Claude Code", &["claude"], &[AgentFormat::ClaudeCode]),
    ("Codex CLI", &["codex", "openai-codex"], &[AgentFormat::OpenAI]),
    ("Cursor CLI", &["cursor"], &[AgentFormat::OpenAI]),
    ("GitHub Copilot CLI", &["copilot", "github-copilot"], &[AgentFormat::OpenAI]),
    ("Gemini CLI", &["gemini", "google-gemini"], &[AgentFormat::Custom]),
    ("Windsurf", &["windsurf"], &[AgentFormat::OpenAI]),
    ("aider", &["aider"], &[AgentFormat::OpenAI]),
    ("Open Interpreter", &["interpreter", "oi"], &[AgentFormat::OpenAI]),
    ("Continue", &["continue"], &[AgentFormat::OpenAI]),
    ("Qwen Coder", &["qwen", "tongyi-coder"], &[AgentFormat::OpenAI]),
    ("DeepSeek Coder", &["deepseek-coder"], &[AgentFormat::OpenAI]),
    ("Amazon Q Developer", &["q", "amazon-q"], &[AgentFormat::Custom]),
    ("Reasonix Code", &["reasonix"], &[AgentFormat::ClaudeCode, AgentFormat::OpenAI]),
];

/// 扫描已安装的 Agent，返回结果列表
pub fn get_agents() -> Vec<AgentInfo> {
    let mut found: Vec<AgentInfo> = Vec::new();

    for (name, binaries, formats) in KNOWN_AGENTS {
        for binary in *binaries {
            if let Some(agent) = try_find_agent(name, binary, formats.to_vec()) {
                found.push(agent);
                break;
            }
        }
    }

    found
}

fn try_find_agent(
    name: &str,
    binary: &str,
    formats: Vec<AgentFormat>,
) -> Option<AgentInfo> {
    match which::which(binary) {
        Ok(path) => {
            let version = try_get_version(binary);
            Some(AgentInfo {
                name: name.to_string(),
                binary: binary.to_string(),
                path: path.display().to_string(),
                version,
                formats,
            })
        }
        Err(_) => None,
    }
}

fn try_get_version(binary: &str) -> Option<String> {
    let output = Command::new(binary)
        .arg("--version")
        .output()
        .ok()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = format!("{}{}", stdout.trim(), stderr.trim());
        if !combined.is_empty() {
            return Some(combined.lines().next()?.to_string());
        }
    }
    None
}
