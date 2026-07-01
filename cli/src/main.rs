use agent_manager_core::config;
use agent_manager_core::launcher;
use agent_manager_core::models::{AgentFormat, Profile, Skill};
use agent_manager_core::scanner;
use agent_manager_core::skills;
use clap::{Parser, Subcommand};
use colored::Colorize;
use dialoguer::{Input, Select};
use std::collections::HashMap;

/// Agent Manager — 管理多个 AI Agent 产品、配置和 Skills
#[derive(Parser)]
#[command(name = "agent-manager", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Scan {
        #[arg(short, long)]
        verbose: bool,
    },
    #[command(subcommand)]
    Profile(ProfileCmd),
    #[command(subcommand)]
    Skill(SkillCmd),
    Launch {
        agent: String,
        #[arg(short, long)]
        profile: Option<String>,
    },
}

#[derive(Subcommand)]
enum ProfileCmd {
    List,
    Add { name: String },
    Show { name: String },
    Edit { name: String },
    Delete { name: String },
    SetDefault { name: String },
}

#[derive(Subcommand)]
enum SkillCmd {
    List,
    Add { name: String },
    Show { name: String },
    Edit { name: String },
    Delete { name: String },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Command::Scan { verbose }) => cmd_scan(*verbose),
        Some(Command::Profile(cmd)) => match cmd {
            ProfileCmd::List => cmd_profile_list(),
            ProfileCmd::Add { name } => cmd_profile_add(name),
            ProfileCmd::Show { name } => cmd_profile_show(name),
            ProfileCmd::Edit { name } => cmd_profile_edit(name),
            ProfileCmd::Delete { name } => cmd_profile_delete(name),
            ProfileCmd::SetDefault { name } => cmd_profile_set_default(name),
        },
        Some(Command::Skill(cmd)) => match cmd {
            SkillCmd::List => cmd_skill_list(),
            SkillCmd::Add { name } => cmd_skill_add(name),
            SkillCmd::Show { name } => cmd_skill_show(name),
            SkillCmd::Edit { name } => cmd_skill_edit(name),
            SkillCmd::Delete { name } => cmd_skill_delete(name),
        },
        Some(Command::Launch { agent, profile }) => cmd_launch(agent, profile.as_deref()),
        None => {
            // 双击启动或无参数：显示帮助并等待按键
            println!("Agent Manager v{}", env!("CARGO_PKG_VERSION"));
            println!("在终端中使用以下命令：");
            println!();
            println!("  agent-manager scan         扫描已安装的 Agent");
            println!("  agent-manager profile list  列出所有 Profile");
            println!("  agent-manager skill list    列出所有 Skills");
            println!("  agent-manager launch <name> 启动 Agent");
            println!();
            println!("按任意键退出...");
            let _ = std::io::stdin().read_line(&mut String::new());
        }
    }
}

// ── scan ──

fn cmd_scan(verbose: bool) {
    println!("{}", "正在扫描已安装的 Agent...".bold());
    println!();

    let agents = scanner::get_agents();

    if agents.is_empty() {
        println!("  未检测到任何 Agent");
        return;
    }

    if verbose {
        for agent in &agents {
            println!(
                "  {}  {}  {}",
                agent.name.cyan(),
                agent.binary.yellow(),
                agent.path.dimmed()
            );
        }
        println!();
    }

    println!("{}", "检测到以下 Agent:".green().bold());
    println!();
    println!("  {:<20} {:<16} {:<15} {}", "名称", "命令", "格式", "路径");
    println!("  {:-<20} {:-<16} {:-<15} {:-<30}", "", "", "", "");

    for agent in &agents {
        let formats_str = agent
            .formats
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!(
            "  {:<20} {:<16} {:<15} {}",
            agent.name.cyan(),
            agent.binary.yellow(),
            formats_str,
            agent.path.dimmed()
        );
    }
    println!();
}

// ── profile ──

