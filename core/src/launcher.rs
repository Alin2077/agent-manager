use crate::config;
use crate::models::AgentFormat;
use std::process::{Command, Stdio};

/// 启动 Agent，返回 Result
pub fn launch_agent(agent: &str, profile_name: Option<&str>) -> Result<i32, String> {
    let global = config::load_global_config();
    let default_name = global
        .default_profile
        .clone()
        .unwrap_or_else(|| "default".to_string());
    let profile_name = profile_name.unwrap_or(&default_name);

    // 检查 agent 是否存在
    which::which(agent).map_err(|_| {
        format!("Agent '{}' 未找到，请确认已安装并在 PATH 中", agent)
    })?;

    // 加载 profile
    let profile = config::load_profile(profile_name).ok_or_else(|| {
        format!("Profile '{}' 不存在", profile_name)
    })?;

    let api_key = config::resolve_api_key(&profile.api_key);

    let (env_key_name, effective_url) = match profile.format {
        AgentFormat::OpenAI => ("OPENAI_API_KEY", profile.base_url.clone()),
        AgentFormat::ClaudeCode => ("ANTHROPIC_API_KEY", profile.base_url.clone()),
        AgentFormat::Custom => ("API_KEY", profile.base_url.clone()),
    };

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

    let mut child = cmd.spawn().map_err(|e| format!("启动失败: {}", e))?;
    let status = child.wait().map_err(|e| format!("等待进程出错: {}", e))?;

    Ok(status.code().unwrap_or(0))
}
