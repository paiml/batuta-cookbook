//! Recipe 500-4: Banco Inference — Real Token Generation
//!
//! This recipe demonstrates Banco's inference pipeline with a loaded GGUF model.
//! When built with `--features banco,inference`, Banco uses realizar's
//! `forward_single_with_cache()` for autoregressive token generation.
//!
//! ```bash
//! # Start with a model
//! batuta serve --banco --model ./tinyllama.gguf --port 8090
//!
//! # Sync chat (real tokens from loaded model)
//! curl -X POST http://localhost:8090/v1/chat/completions \
//!   -H "Content-Type: application/json" \
//!   -d '{"messages":[{"role":"user","content":"Hello!"}],"max_tokens":50}'
//!
//! # Streaming (SSE, one token per event)
//! curl -N -X POST http://localhost:8090/v1/chat/completions \
//!   -H "Content-Type: application/json" \
//!   -d '{"messages":[{"role":"user","content":"Hello!"}],"stream":true,"max_tokens":50}'
//!
//! # Adjust sampling parameters
//! curl -X PUT http://localhost:8090/api/v1/chat/parameters \
//!   -H "Content-Type: application/json" \
//!   -d '{"temperature":0.3,"top_k":20,"max_tokens":128}'
//!
//! # Check model status
//! curl http://localhost:8090/api/v1/models/status
//! ```
//!
//! ## Inference Pipeline
//!
//! 1. **Tokenize**: Encode prompt via BPE tokenizer (merge rules) or greedy fallback
//! 2. **Prefill**: Process all prompt tokens through KV cache
//! 3. **Decode**: Autoregressive loop — forward → sample → emit token
//! 4. **Stop**: EOS token or max_tokens reached
//!
//! ## Tokenizer Loading
//!
//! Banco searches for a proper BPE tokenizer alongside the model:
//! 1. `{model-stem}.tokenizer.json` (e.g., `model.tokenizer.json`)
//! 2. `tokenizer.json` in the same directory
//!
//! With a BPE tokenizer: correct merge rules, production-quality tokenization.
//! Without: greedy longest-match fallback (approximate).
//!
//! ## Model Formats
//!
//! | Format | Extension | Status |
//! |--------|-----------|--------|
//! | GGUF | `.gguf` | Full inference via `MappedGGUFModel` |
//! | APR | `.apr` | Full inference via `OwnedQuantizedModel::from_apr()` |
//! | SafeTensors | `.safetensors` | Metadata only (no inference yet) |
//!
//! ## Sampling Modes
//!
//! - **Greedy** (temperature=0): Always picks highest-probability token
//! - **Top-k** (temperature>0, top_k>1): Sample from top-k tokens with
//!   temperature scaling and softmax normalization
//!
//! ## Feature Flags
//!
//! | Feature | What |
//! |---------|------|
//! | `banco` | HTTP API skeleton, echo mode |
//! | `banco,inference` | Real inference via realizar |
//! | `banco,inference,ml` | Inference + proper BPE tokenizer from aprender |
//!
//! Without `inference`, chat returns a dry-run echo describing the routing
//! decision. With `inference` + a loaded model, chat returns real generated text.
//! Adding `ml` enables BPE tokenizer loading from sibling `tokenizer.json`.

fn main() {
    println!("Recipe 500-4: Banco Inference");
    println!();
    println!("Build and run:");
    println!("  cargo run --features banco,inference,ml -- serve --banco --model ./model.gguf");
    println!();
    println!("Tokenizer search:");
    println!("  1. {{stem}}.tokenizer.json  (sibling file)");
    println!("  2. tokenizer.json           (same directory)");
    println!("  3. Greedy fallback           (no tokenizer.json found)");
    println!();
    println!("Inference pipeline:");
    println!("  prompt → BPE tokenize → prefill KV cache → decode loop → response");
    println!();
    println!("Supported formats: .gguf (GGUF), .apr (APR v2)");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_recipe_500_4_compiles() {
        // Smoke test — the recipe describes CLI usage, not library code
        super::main();
    }
}
