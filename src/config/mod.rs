use std::collections::HashMap;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::{env, fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    llm: LLMConfig,
    agents: AgentConfig,
    input: InputConfig,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error reading config: {0}")]
    Io(#[from] std::io::Error),
    #[error("Toml parse error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("Env variable error: {0}")]
    EnvVar(#[from] std::env::VarError),
    #[error("Env parse error: {0}")]
    EnvParse(String),
}

// Needed for HashMap Key
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
enum LlmApiProvider {
    OpenAi,
    Anthropic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum LlmModel {
    DeepSeek_Coder_V2_Lite_Instruct,
    Qwen2_5_Coder_7B,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum LLMConfig {
    Cloud {
        api_keys: HashMap<LlmApiProvider, String>,
    },
    OnDevice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum AgentBackend {
    Cloud {provider: LlmApiProvider, model: String},
    OnDevice { model: LlmModel},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgentConfig {
    senior: AgentBackend,
    junior: AgentBackend,
    max_rounds: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Input {
    File,
    Gitdiff,
    Gitpr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InputConfig {
    input: Input,
    input_details: Option<String>,
}



impl Config {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        // Load and parse TOML
        let path_ref = path.as_ref();
        let raw = fs::read_to_string(path_ref)?;
        let mut cfg: Self = toml::from_str(&raw)?;
        cfg.apply_env_overrides()?;
        Ok(cfg)
    }

    pub fn from_env() -> Result<Self, ConfigError> {
        // Load from .env or env vars
        let _ = dotenvy::dotenv();
        let mut cfg = Self {
            llm : LLMConfig::Cloud { api_keys: HashMap::new() },
            agents: AgentConfig{
                senior: AgentBackend::Cloud { provider: (LlmApiProvider::OpenAi), model: String::new() },
                junior: AgentBackend::Cloud { provider: (LlmApiProvider::OpenAi), model: String::new() },
                max_rounds: 5
            },
            input: InputConfig{
                input: Input::File,
                input_details: Some(String::new()),
            },
        };
        cfg.apply_env_overrides()?;
        Ok(cfg)
    }

    fn apply_env_overrides(&mut self) -> Result<(), ConfigError> {
        //
        Ok(())
    }
}