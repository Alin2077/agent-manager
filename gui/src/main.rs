#![windows_subsystem = "windows"]

use agent_manager_core::{config, launcher, models, scanner, skills};

// ── Agents ──

#[tauri::command]
fn scan_agents() -> Vec<models::AgentInfo> {
    let agents = scanner::get_agents();
    scanner::save_agents_cache(&agents);
    agents
}

#[tauri::command]
fn load_agents() -> Vec<models::AgentInfo> {
    let cached = scanner::load_cached_agents();
    if cached.is_empty() {
        // 首次运行，自动扫描
        let agents = scanner::get_agents();
        scanner::save_agents_cache(&agents);
        agents
    } else {
        cached
    }
}

// ── Profiles ──

#[tauri::command]
fn list_profiles() -> Vec<models::Profile> {
    config::get_all_profiles()
}

#[tauri::command]
fn get_profile(name: String) -> Option<models::Profile> {
    config::load_profile(&name)
}

#[tauri::command]
fn save_profile(profile: models::Profile) -> Result<(), String> {
    config::save_profile(&profile)
}

#[tauri::command]
fn delete_profile(name: String) -> Result<(), String> {
    config::delete_profile(&name)
}

#[tauri::command]
fn set_default_profile(name: String) -> Result<(), String> {
    let mut global = config::load_global_config();
    global.default_profile = Some(name);
    config::save_global_config(&global)
}

#[tauri::command]
fn get_default_profile() -> Option<String> {
    config::load_global_config().default_profile
}

// ── Skills ──

#[tauri::command]
fn list_skills() -> Vec<models::Skill> {
    skills::get_all_skills()
}

#[tauri::command]
fn get_skill(name: String) -> Option<models::Skill> {
    skills::load_skill(&name)
}

#[tauri::command]
fn save_skill(skill: models::Skill) -> Result<(), String> {
    skills::save_skill(&skill)
}

#[tauri::command]
fn delete_skill(name: String) -> Result<(), String> {
    skills::delete_skill(&name)
}

#[tauri::command]
fn scan_skills(agent: String) -> Vec<models::Skill> {
    scanner::scan_agent_skills(&agent)
}

// ── Launch ──

#[tauri::command]
fn launch_agent(agent: String, profile: Option<String>) -> Result<i32, String> {
    launcher::launch_agent(&agent, profile.as_deref())
}

// ── Settings ──

#[tauri::command]
fn get_version(app: tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}

#[tauri::command]
fn get_language() -> String {
    config::load_global_config().language
}

#[tauri::command]
fn save_language(lang: String) -> Result<(), String> {
    let mut global = config::load_global_config();
    global.language = lang;
    config::save_global_config(&global)
}

#[tauri::command]
async fn check_update(app: tauri::AppHandle) -> Result<String, String> {
    let current = app.package_info().version.to_string();
    let url = "https://api.github.com/repos/Alin2077/agent-manager/releases/latest";

    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("User-Agent", "agent-manager")
        .send()
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?;

    let json: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("解析失败: {}", e))?;

    let latest = json["tag_name"]
        .as_str()
        .unwrap_or("v0.0.0")
        .trim_start_matches('v');

    if latest != current {
        let html_url = json["html_url"].as_str().unwrap_or(
            "https://github.com/Alin2077/agent-manager/releases/latest"
        );
        webbrowser::open(html_url).ok();
        Ok(format!("发现新版本 v{}（当前 v{}），已打开下载页面", latest, current))
    } else {
        Ok(format!("已是最新版本 v{}", current))
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            scan_agents,
            load_agents,
            list_profiles,
            get_profile,
            save_profile,
            delete_profile,
            set_default_profile,
            get_default_profile,
            list_skills,
            get_skill,
            save_skill,
            delete_skill,
            scan_skills,
            launch_agent,
            get_version,
            get_language,
            save_language,
            check_update,
        ])
        .run(tauri::generate_context!())
        .expect("启动 GUI 失败");
}
