use crate::models::{GlobalConfig, Profile};
use std::fs;
use std::path::PathBuf;

fn config_dir() -> PathBuf {
    let dir = dirs::home_dir()
        .expect("无法获取用户主目录")
        .join(".agent-manager");
    fs::create_dir_all(&dir).ok();
    dir
}

fn profiles_dir() -> PathBuf {
    let dir = config_dir().join("profiles");
    fs::create_dir_all(&dir).ok();
    dir
}

fn profile_path(name: &str) -> PathBuf {
    profiles_dir().join(format!("{}.toml", name))
}

fn global_config_path() -> PathBuf {
    config_dir().join("config.toml")
}

// ── Profile 数据操作 ──

/// 列出所有 Profile
pub fn get_all_profiles() -> Vec<Profile> {
    let dir = profiles_dir();
    let mut profiles: Vec<Profile> = Vec::new();
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            if entry.path().extension().map(|e| e == "toml").unwrap_or(false) {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Ok(p) = toml::from_str::<Profile>(&content) {
                        profiles.push(p);
                    }
                }
            }
        }
    }
    profiles
}

/// 加载单个 Profile
pub fn load_profile(name: &str) -> Option<Profile> {
    let path = profile_path(name);
    if path.exists() {
        let content = fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    } else {
        None
    }
}

/// 保存 Profile（新建或覆盖）
pub fn save_profile(profile: &Profile) -> Result<(), String> {
    let path = profile_path(&profile.name);
    let content = toml::to_string_pretty(profile).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(())
}

/// 删除 Profile
pub fn delete_profile(name: &str) -> Result<(), String> {
    let path = profile_path(name);
    if !path.exists() {
        return Err(format!("Profile '{}' 不存在", name));
    }
    fs::remove_file(&path).map_err(|e| e.to_string())?;

    // 如果删除的是默认 profile，清除默认
    let mut global = load_global_config();
    if global.default_profile.as_deref() == Some(name) {
        global.default_profile = None;
        save_global_config(&global)?;
    }
    Ok(())
}

/// 检查 Profile 是否存在
pub fn profile_exists(name: &str) -> bool {
    profile_path(name).exists()
}

// ── 全局配置 ──

pub fn load_global_config() -> GlobalConfig {
    let path = global_config_path();
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        toml::from_str(&content).unwrap_or(GlobalConfig {
            default_profile: None,
            language: "zh-CN".into(),
        })
    } else {
        GlobalConfig {
            default_profile: None,
            language: "zh-CN".into(),
        }
    }
}

pub fn save_global_config(config: &GlobalConfig) -> Result<(), String> {
    let path = global_config_path();
    let content = toml::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(())
}

// ── API Key 解析 ──

/// 解析 API Key：如果以 $ 开头，读取环境变量
pub fn resolve_api_key(raw: &str) -> String {
    if raw.starts_with('$') {
        let var_name = &raw[1..];
        std::env::var(var_name).unwrap_or_else(|_| raw.to_string())
    } else {
        raw.to_string()
    }
}

/// 安全展示 Key（脱敏）
pub fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        return "****".to_string();
    }
    format!("{}...{}", &key[..4], &key[key.len() - 4..])
}
