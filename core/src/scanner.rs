use crate::models::{AgentFormat, AgentInfo, Skill};
use std::fs;
use std::path::PathBuf;
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

// ── Agent 缓存 ──

fn agents_cache_path() -> PathBuf {
    let dir = dirs::home_dir()
        .expect("无法获取用户主目录")
        .join(".agent-manager");
    fs::create_dir_all(&dir).ok();
    dir.join("agents.json")
}

/// 读取缓存的 Agent 列表
pub fn load_cached_agents() -> Vec<AgentInfo> {
    let path = agents_cache_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(agents) = serde_json::from_str::<Vec<AgentInfo>>(&content) {
                return agents;
            }
        }
    }
    vec![]
}

/// 保存 Agent 列表到缓存
pub fn save_agents_cache(agents: &[AgentInfo]) {
    let path = agents_cache_path();
    if let Ok(json) = serde_json::to_string_pretty(agents) {
        fs::write(&path, json).ok();
    }
}

// ── Agent 扫描 ──

/// 扫描已安装的 Agent
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

// ── Skill 扫描 ──

/// 已知 Agent 的 Skills 目录映射
fn agent_skills_dir(binary: &str) -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    match binary {
        "claude" => Some(home.join(".claude").join("skills")),
        "codex" => Some(home.join(".codex").join("skills")),
        "reasonix" => Some(home.join(".reasonix").join("skills")),
        "aider" => Some(home.join(".aider").join("skills")),
        _ => None,
    }
}

/// 扫描指定 Agent 已安装的 Skills
pub fn scan_agent_skills(agent_binary: &str) -> Vec<Skill> {
    let dir = match agent_skills_dir(agent_binary) {
        Some(d) => d,
        None => return vec![],
    };

    if !dir.exists() {
        return vec![];
    }

    let mut skills = Vec::new();
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" || ext == "toml" || ext == "txt" {
                        if let Ok(content) = fs::read_to_string(&path) {
                            let name = path.file_stem()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                            let preview = if content.len() > 200 {
                                format!("{}...", &content[..200])
                            } else {
                                content.clone()
                            };
                            skills.push(Skill {
                                name,
                                agent: agent_binary.to_string(),
                                description: format!("来自 {}", agent_binary),
                                version: "1.0".into(),
                                prompt: preview,
                            });
                        }
                    }
                }
            }
        }
    }
    skills
}
