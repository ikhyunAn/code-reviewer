# CodeReviewer

> **Status**: 🚧 Early Development - Not yet runnable

A multi-agent code review system that simulates real-world peer review discussions using heterogeneous LLM models, with support for both cloud-based and on-device (MacBook) inference.

## Overview

CodeReviewer introduces a novel approach to automated code review by orchestrating multiple LLM agents in different reviewer roles (Senior Developer, Junior Developer) that engage in multi-round discussions—just like human reviewers would. This collaborative approach aims to produce more nuanced, thorough, and reliable code reviews than single-agent systems.

### Why Multi-Agent?

Traditional automated code review tools use a single model to analyze code. CodeReviewer takes a different approach:

- **Role-Based Agents**: Each LLM assumes a specific role (Senior/Junior) with distinct perspectives and system prompts
- **Multi-Round Discussions**: Agents review each other's feedback, building consensus through iterative dialogue
- **Heterogeneous Models**: Different agents can use different models (cloud or local) optimized for their role
- **Consensus Detection**: The system identifies when agents reach agreement, mimicking real code review workflows

## Key Features (Planned)

- ✅ **Multi-Agent Pipeline**: Configurable conversation flow between Senior and Junior developer agents
- ✅ **Hybrid Inference**: Seamlessly switch between cloud APIs (OpenAI, Anthropic) and on-device models
- ✅ **On-Device First**: Optimized for Apple Silicon with quantized models (GGUF) and GPU acceleration
- ✅ **Multiple Input Sources**: Support for Git diffs, local files, and GitHub Pull Requests
- ✅ **Conversation Memory**: Intelligent context management for long discussions
- ✅ **Consensus Detection**: Automatic detection of agreement between agents
- ✅ **Streaming Output**: Real-time feedback as agents discuss code

## Architecture

```
┌─────────────────────────────────────────────────────┐
│              UI Layer (Future: Tauri 2.0)           │
├─────────────────────────────────────────────────────┤
│          Multi-Agent Pipeline Coordinator           │
│   ┌──────────────────────────────────────────┐      │
│   │ Senior Agent ←→ Junior Agent             │      │
│   │ • Conversation State Machine             │      │
│   │ • Round Management (up to N rounds)      │      │
│   │ • Consensus Detection                    │      │
│   └──────────────────────────────────────────┘      │
├─────────────────────────────────────────────────────┤
│           LLM Backend Abstraction Layer             │
│   ┌──────────────┬───────────────────────────┐      │
│   │ Cloud APIs   │   On-Device Inference     │      │
│   │ - OpenAI     │   - Candle (GGUF)         │      │
│   │ - Anthropic  │   - Metal Acceleration    │      │
│   │              │   - CoreML (future)       │      │
│   └──────────────┴───────────────────────────┘      │
├─────────────────────────────────────────────────────┤
│              Code Input Layer                       │
│   • Git Diffs (git2) • Files • GitHub PRs           │
└─────────────────────────────────────────────────────┘
```

## Project Structure (Planned)

```
code-reviewer/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   │
│   ├── agents/
│   │   ├── mod.rs
│   │   ├── agent.rs              // Agent trait & implementation
│   │   ├── roles.rs              // Senior/Junior role definitions
│   │   ├── pipeline.rs           // Multi-round discussion coordinator
│   │   └── consensus.rs          // Agreement detection logic
│   │
│   ├── llm/
│   │   ├── mod.rs
│   │   ├── provider.rs           // LLMProvider trait
│   │   ├── cloud.rs              // Cloud backend (llm-connector)
│   │   ├── candle_backend.rs     // Candle on-device
│   │   ├── coreml_backend.rs     // CoreML (future)
│   │   └── config.rs             // Backend configuration
│   │
│   ├── code_input/
│   │   ├── mod.rs
│   │   ├── source.rs             // CodeSource trait
│   │   ├── git.rs                // Git diff handling (git2)
│   │   ├── files.rs              // File reader
│   │   └── github.rs             // GitHub PR fetcher (octocrab)
│   │
│   ├── conversation/
│   │   ├── mod.rs
│   │   ├── memory.rs             // Conversation state management
│   │   ├── message.rs            // Message types
│   │   └── state_machine.rs      // Discussion state transitions
│   │
│   ├── prompts/
│   │   ├── mod.rs
│   │   ├── senior_prompts.rs     // Senior developer system prompts
│   │   └── junior_prompts.rs     // Junior developer system prompts
│   │
│   └── cli/
│   │   ├── mod.rs
│   │   └── commands.rs           // CLI interface
│   │
│   │
│   └── config/
│   │   └── mod.rs                // Contains Rust code to load, parse, and manage config
│
├── models/                       // TBD: Quantized models directory (gitignored)
├── config/
│   └── config.toml               // Application configuration
└── tests/
    ├── integration/
    └── unit/
```

