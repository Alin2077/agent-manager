use crate::models::{AgentFormat, GlobalConfig, Profile};
use colored::Colorize;
use dialoguer::{Input, Select};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// 获取 agent-manager 配置根目录：~/.agent-manager/
fn config_dir() -> PathBuf {
    let dir = dirs::home_dir()
        .expect("无法获取用户主目录")
        .join(".agent-manager");
    fs::create_dir_all(&dir).expect("无法创建配置目录");
    dir
}

fn profiles_dir() -> PathBuf {
    let dir = config_dir().join("profiles");
    fs::create_dir_all(&dir).expect("无法创建 profiles 目录");
    dir
}

fn profile_path(name: &str) -> PathBuf {
    profiles_dir().join(format!("{}.toml", name))
}

fn global_config_path() -> PathBuf {
    config_dir().join("config.toml")
}

// ---------- Profile CRUD ----------

pub fn list_profiles() {
    let dir = profiles_dir();
    let entries = match fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => {
            println!("{}", "📭 还没有任何 Profile".yellow());
            return;
        }
    };

    let mut profiles: Vec<Profile> = Vec::new();
    for entry in entries.flatten() {
        if entry.path().extension().map(|e| e == "toml").unwrap_or(false) {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                if let Ok(p) = toml::from_str::<Profile>(&content) {
                    profiles.push(p);
                }
            }
        }
    }

    if profiles.is_empty() {
        println!("{}", "📭 还没有任何 Profile".yellow());
        println!("  💡 使用 agent-manager profile add <name> 创建");
        return;
    }

    let default = load_global_config().default_profile;

    println!("{}", "📋 所有 Profile:".bold());
    println!();
    for p in &profiles {
        let marker = if Some(p.name.clone()) == default {
            " ⭐".yellow()
        } else {
            "".into()
        };
        println!(
            "  {}{}  {} | {} | {}",
            p.name.cyan().bold(),
            marker,
            format!("[{}]", p.format).dimmed(),
            p.model,
            p.base_url.dimmed()
        );
    }
    println!();
}

pub fn add_profile(name: &str) {
    let path = profile_path(name);
    if path.exists() {
        println!("{} Profile '{}' 已存在", "❌".red(), name);
        return;
    }

    println!("{} 创建 Profile: {}", "✨".bold(), name.cyan().bold());
    println!();

    // 选择格式
    let formats = vec!["openai", "claude-code", "custom"];
    let format_idx = Select::new()
        .with_prompt("选择配置格式")
        .items(&formats)
        .default(0)
        .interact()
        .unwrap();
    let format = match format_idx {
        0 => AgentFormat::OpenAI,
        1 => AgentFormat::ClaudeCode,
        _ => AgentFormat::Custom,
    };

    // 交互输入
    let base_url: String = Input::new()
        .with_prompt("Base URL")
        .default(match format {
            AgentFormat::OpenAI => "https://api.openai.com/v1".into(),
            AgentFormat::ClaudeCode => "https://api.anthropic.com/v1".into(),
            AgentFormat::Custom => "http://localhost:8080".into(),
        })
        .interact_text()
        .unwrap();

    let model: String = Input::new()
        .with_prompt("模型名称")
        .default(match format {
            AgentFormat::OpenAI => "gpt-4o".into(),
            AgentFormat::ClaudeCode => "claude-sonnet-4-20250514".into(),
            AgentFormat::Custom => "default-model".into(),
        })
        .interact_text()
        .unwrap();

    let api_key: String = Input::new()
        .with_prompt("API Key（可直接输入或使用 $ENV_VAR 引用环境变量）")
        .interact_text()
        .unwrap();

    let max_tokens: String = Input::new()
        .with_prompt("Max Tokens（留空使用默认）")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let profile = Profile {
        name: name.to_string(),
        format,
        base_url,
        model,
        api_key,
        max_tokens: max_tokens.parse().ok(),
        extra: HashMap::new(),
    };

    let content = toml::to_string_pretty(&profile).expect("序列化失败");
    fs::write(&path, content).expect("写入配置失败");

    println!();
    println!("{} Profile '{}' 已创建", "✅".green(), name);
    println!("  路径: {}", path.display().to_string().dimmed());
}

