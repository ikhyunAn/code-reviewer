use std::collections::HashMap;

struct Config {
    llm: LLMConfig,
    agents: AgentConfig,
    input: InputConfig,
}

// Needed for HashMap Key
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum LlmApiProvider {
    OpenAi,
    Anthropic,
}

enum LlmModel {
    Deepseek,
    Qwen,
    Llama,
}

enum LLMConfig {
    Cloud {
        api_keys: HashMap<LlmApiProvider, String>,
    },
    OnDevice,
}

enum AgentBackend {
    Cloud {provider: LlmApiProvider, model: String},
    OnDevice { model: LlmModel},
}

struct AgentConfig {
    senior: AgentBackend,
    junior: AgentBackend,
    max_rounds: usize,
}

enum Input {
    File,
    Gitdiff,
    Gitpr,
}

struct InputConfig {
    input: Input,
    input_details: Option<String>,
}