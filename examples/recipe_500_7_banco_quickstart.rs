//! Recipe 500-7: Banco Quickstart — Local AI Workbench in 60 Seconds
//!
//! ```bash
//! # Install (banco includes BPE tokenizer, Arrow data, training/merge)
//! cargo install batuta --features banco
//!
//! # Start (no model — explore the API)
//! batuta serve --banco --port 8090
//!
//! # Start with a model (real inference — add inference feature)
//! cargo install batuta --features banco,inference
//! batuta serve --banco --model ./model.gguf --port 8090
//!
//! # Start with APR model (our native format)
//! batuta serve --banco --model ./model.apr --port 8090
//!
//! # === Chat ===
//! curl -X POST http://localhost:8090/v1/chat/completions \
//!   -H "Content-Type: application/json" \
//!   -d '{"messages":[{"role":"user","content":"Hello!"}]}'
//!
//! # === Upload + RAG ===
//! curl -X POST http://localhost:8090/api/v1/data/upload/json \
//!   -d '{"name":"docs.txt","content":"Your knowledge base..."}'
//! # (auto-indexed for RAG — search immediately)
//! curl -X POST http://localhost:8090/v1/chat/completions \
//!   -d '{"messages":[{"role":"user","content":"search my docs"}],"rag":true}'
//!
//! # === Conversations ===
//! curl http://localhost:8090/api/v1/conversations
//! curl "http://localhost:8090/api/v1/conversations/search?q=hello"
//!
//! # === Data Pipeline ===
//! curl -X POST http://localhost:8090/api/v1/data/recipes \
//!   -d '{"name":"prep","steps":[{"type":"chunk","config":{"max_tokens":512}}]}'
//!
//! # === System Status ===
//! curl http://localhost:8090/api/v1/system
//! curl http://localhost:8090/api/v1/rag/status
//! curl "http://localhost:8090/api/v1/audit?limit=10"
//! ```
//!
//! ## What You Get
//!
//! | Feature | Without Model | With Model |
//! |---------|--------------|------------|
//! | Chat | Helpful guidance | Real tokens |
//! | RAG | BM25 search | BM25 + model context |
//! | Tokenize | ~4 chars/token heuristic | Real vocabulary |
//! | Embeddings | 128-dim hash | Model embedding layer |
//! | Eval | "no_model" status | Real perplexity |
//! | Batch | Dry-run echo | Real inference |
//!
//! ## Compatibility
//!
//! Works with: OpenAI Python SDK, LangChain, Ollama tools (Open WebUI, Aider, Continue.dev)
//!
//! ## Persistence
//!
//! All data in `~/.banco/` — survives restarts.

fn main() {
    println!("Recipe 500-7: Banco Quickstart");
    println!("  cargo install batuta --features banco");
    println!("  batuta serve --banco --port 8090");
    println!();
    println!("82 endpoints, 345 L1 + 50 L2 = 395 tests, zero cloud dependency");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_recipe_500_7_compiles() {
        super::main();
    }
}
