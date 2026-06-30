use crate::config;
use crate::models::AgentFormat;
use colored::Colorize;
use std::process::{Command, Stdio};

/// 启动 Agent
pub fn launch_agent(agent: &str, profile_name: Option<&str>) {
    let global = config::load_global_config();
    let default_name = global
        .default_profile
        .clone()
        .unwrap_or_else(|| "default".to_string());
    let profile_name = profile_name.unwrap_or(&default_name);

    match which::which(agent) {
        Ok(path) => {
            println!(
                "{} 找到 Agent: {} ({})",
                "✅".green(),
                agent.cyan().bold(),
                path.display().to_string().dimmed()
            );
        }
        Err(_) => {
            println!(
                "{} Agent '{}' 未找到，请确认已安装并在 PATH 中",
                "❌".red(),
                agent
            );
            return;
        }
    }

    let profile = match config::load_profile(profile_name) {
        Some(p) => p,
        None => {
            println!(
                "{} Profile '{}' 不存在，请先创建",
                "❌".red(),
                profile_name
            );
            println!("  💡 使用 agent-manager profile add {}", profile_name);
            return;
        }
    };

    let api_key = config::resolve_api_key(&profile.api_key);

    println!();
    println!("{} 启动参数:", "🚀".bold());
    println!("  Agent:     {}", agent.cyan());
    println!("  Profile:   {}", profile_name.cyan());
    println!("  格式:      {}", profile.format.to_string().yellow());
    println!("  Base URL:  {}", profile.base_url);
    println!("  模型:      {}", profile.model);
    println!();

    let (env_key_name, effective_url) = match profile.format {
        AgentFormat::OpenAI => ("OPENAI_API_KEY", profile.base_url.clone()),
        AgentFormat::ClaudeCode => ("ANTHROPIC_API_KEY", profile.base_url.clone()),
        AgentFormat::Custom => ("API_KEY", profile.base_url.clone()),
    };

    println!("{} 正在启动 {}...", "⏳".bold(), agent.cyan());
    println!(
        "  环境变量: {}={}",
        env_key_name,
        "*".repeat(api_key.len().min(8))
    );
    println!();

    let mut cmd = Command::new(agent);
    cmd.env(env_key_name, &api_key);

    match profile.format {
        AgentFormat::OpenAI => {
            if !effective_url.is_empty() {
                cmd.env("OPENAI_BASE_URL", &effective_url);
            }
            cmd.env("OPENAI_MODEL", &profile.model);
        }
        AgentFormat::ClaudeCode => {
            if !effective_url.is_empty() {
                cmd.env("ANTHROPIC_BASE_URL", &effective_url);
            }
            cmd.env("ANTHROPIC_MODEL", &profile.model);
        }
        AgentFormat::Custom => {
            cmd.env("API_BASE_URL", &effective_url);
            cmd.env("API_MODEL", &profile.model);
        }
    }

    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    match cmd.spawn() {
        Ok(mut child) => {
            match child.wait() {
                Ok(status) => {
                    if !status.success() {
                        eprintln!();
                        eprintln!(
                            "{} Agent 退出码: {}",
                            "⚠️".yellow(),
                            status.code().unwrap_or(-1)
                        );
                    }
                }
                Err(e) => {
                    eprintln!("{} 等待进程时出错: {}", "❌".red(), e);
                }
            }
        }
        Err(e) => {
            eprintln!("{} 启动 Agent 失败: {}", "❌".red(), e);
        }
    }
}
