use crate::models::{AgentFormat, AgentInfo};
use colored::Colorize;
use std::process::Command;

/// 已知的 Agent 产品定义：名称、可能的二进制名、支持的格式
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

/// 扫描已安装的 Agent
pub fn scan_agents(verbose: bool) {
    println!("{}", "🔍 正在扫描已安装的 Agent...".bold());
    println!();

    let mut found: Vec<AgentInfo> = Vec::new();

    for (name, binaries, formats) in KNOWN_AGENTS {
        if verbose {
            println!("  检查 {}...", name);
        }

        for binary in *binaries {
            if let Some(agent) = try_find_agent(name, binary, formats.to_vec(), verbose) {
                found.push(agent);
                break; // 找到第一个就停
            }
        }
    }

    if found.is_empty() {
        println!("  ❌ 未检测到任何已安装的 Agent 产品");
        println!();
        println!("  💡 提示：确保 Agent 的二进制文件在 PATH 环境变量中");
        return;
    }

    println!("{}", "✅ 检测到以下 Agent 产品：".green().bold());
    println!();
    println!(
        "  {:<20} {:<16} {:<15} {}",
        "名称", "命令", "格式", "路径"
    );
    println!(
        "  {:-<20} {:-<16} {:-<15} {:-<30}",
        "", "", "", ""
    );

    for agent in &found {
        let formats_str = agent
            .formats
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        println!(
            "  {:<20} {:<16} {:<15} {}",
            agent.name.cyan(),
            agent.binary.yellow(),
            formats_str,
            agent.path.dimmed()
        );
    }

    println!();
}

fn try_find_agent(
    name: &str,
    binary: &str,
    formats: Vec<AgentFormat>,
    verbose: bool,
) -> Option<AgentInfo> {
    // 首先用 which 在 PATH 中查找
    match which::which(binary) {
        Ok(path) => {
            let version = try_get_version(binary);
            if verbose {
                println!("    ✅ 在 PATH 中找到: {}", path.display());
            }
            Some(AgentInfo {
                name: name.to_string(),
                binary: binary.to_string(),
                path: path.display().to_string(),
                version,
                formats,
            })
        }
        Err(_) => {
            if verbose {
                println!("    ⏭️  未在 PATH 中找到: {}", binary);
            }
            None
        }
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
