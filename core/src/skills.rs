use crate::models::Skill;
use std::fs;
use std::path::PathBuf;

fn config_dir() -> PathBuf {
    dirs::home_dir()
        .expect("无法获取用户主目录")
        .join(".agent-manager")
}

fn skills_dir() -> PathBuf {
    let dir = config_dir().join("skills");
    fs::create_dir_all(&dir).ok();
    dir
}

fn skill_path(name: &str) -> PathBuf {
    skills_dir().join(format!("{}.toml", name))
}

/// 列出所有 Skills
pub fn get_all_skills() -> Vec<Skill> {
    let dir = skills_dir();
    let mut skills: Vec<Skill> = Vec::new();
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            if entry.path().extension().map(|e| e == "toml").unwrap_or(false) {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Ok(s) = toml::from_str::<Skill>(&content) {
                        skills.push(s);
                    }
                }
            }
        }
    }
    skills
}

/// 加载单个 Skill
pub fn load_skill(name: &str) -> Option<Skill> {
    let path = skill_path(name);
    if path.exists() {
        let content = fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    } else {
        None
    }
}

/// 保存 Skill
pub fn save_skill(skill: &Skill) -> Result<(), String> {
    let path = skill_path(&skill.name);
    let content = toml::to_string_pretty(skill).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(())
}

/// 删除 Skill
pub fn delete_skill(name: &str) -> Result<(), String> {
    let path = skill_path(name);
    if !path.exists() {
        return Err(format!("Skill '{}' 不存在", name));
    }
    fs::remove_file(&path).map_err(|e| e.to_string())?;
    Ok(())
}

/// 检查 Skill 是否存在
pub fn skill_exists(name: &str) -> bool {
    skill_path(name).exists()
}
