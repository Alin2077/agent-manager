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

    // 1. 查找 agent 路径：优先用缓存路径
    let agent_path = find_agent_path(agent)?;

    // 2. 加载 profile
    let profile = config::load_profile(profile_name).ok_or_else(|| {
        format!("Profile '{}' 不存在", profile_name)
    })?;

    let api_key = config::resolve_api_key(&profile.api_key);

    let (env_key_name, effective_url) = match profile.format {
        AgentFormat::OpenAI => ("OPENAI_API_KEY", profile.base_url.clone()),
        AgentFormat::ClaudeCode => ("ANTHROPIC_API_KEY", profile.base_url.clone()),
        AgentFormat::Custom => ("API_KEY", profile.base_url.clone()),
    };

    let mut cmd = Command::new(&agent_path);
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

    // 支持 extra 字段作为额外启动参数
    for (k, v) in &profile.extra {
        cmd.arg(format!("--{}", k));
        if !v.is_empty() {
            cmd.arg(v);
        }
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
    // 1) 从缓存查找
    let cached = scanner::load_cached_agents();
    if let Some(a) = cached.iter().find(|a| a.binary == agent) {
        if std::path::Path::new(&a.path).exists() {
            return Ok(a.path.clone());
        }
    }

    // 2) 从 PATH 查找
    which::which(agent)
        .map(|p| p.display().to_string())
        .map_err(|_| {
            format!("Agent '{}' 未找到。请先扫描 Agent 或确认已安装", agent)
        })
}
