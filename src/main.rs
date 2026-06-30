mod config;
mod launcher;
mod models;
mod scanner;
mod skills;

use clap::{Parser, Subcommand};

/// Agent Manager — 管理多个 AI Agent 产品、配置和 Skills
#[derive(Parser)]
#[command(name = "agent-manager", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 扫描电脑上已安装的 Agent 产品
    Scan {
        /// 显示详细扫描信息
        #[arg(short, long)]
        verbose: bool,
    },

    /// 管理 Agent 配置 Profile
    #[command(subcommand)]
    Profile(ProfileCmd),

    /// 管理 Skills
    #[command(subcommand)]
    Skill(SkillCmd),

    /// 以指定配置启动 Agent
    Launch {
        /// Agent 产品名 (binary name)
        agent: String,
        /// 使用的 Profile 名称，不指定则用 default
        #[arg(short, long)]
        profile: Option<String>,
    },
}

#[derive(Subcommand)]
enum ProfileCmd {
    /// 列出所有 Profile
    List,
    /// 创建新 Profile（交互式）
    Add {
        /// Profile 名称
        name: String,
    },
    /// 查看 Profile 详情
    Show {
        /// Profile 名称
        name: String,
    },
    /// 编辑已有 Profile（交互式）
    Edit {
        /// Profile 名称
        name: String,
    },
    /// 删除 Profile
    Delete {
        /// Profile 名称
        name: String,
    },
    /// 设置默认 Profile
    SetDefault {
        /// Profile 名称
        name: String,
    },
}

#[derive(Subcommand)]
enum SkillCmd {
    /// 列出所有 Skills
    List,
    /// 创建新 Skill（交互式）
    Add {
        /// Skill 名称
        name: String,
    },
    /// 查看 Skill 内容
    Show {
        /// Skill 名称
        name: String,
    },
    /// 编辑已有 Skill
    Edit {
        /// Skill 名称
        name: String,
    },
    /// 删除 Skill
    Delete {
        /// Skill 名称
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Scan { verbose } => {
            scanner::scan_agents(*verbose);
        }
        Command::Profile(cmd) => match cmd {
            ProfileCmd::List => config::list_profiles(),
            ProfileCmd::Add { name } => config::add_profile(name),
            ProfileCmd::Show { name } => config::show_profile(name),
            ProfileCmd::Edit { name } => config::edit_profile(name),
            ProfileCmd::Delete { name } => config::delete_profile(name),
            ProfileCmd::SetDefault { name } => config::set_default_profile(name),
        },
        Command::Skill(cmd) => match cmd {
            SkillCmd::List => skills::list_skills(),
            SkillCmd::Add { name } => skills::add_skill(name),
            SkillCmd::Show { name } => skills::show_skill(name),
            SkillCmd::Edit { name } => skills::edit_skill(name),
            SkillCmd::Delete { name } => skills::delete_skill(name),
        },
        Command::Launch { agent, profile } => {
            launcher::launch_agent(agent, profile.as_deref());
        }
    }
}