fn cmd_profile_list() {
    let profiles = config::get_all_profiles();
    if profiles.is_empty() {
        println!("{}", "还没有任何 Profile".yellow());
        return;
    }
    let global = config::load_global_config();
    println!("{}", "所有 Profile:".bold());
    println!();
    for p in &profiles {
        let marker = if Some(p.name.clone()) == global.default_profile {
            " *".yellow()
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

fn cmd_profile_add(name: &str) {
    if config::profile_exists(name) {
        println!("{} Profile '{}' 已存在", "X".red(), name);
        return;
    }

    println!("{} 创建 Profile: {}", "+".bold(), name.cyan().bold());
    println!();

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
        .with_prompt("API Key")
        .interact_text()
        .unwrap();

    let max_tokens: String = Input::new()
        .with_prompt("Max Tokens（留空使用默认）")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let profile = Profile {
        name: name.to_string(),
        agent: String::new(),
        agent_name: String::new(),
        format,
        base_url,
        model,
        api_key,
        max_tokens: max_tokens.parse().ok(),
        extra: HashMap::new(),
    };

    match config::save_profile(&profile) {
        Ok(()) => println!("{} Profile '{}' 已创建", "OK".green(), name),
        Err(e) => println!("{} 创建失败: {}", "X".red(), e),
    }
}

fn cmd_profile_show(name: &str) {
    match config::load_profile(name) {
        Some(p) => {
            println!("{} Profile: {}", ">".bold(), p.name.cyan().bold());
            println!("  格式:      {}", p.format.to_string().yellow());
            println!("  Base URL:  {}", p.base_url);
            println!("  模型:      {}", p.model);
            println!("  API Key:   {}", config::mask_key(&p.api_key));
            if let Some(mt) = p.max_tokens {
                println!("  Max Tokens: {}", mt);
            }
        }
        None => println!("{} Profile '{}' 不存在", "X".red(), name),
    }
}

fn cmd_profile_edit(name: &str) {
    let mut profile = match config::load_profile(name) {
        Some(p) => p,
        None => {
            println!("{} Profile '{}' 不存在", "X".red(), name);
            return;
        }
    };

    println!("编辑 Profile: {}", name.cyan().bold());
    println!("  （直接回车保留原值）");
    println!();

    profile.base_url = Input::new()
        .with_prompt("Base URL")
        .default(profile.base_url.clone())
        .interact_text()
        .unwrap();

    profile.model = Input::new()
        .with_prompt("模型名称")
        .default(profile.model.clone())
        .interact_text()
        .unwrap();

    profile.api_key = Input::new()
        .with_prompt("API Key")
        .default(profile.api_key.clone())
        .interact_text()
        .unwrap();

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

    match config::save_profile(&profile) {
        Ok(()) => println!("{} Profile '{}' 已更新", "OK".green(), name),
        Err(e) => println!("{} 更新失败: {}", "X".red(), e),
    }
}

fn cmd_profile_delete(name: &str) {
    match config::delete_profile(name) {
        Ok(()) => println!("{} Profile '{}' 已删除", "DEL".bold(), name),
        Err(e) => println!("{} {}", "X".red(), e),
    }
}

fn cmd_profile_set_default(name: &str) {
    if !config::profile_exists(name) {
        println!("{} Profile '{}' 不存在", "X".red(), name);
        return;
    }
    let mut global = config::load_global_config();
    global.default_profile = Some(name.to_string());
    match config::save_global_config(&global) {
        Ok(()) => println!("{} 默认 Profile = '{}'", "*".bold(), name.cyan()),
        Err(e) => println!("{} {}", "X".red(), e),
    }
}

// ── skill ──

fn cmd_skill_list() {
    let skills = skills::get_all_skills();
    if skills.is_empty() {
        println!("{}", "还没有任何 Skill".yellow());
        return;
    }
    println!("{}", "所有 Skills:".bold());
    println!();
    for s in &skills {
        let preview = if s.prompt.len() > 60 {
            format!("{}...", &s.prompt[..60])
        } else {
            s.prompt.clone()
        };
        println!("  {} v{}  {}", s.name.cyan().bold(), s.version, preview.dimmed());
    }
    println!();
}

fn cmd_skill_add(name: &str) {
    if skills::skill_exists(name) {
        println!("{} Skill '{}' 已存在", "X".red(), name);
        return;
    }

    println!("创建 Skill: {}", name.cyan().bold());
    println!();

    let description: String = Input::new()
        .with_prompt("描述")
        .interact_text()
        .unwrap();

    let version: String = Input::new()
        .with_prompt("版本号")
        .default("1.0".into())
        .interact_text()
        .unwrap();

    let prompt: String = Input::new()
        .with_prompt("Prompt 内容")
        .interact_text()
        .unwrap();

    let skill = Skill {
        name: name.to_string(),
        agent: String::new(),
        description,
        version,
        prompt,
    };

    match skills::save_skill(&skill) {
        Ok(()) => println!("{} Skill '{}' 已创建", "OK".green(), name),
        Err(e) => println!("{} {}", "X".red(), e),
    }
}

fn cmd_skill_show(name: &str) {
    match skills::load_skill(name) {
        Some(s) => {
            println!("Skill: {}", s.name.cyan().bold());
            println!("  版本: {}", s.version);
            println!("  描述: {}", s.description);
            println!();
            println!("{}", s.prompt);
        }
        None => println!("{} Skill '{}' 不存在", "X".red(), name),
    }
}

fn cmd_skill_edit(name: &str) {
    let mut skill = match skills::load_skill(name) {
        Some(s) => s,
        None => {
            println!("{} Skill '{}' 不存在", "X".red(), name);
            return;
        }
    };

    skill.description = Input::new()
        .with_prompt("描述")
        .default(skill.description.clone())
        .interact_text()
        .unwrap();

    skill.version = Input::new()
        .with_prompt("版本号")
        .default(skill.version.clone())
        .interact_text()
        .unwrap();

    skill.prompt = Input::new()
        .with_prompt("Prompt 内容")
        .default(skill.prompt.clone())
        .interact_text()
        .unwrap();

    match skills::save_skill(&skill) {
        Ok(()) => println!("{} Skill '{}' 已更新", "OK".green(), name),
        Err(e) => println!("{} {}", "X".red(), e),
    }
}

fn cmd_skill_delete(name: &str) {
    match skills::delete_skill(name) {
        Ok(()) => println!("{} Skill '{}' 已删除", "DEL".bold(), name),
        Err(e) => println!("{} {}", "X".red(), e),
    }
}

// ── launch ──

fn cmd_launch(agent: &str, profile_name: Option<&str>) {
    println!("正在启动 {}...", agent.cyan().bold());

    match launcher::launch_agent(agent, profile_name) {
        Ok(code) => {
            if code != 0 {
                println!("{} Agent 退出码: {}", "WARN".yellow(), code);
            }
        }
        Err(e) => {
            println!("{} {}", "X".red(), e);
        }
    }
}