## Tech Stack

| Component | Technology | Rationale |
|-----------|------------|-----------|
| **Language** | Rust 2021 | Memory safety, performance, async support |
| **Cloud APIs** | llm-connector | Unified interface for OpenAI/Anthropic |
| **On-Device** | Candle + Candle-Transformers | Pure Rust, GGUF support, Metal acceleration |
| **Git Integration** | git2 | Mature libgit2 bindings |
| **GitHub API** | octocrab | Async PR/issue management |
| **Async Runtime** | tokio | Industry standard |
| **Future UI** | Tauri 2.0 | Lightweight, native macOS integration |

## Recommended Models for On-Device

- **StarCoder2-7B** (Q5_K_M): Specialized for code understanding
- **CodeLlama-7B-Instruct** (Q5_K_M): Good instruction following
- **Qwen2.5-Coder-7B** (Q5_K_M): Latest, excellent code comprehension
- **DeepSeek-Coder-6.7B** (Q5_K_M): Strong coding capabilities

_All models will use Q5_K_M quantization for optimal size/quality balance (~1.6GB per model)_

## Roadmap

### Phase 1: Foundation
- [x] Architecture design
- [ ] Core abstractions (LLMProvider, Agent, CodeSource traits)
- [ ] Cloud API integration
- [ ] Git diff parsing
- [ ] Basic 2-agent conversation (1 round)
- [ ] CLI interface

### Phase 2: Multi-Round Pipeline
- [ ] Conversation state machine
- [ ] Memory management (sliding window)
- [ ] Multi-round discussion logic (up to 5 rounds)
- [ ] Consensus detection algorithm
- [ ] GitHub PR input support

### Phase 3: On-Device Inference
- [ ] Candle integration for GGUF models
- [ ] Model download/caching system
- [ ] Metal GPU acceleration
- [ ] Performance benchmarking

### Phase 4: CoreML Optimization
- [ ] Apple Neural Engine support
- [ ] Model conversion pipeline
- [ ] Performance tuning

### Phase 5: UI & Polish
- [ ] Tauri 2.0 desktop application
- [ ] Real-time streaming output
- [ ] Review history & export (Markdown/PDF)
- [ ] Settings UI

### Future Enhancements
- [ ] Product Manager agent (3rd role)
- [ ] Fine-tuned models on code review datasets
- [ ] GitHub App for automatic PR reviews
- [ ] Vector database integration for codebase context
- [ ] Web interface

## Performance Targets: TBD

| Metric | Cloud | Local (Q5_K_M) |
|--------|-------|----------------|
| First Token | 200-500ms | 1-3s |
| Throughput | 50-100 tok/s | 20-40 tok/s |
| Memory | <500MB | 2-3GB |
| Full Review (500 LOC) | 30-60s | 2-4min |

## Contributing

This project is in early development. Contributions, ideas, and feedback are welcome once the foundation is stable.

## License

_To be determined_

---

**Author**: [ikhyunAn](https://github.com/ikhyunAn)

**Last Updated**: October 2025