use crate::models::Skill;
use colored::Colorize;
use dialoguer::{Input, Editor};
use std::fs;
use std::path::PathBuf;

fn config_dir() -> PathBuf {
    dirs::home_dir()
        .expect("无法获取用户主目录")
        .join(".agent-manager")
}

fn skills_dir() -> PathBuf {
    let dir = config_dir().join("skills");
    fs::create_dir_all(&dir).expect("无法创建 skills 目录");
    dir
}

fn skill_path(name: &str) -> PathBuf {
    skills_dir().join(format!("{}.toml", name))
}

// ---------- Skill CRUD ----------

pub fn list_skills() {
    let dir = skills_dir();
    let entries = match fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => {
            println!("{}", "📭 还没有任何 Skill".yellow());
            return;
        }
    };

    let mut skills: Vec<Skill> = Vec::new();
    for entry in entries.flatten() {
        if entry.path().extension().map(|e| e == "toml").unwrap_or(false) {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                if let Ok(s) = toml::from_str::<Skill>(&content) {
                    skills.push(s);
                }
            }
        }
    }

    if skills.is_empty() {
        println!("{}", "📭 还没有任何 Skill".yellow());
        println!("  💡 使用 agent-manager skill add <name> 创建");
        return;
    }

    println!("{}", "📚 所有 Skills:".bold());
    println!();
    for s in &skills {
        // 截断 prompt 预览
        let preview = if s.prompt.len() > 60 {
            format!("{}...", &s.prompt[..60])
        } else {
            s.prompt.clone()
        };
        println!("  {} v{}  {}", s.name.cyan().bold(), s.version, preview.dimmed());
    }
    println!();
}

pub fn add_skill(name: &str) {
    let path = skill_path(name);
    if path.exists() {
        println!("{} Skill '{}' 已存在", "❌".red(), name);
        return;
    }

    println!("{} 创建 Skill: {}", "✨".bold(), name.cyan().bold());
    println!();

    let description: String = Input::new()
        .with_prompt("描述（一句话）")
        .interact_text()
        .unwrap();

    let version: String = Input::new()
        .with_prompt("版本号")
        .default("1.0".into())
        .interact_text()
        .unwrap();

    let prompt = if let Ok(s) = Editor::new().edit("") {
        s.unwrap_or_default()
    } else {
        Input::new()
            .with_prompt("Prompt 内容（输入 Skill 的完整 prompt）")
            .interact_text()
            .unwrap()
    };

    let skill = Skill {
        name: name.to_string(),
        description,
        version,
        prompt,
    };

    let content = toml::to_string_pretty(&skill).expect("序列化失败");
    fs::write(&path, content).expect("写入 Skill 失败");

    println!();
    println!("{} Skill '{}' 已创建", "✅".green(), name);
    println!("  路径: {}", path.display().to_string().dimmed());
}

pub fn show_skill(name: &str) {
    let path = skill_path(name);
    if !path.exists() {
        println!("{} Skill '{}' 不存在", "❌".red(), name);
        return;
    }

    let content = fs::read_to_string(&path).expect("读取 Skill 失败");
    let skill: Skill = toml::from_str(&content).expect("解析 Skill 失败");

    println!("{} Skill: {}", "📚".bold(), skill.name.cyan().bold());
    println!("  版本:     {}", skill.version);
    println!("  描述:     {}", skill.description);
    println!();
    println!("{}", "─── Prompt ───".bold());
    println!("{}", skill.prompt);
    println!("{}", "──────────────".bold());
}

pub fn edit_skill(name: &str) {
    let path = skill_path(name);
    if !path.exists() {
        println!("{} Skill '{}' 不存在", "❌".red(), name);
        return;
    }

    let content = fs::read_to_string(&path).expect("读取 Skill 失败");
    let mut skill: Skill = toml::from_str(&content).expect("解析 Skill 失败");

    println!("{} 编辑 Skill: {}", "✏️".bold(), name.cyan().bold());
    println!("  （直接回车保留原值）");
    println!();

    let description: String = Input::new()
        .with_prompt("描述")
        .default(skill.description.clone())
        .interact_text()
        .unwrap();
    skill.description = description;

    let version: String = Input::new()
        .with_prompt("版本号")
        .default(skill.version.clone())
        .interact_text()
        .unwrap();
    skill.version = version;

    let new_prompt = if let Ok(s) = Editor::new().edit(&skill.prompt) {
        s.unwrap_or(skill.prompt.clone())
    } else {
        Input::new()
            .with_prompt("Prompt 内容")
            .default(skill.prompt.clone())
            .interact_text()
            .unwrap()
    };
    skill.prompt = new_prompt;

    let content = toml::to_string_pretty(&skill).expect("序列化失败");
    fs::write(&path, content).expect("写入 Skill 失败");

    println!();
    println!("{} Skill '{}' 已更新", "✅".green(), name);
}

pub fn delete_skill(name: &str) {
    let path = skill_path(name);
    if !path.exists() {
        println!("{} Skill '{}' 不存在", "❌".red(), name);
        return;
    }

    fs::remove_file(&path).expect("删除 Skill 失败");
    println!("{} Skill '{}' 已删除", "🗑️".bold(), name);
}
