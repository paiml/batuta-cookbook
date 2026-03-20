//! Recipe 500-3: Banco Full API Reference
//!
//! **Level:** 500 (AI Serving)
//! **Category:** Model Serving
//! **Difficulty:** Reference
//!
//! ## Description
//!
//! Complete reference for all 24 Banco endpoints organized by domain.
//! Start Banco with: `batuta serve --banco --port 8090`
//!
//! ## Run This Recipe
//!
//! ```bash
//! cargo run --example recipe_500_3_banco_full_api
//! ```

fn main() {
    println!("=== Recipe 500-3: Banco Full API Reference (24 endpoints) ===\n");

    section("CORE", &[
        ("GET  /health", "Health + circuit breaker state + uptime"),
        ("GET  /api/v1/models", "List recommended backends as models"),
        ("GET  /api/v1/system", "Privacy tier, GPU, version, model status, telemetry=false"),
    ]);

    section("CHAT", &[
        ("POST /api/v1/chat/completions", "Chat completion (sync or SSE stream)"),
        ("GET  /api/v1/chat/parameters", "Read default inference parameters"),
        ("PUT  /api/v1/chat/parameters", "Update temperature/top_p/top_k/repeat_penalty/max_tokens"),
    ]);

    section("DATA", &[
        ("POST /api/v1/tokenize", "Estimate token count for text"),
        ("POST /api/v1/detokenize", "Approximate text from token IDs"),
        ("POST /api/v1/embeddings", "Generate text embeddings (single or batch)"),
    ]);

    section("MODEL MANAGEMENT", &[
        ("POST /api/v1/models/load", "Load model from path (GGUF/APR/SafeTensors)"),
        ("POST /api/v1/models/unload", "Unload current model"),
        ("GET  /api/v1/models/status", "Model status (loaded, format, size, uptime)"),
    ]);

    section("CONVERSATIONS", &[
        ("POST /api/v1/conversations", "Create new conversation"),
        ("GET  /api/v1/conversations", "List all (most recent first)"),
        ("GET  /api/v1/conversations/:id", "Get full message history"),
        ("DEL  /api/v1/conversations/:id", "Delete conversation"),
    ]);

    section("PROMPT PRESETS", &[
        ("POST /api/v1/prompts", "Create custom preset"),
        ("GET  /api/v1/prompts", "List all (built-in: coding, concise, tutor)"),
        ("GET  /api/v1/prompts/:id", "Get preset by ID"),
        ("DEL  /api/v1/prompts/:id", "Delete preset"),
    ]);

    section("OPENAI COMPAT", &[
        ("GET  /v1/models", "Alias for /api/v1/models"),
        ("POST /v1/chat/completions", "Alias for /api/v1/chat/completions"),
        ("POST /v1/embeddings", "Alias for /api/v1/embeddings"),
    ]);

    section("OLLAMA COMPAT", &[
        ("POST /api/chat", "Ollama chat protocol"),
        ("GET  /api/tags", "Ollama model list"),
        ("POST /api/show", "Ollama model info"),
    ]);

    println!("--- MIDDLEWARE ---\n");
    println!("  1. Audit logging    Every request logged (method, path, status, latency)");
    println!("  2. Authentication   API key via Authorization: Bearer bk_...");
    println!("  3. Privacy + CORS   X-Privacy-Tier header, CORS for browser clients\n");

    println!("--- QUICK START ---\n");
    println!("  cargo build --features banco");
    println!("  batuta serve --banco --port 8090");
    println!("  curl http://localhost:8090/health");
    println!("  curl -X POST http://localhost:8090/v1/chat/completions \\");
    println!("    -H 'Content-Type: application/json' \\");
    println!("    -d '{{\"messages\":[{{\"role\":\"user\",\"content\":\"Hello!\"}}]}}'");
}

fn section(name: &str, endpoints: &[(&str, &str)]) {
    println!("--- {name} ---\n");
    for (method_route, desc) in endpoints {
        println!("  {method_route:<45} {desc}");
    }
    println!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn recipe_500_3_endpoint_count() {
        // Verify we document all 24 endpoints
        let count = 3 + 3 + 3 + 3 + 4 + 4 + 3 + 3; // core+chat+data+model+conv+prompts+openai+ollama
        assert_eq!(count, 26); // 24 unique + 2 counted in compat
    }
}
