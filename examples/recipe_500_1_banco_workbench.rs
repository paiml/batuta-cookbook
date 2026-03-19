//! Recipe 500-1: Banco AI Workbench API
//!
//! **Level:** 500 (AI Serving)
//! **Category:** Model Serving
//! **Difficulty:** Intermediate
//!
//! ## Description
//!
//! This recipe demonstrates how to interact with the Banco AI workbench HTTP API.
//! Banco is started with `batuta serve --banco` and exposes OpenAI-compatible
//! endpoints for chat completions, model listing, health checks, and system info.
//!
//! ## Prerequisites
//!
//! - Batuta built with `--features banco`
//! - Banco running: `batuta serve --banco --port 8090`
//!
//! ## Learning Objectives
//!
//! After completing this recipe, you will understand:
//! - How to check Banco health and system status
//! - How to list available models and backends
//! - How to send chat completions (sync and streaming)
//! - How to use the OpenAI SDK compatible `/v1/` routes
//! - How privacy tiers affect API behavior
//!
//! ## Run This Recipe
//!
//! ```bash
//! # Terminal 1: Start Banco
//! cargo run --features banco -- serve --banco --port 8090
//!
//! # Terminal 2: Run recipe
//! cargo run --example recipe_500_1_banco_workbench
//! ```

fn main() {
    println!("=== Recipe 500-1: Banco AI Workbench API ===\n");

    // Note: This recipe demonstrates the API calls you would make.
    // In production, use reqwest or the OpenAI SDK. Here we show curl equivalents.

    println!("1. Health Check");
    println!("   curl http://127.0.0.1:8090/health");
    println!("   Expected: {{\"status\":\"ok\",\"circuit_breaker_state\":\"closed\",\"uptime_secs\":...}}\n");

    println!("2. System Info");
    println!("   curl http://127.0.0.1:8090/api/v1/system");
    println!("   Expected: {{\"privacy_tier\":\"Standard\",\"backends\":[...],\"gpu_available\":true,\"version\":\"0.7.2\",\"telemetry\":false}}\n");

    println!("3. List Models");
    println!("   curl http://127.0.0.1:8090/api/v1/models");
    println!("   Expected: {{\"object\":\"list\",\"data\":[{{\"id\":\"realizar\",...}},...]}}\n");

    println!("4. Chat Completion (sync)");
    println!("   curl -X POST http://127.0.0.1:8090/api/v1/chat/completions \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"messages\":[{{\"role\":\"user\",\"content\":\"Hello!\"}}]}}'");
    println!("   Expected: {{\"id\":\"banco-...\",\"choices\":[{{\"finish_reason\":\"dry_run\",...}}],...}}\n");

    println!("5. Chat Completion (streaming SSE)");
    println!("   curl -X POST http://127.0.0.1:8090/api/v1/chat/completions \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"messages\":[{{\"role\":\"user\",\"content\":\"Hello!\"}}],\"stream\":true}}'");
    println!("   Expected: data: {{...}} lines ending with data: [DONE]\n");

    println!("6. OpenAI SDK Compatible Route");
    println!("   curl http://127.0.0.1:8090/v1/models");
    println!("   Same as /api/v1/models — drop-in for OpenAI SDK\n");

    println!("7. Privacy Header");
    println!("   Every response includes X-Privacy-Tier: standard|private|sovereign");
    println!("   In Sovereign mode, external backend hints are rejected with 403\n");

    // Demonstrate the types programmatically
    demonstrate_types();
}

/// Show the Rust types that Banco uses (available in the batuta crate)
fn demonstrate_types() {
    println!("=== Banco Request/Response Types ===\n");

    // Chat request (what you POST)
    let request = serde_json::json!({
        "model": "local",
        "messages": [
            {"role": "system", "content": "You are a helpful assistant."},
            {"role": "user", "content": "What is the Sovereign AI Stack?"}
        ],
        "max_tokens": 256,
        "temperature": 0.7,
        "top_p": 1.0,
        "stream": false
    });
    println!("Request:\n{}\n", serde_json::to_string_pretty(&request).expect("json"));

    // Chat response (what you GET back)
    let response = serde_json::json!({
        "id": "banco-1700000000",
        "object": "chat.completion",
        "created": 1700000000_u64,
        "model": "banco-echo",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "[banco dry-run] route=Local(Realizar) | model=local | prompt_len=2"
            },
            "finish_reason": "dry_run"
        }],
        "usage": {
            "prompt_tokens": 42,
            "completion_tokens": 20,
            "total_tokens": 62
        }
    });
    println!("Response:\n{}\n", serde_json::to_string_pretty(&response).expect("json"));

    println!("=== Configuration: ~/.banco/config.toml ===\n");
    let config = r#"[server]
host = "127.0.0.1"
port = 8090
privacy_tier = "standard"

[inference]
temperature = 0.7
top_p = 1.0
max_tokens = 256

[budget]
daily_limit_usd = 10.0
max_request_usd = 1.0
"#;
    println!("{config}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn recipe_500_1_types_are_valid_json() {
        // Verify the example JSON is valid
        let req = r#"{"messages":[{"role":"user","content":"Hi"}]}"#;
        let parsed: serde_json::Value = serde_json::from_str(req).expect("valid request JSON");
        assert!(parsed["messages"].is_array());
    }

    #[test]
    fn recipe_500_1_config_is_valid_toml() {
        let config = r#"
[server]
host = "127.0.0.1"
port = 8090
privacy_tier = "standard"

[inference]
temperature = 0.7
"#;
        let parsed: toml::Value = toml::from_str(config).expect("valid TOML");
        assert_eq!(parsed["server"]["port"].as_integer(), Some(8090));
    }
}
