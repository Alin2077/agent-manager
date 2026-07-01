use agent_manager_core::{config, launcher, models, scanner, skills};

// ── Agents ──

#[tauri::command]
fn scan_agents() -> Vec<models::AgentInfo> {
    scanner::get_agents()
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

// ── Launch ──

#[tauri::command]
fn launch_agent(agent: String, profile: Option<String>) -> Result<i32, String> {
    launcher::launch_agent(&agent, profile.as_deref())
}

// ── Updater ──

#[tauri::command]
fn get_version(app: tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}

#[tauri::command]
async fn check_update(app: tauri::AppHandle) -> Result<String, String> {
    let updater = tauri_plugin_updater::UpdaterExt::updater(&app)
        .map_err(|e| e.to_string())?;
    match updater.check().await.map_err(|e| e.to_string())? {
        Some(update) => {
            let version = update.version.clone();
            update.download_and_install(
                |_chunk, _total| {},
                || {}
            ).await.map_err(|e| e.to_string())?;
            Ok(format!("已更新到 v{}，请重启应用", version))
        }
        None => Ok("已是最新版本".into()),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            scan_agents,
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
            launch_agent,
            get_version,
            check_update,
        ])
        .run(tauri::generate_context!())
        .expect("启动 GUI 失败");
}
