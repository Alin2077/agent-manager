use crate::config;
use crate::models::AgentFormat;
use crate::scanner;
use std::process::{Command, Stdio};

/// 启动 Agent，返回退出码
pub fn launch_agent(agent: &str, profile_name: Option<&str>) -> Result<i32, String> {
    let global = config::load_global_config();
    let default_name = global
        .default_profile
        .clone()
        .unwrap_or_else(|| "default".to_string());
    let profile_name = profile_name.unwrap_or(&default_name);

    let agent_path = find_agent_path(agent)?;

    let profile = config::load_profile(profile_name).ok_or_else(|| {
        format!("Profile '{}' 不存在", profile_name)
    })?;

    let api_key = config::resolve_api_key(&profile.api_key);

    let mut cmd = Command::new(&agent_path);

    // ── 按 Agent 格式设置标准环境变量 ──
    match profile.format {
        AgentFormat::OpenAI => {
            cmd.env("OPENAI_API_KEY", &api_key);
            if !profile.base_url.is_empty() {
                cmd.env("OPENAI_BASE_URL", &profile.base_url);
            }
            if !profile.model.is_empty() {
                cmd.env("OPENAI_MODEL", &profile.model);
            }
        }
        AgentFormat::ClaudeCode => {
            cmd.env("ANTHROPIC_API_KEY", &api_key);
            if !profile.base_url.is_empty() {
                cmd.env("ANTHROPIC_BASE_URL", &profile.base_url);
            }
            if !profile.model.is_empty() {
                cmd.env("ANTHROPIC_MODEL", &profile.model);
            }
        }
        AgentFormat::Custom => {
            cmd.env("API_KEY", &api_key);
            if !profile.base_url.is_empty() {
                cmd.env("API_BASE_URL", &profile.base_url);
            }
            if !profile.model.is_empty() {
                cmd.env("API_MODEL", &profile.model);
            }
        }
    }

    // ── extra 字段：所有 key=value 直接作为环境变量注入 ──
    for (k, v) in &profile.extra {
        cmd.env(k, v);
    }

    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    let mut child = cmd.spawn().map_err(|e| format!("启动失败: {}", e))?;
    let status = child.wait().map_err(|e| format!("等待进程出错: {}", e))?;

    Ok(status.code().unwrap_or(0))
}

/// 查找 agent 的可执行文件路径
fn find_agent_path(agent: &str) -> Result<String, String> {
    let cached = scanner::load_cached_agents();
    if let Some(a) = cached.iter().find(|a| a.binary == agent) {
        if std::path::Path::new(&a.path).exists() {
            return Ok(a.path.clone());
        }
    }
    which::which(agent)
        .map(|p| p.display().to_string())
        .map_err(|_| {
            format!("Agent '{}' 未找到。请先扫描 Agent 或确认已安装", agent)
        })
}
