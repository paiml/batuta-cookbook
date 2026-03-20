//! Recipe 500-2: Banco Conversations & Prompt Presets
//!
//! **Level:** 500 (AI Serving)
//! **Category:** Model Serving
//! **Difficulty:** Intermediate
//!
//! ## Description
//!
//! Demonstrates Banco's conversation persistence and system prompt presets.
//! Conversations are stored server-side and auto-titled from the first message.
//! Prompt presets let you save and reuse system prompts via `@preset:name`.
//!
//! ## Prerequisites
//!
//! - Banco running: `batuta serve --banco --port 8090`
//!
//! ## Run This Recipe
//!
//! ```bash
//! cargo run --example recipe_500_2_banco_conversations
//! ```

fn main() {
    println!("=== Recipe 500-2: Banco Conversations & Prompt Presets ===\n");

    println!("--- Conversations ---\n");

    println!("1. Create a conversation");
    println!("   curl -X POST http://localhost:8090/api/v1/conversations \\");
    println!("     -H 'Content-Type: application/json' -d '{{}}'\n");

    println!("2. Chat within that conversation");
    println!("   curl -X POST http://localhost:8090/api/v1/chat/completions \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"messages\":[{{\"role\":\"user\",\"content\":\"Hello!\"}}],");
    println!("          \"conversation_id\":\"conv-...\"}}'");
    println!("   Messages are appended to the conversation automatically.\n");

    println!("3. List conversations (most recent first)");
    println!("   curl http://localhost:8090/api/v1/conversations\n");

    println!("4. Get full conversation history");
    println!("   curl http://localhost:8090/api/v1/conversations/conv-...\n");

    println!("5. Delete a conversation");
    println!("   curl -X DELETE http://localhost:8090/api/v1/conversations/conv-...\n");

    println!("--- System Prompt Presets ---\n");

    println!("Built-in presets: coding, concise, tutor\n");

    println!("6. List all presets");
    println!("   curl http://localhost:8090/api/v1/prompts\n");

    println!("7. Use a preset in chat via @preset: reference");
    println!("   curl -X POST http://localhost:8090/api/v1/chat/completions \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"messages\":[");
    println!("       {{\"role\":\"system\",\"content\":\"@preset:coding\"}},");
    println!("       {{\"role\":\"user\",\"content\":\"Write fizzbuzz\"}}");
    println!("     ]}}'");
    println!("   The @preset:coding reference expands server-side.\n");

    println!("8. Create a custom preset");
    println!("   curl -X POST http://localhost:8090/api/v1/prompts \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"name\":\"Pirate\",\"content\":\"You are a pirate.\"}}'");

    println!("\n--- Ollama Compatibility ---\n");

    println!("Banco speaks Ollama protocol for tool compatibility:");
    println!("   curl http://localhost:8090/api/tags             # List models");
    println!("   curl -X POST http://localhost:8090/api/chat \\");
    println!("     -d '{{\"model\":\"local\",\"messages\":[{{\"role\":\"user\",\"content\":\"Hi\"}}]}}'");
}

#[cfg(test)]
mod tests {
    #[test]
    fn recipe_500_2_conversation_json_valid() {
        let body = r#"{"messages":[{"role":"user","content":"Hi"}],"conversation_id":"conv-123"}"#;
        let parsed: serde_json::Value = serde_json::from_str(body).expect("valid JSON");
        assert!(parsed["conversation_id"].is_string());
    }

    #[test]
    fn recipe_500_2_preset_ref_format() {
        let content = "@preset:coding";
        assert!(content.starts_with("@preset:"));
        let name = content.strip_prefix("@preset:").expect("strip");
        assert_eq!(name, "coding");
    }
}
