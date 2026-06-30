use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Agent 产品类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentFormat {
    #[serde(rename = "openai")]
    OpenAI,
    #[serde(rename = "claude-code")]
    ClaudeCode,
    #[serde(rename = "custom")]
    Custom,
}

impl std::fmt::Display for AgentFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentFormat::OpenAI => write!(f, "openai"),
            AgentFormat::ClaudeCode => write!(f, "claude-code"),
            AgentFormat::Custom => write!(f, "custom"),
        }
    }
}

/// 扫描到的已安装 Agent 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    /// 显示名称，如 "Claude Code"
    pub name: String,
    /// 二进制名称，如 "claude"
    pub binary: String,
    /// 安装路径
    pub path: String,
    /// 版本号（如果能获取到）
    pub version: Option<String>,
    /// 支持的配置格式
    pub formats: Vec<AgentFormat>,
}

/// Agent 配置 Profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Profile 名称（用作文件名）
    pub name: String,
    /// 配置格式
    pub format: AgentFormat,
    /// API Base URL
    pub base_url: String,
    /// 模型名称
    pub model: String,
    /// API Key（明文或 $ENV_VAR 引用）
    pub api_key: String,
    /// 最大 Token 数
    pub max_tokens: Option<u32>,
    /// 其他自定义参数
    #[serde(default)]
    pub extra: HashMap<String, String>,
}

/// Skill 定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub version: String,
    pub prompt: String,
}

/// 全局配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub default_profile: Option<String>,
}