pub fn show_profile(name: &str) {
    let path = profile_path(name);
    if !path.exists() {
        println!("{} Profile '{}' 不存在", "❌".red(), name);
        return;
    }

    let content = fs::read_to_string(&path).expect("读取配置失败");
    let profile: Profile = toml::from_str(&content).expect("解析配置失败");

    println!("{} Profile: {}", "📋".bold(), profile.name.cyan().bold());
    println!();
    println!("  格式:      {}", profile.format.to_string().yellow());
    println!("  Base URL:  {}", profile.base_url);
    println!("  模型:      {}", profile.model);
    // 安全展示：隐藏 API Key 中间部分
    let mask = mask_key(&profile.api_key);
    println!("  API Key:   {}", mask);
    if let Some(mt) = profile.max_tokens {
        println!("  Max Tokens: {}", mt);
    }
    if !profile.extra.is_empty() {
        println!("  额外参数:");
        for (k, v) in &profile.extra {
            println!("    {} = {}", k, v);
        }
    }
    println!();
}

pub fn edit_profile(name: &str) {
    let path = profile_path(name);
    if !path.exists() {
        println!("{} Profile '{}' 不存在", "❌".red(), name);
        return;
    }

    let content = fs::read_to_string(&path).expect("读取配置失败");
    let mut profile: Profile = toml::from_str(&content).expect("解析配置失败");

    println!("{} 编辑 Profile: {}", "✏️".bold(), name.cyan().bold());
    println!("  （直接回车保留原值）");
    println!();

    let base_url: String = Input::new()
        .with_prompt("Base URL")
        .default(profile.base_url.clone())
        .interact_text()
        .unwrap();
    profile.base_url = base_url;

    let model: String = Input::new()
        .with_prompt("模型名称")
        .default(profile.model.clone())
        .interact_text()
        .unwrap();
    profile.model = model;

    let api_key: String = Input::new()
        .with_prompt("API Key")
        .default(profile.api_key.clone())
        .interact_text()
        .unwrap();
    profile.api_key = api_key;

    // 格式
    let formats = vec!["openai", "claude-code", "custom"];
    let default_idx = match profile.format {
        AgentFormat::OpenAI => 0,
        AgentFormat::ClaudeCode => 1,
        AgentFormat::Custom => 2,
    };
    let format_idx = Select::new()
        .with_prompt("配置格式")
        .items(&formats)
        .default(default_idx)
        .interact()
        .unwrap();
    profile.format = match format_idx {
        0 => AgentFormat::OpenAI,
        1 => AgentFormat::ClaudeCode,
        _ => AgentFormat::Custom,
    };

    let content = toml::to_string_pretty(&profile).expect("序列化失败");
    fs::write(&path, content).expect("写入配置失败");

    println!();
    println!("{} Profile '{}' 已更新", "✅".green(), name);
}

pub fn delete_profile(name: &str) {
    let path = profile_path(name);
    if !path.exists() {
        println!("{} Profile '{}' 不存在", "❌".red(), name);
        return;
    }

    fs::remove_file(&path).expect("删除配置失败");
    println!("{} Profile '{}' 已删除", "🗑️".bold(), name);

    // 如果删除的是默认 profile，清除默认
    let mut global = load_global_config();
    if global.default_profile.as_deref() == Some(name) {
        global.default_profile = None;
        save_global_config(&global);
        println!("  （已清除默认 Profile）");
    }
}

pub fn set_default_profile(name: &str) {
    let path = profile_path(name);
    if !path.exists() {
        println!("{} Profile '{}' 不存在", "❌".red(), name);
        return;
    }

    let mut global = load_global_config();
    global.default_profile = Some(name.to_string());
    save_global_config(&global);

    println!("{} 默认 Profile 已设置为 '{}'", "⭐".bold(), name.cyan());
}

// ---------- 辅助工具 ----------

pub fn load_profile(name: &str) -> Option<Profile> {
    let path = profile_path(name);
    if path.exists() {
        let content = fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    } else {
        None
    }
}

pub fn resolve_api_key(raw: &str) -> String {
    if raw.starts_with('$') {
        let var_name = &raw[1..];
        std::env::var(var_name).unwrap_or_else(|_| {
            eprintln!(
                "{} 环境变量 '{}' 未设置",
                "⚠️".yellow(),
                var_name
            );
            raw.to_string()
        })
    } else {
        raw.to_string()
    }
}

pub fn load_global_config() -> GlobalConfig {
    let path = global_config_path();
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        toml::from_str(&content).unwrap_or(GlobalConfig {
            default_profile: None,
        })
    } else {
        GlobalConfig {
            default_profile: None,
        }
    }
}

pub fn save_global_config(config: &GlobalConfig) {
    let path = global_config_path();
    let content = toml::to_string_pretty(config).expect("序列化失败");
    fs::write(&path, content).expect("写入全局配置失败");
}

fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        return "****".to_string();
    }
    format!("{}...{}", &key[..4], &key[key.len() - 4..])
}
