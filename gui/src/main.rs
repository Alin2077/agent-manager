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

fn main() {
    tauri::Builder::default()
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
        ])
        .run(tauri::generate_context!())
        .expect("启动 GUI 失败");
}
