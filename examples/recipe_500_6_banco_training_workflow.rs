//! Recipe 500-6: Banco Training Workflow — Upload → Recipe → Train → Eval → Compare
//!
//! Complete Phase 3 workflow demonstrating the sovereign AI workbench loop:
//!
//! ```bash
//! # 1. Start Banco
//! batuta serve --banco --port 8090
//!
//! # 2. Upload training documents
//! curl -X POST http://localhost:8090/api/v1/data/upload/json \
//!   -H "Content-Type: application/json" \
//!   -d '{"name": "docs.txt", "content": "Your training data..."}'
//!
//! # 3. Create a data recipe
//! curl -X POST http://localhost:8090/api/v1/data/recipes \
//!   -H "Content-Type: application/json" \
//!   -d '{"name": "prep", "source_files": ["FILE-ID"],
//!        "steps": [
//!          {"type": "extract_text", "config": {}},
//!          {"type": "chunk", "config": {"max_tokens": 512}},
//!          {"type": "format", "config": {"template": "chatml"}}
//!        ]}'
//!
//! # 4. Run the recipe → produces dataset
//! curl -X POST http://localhost:8090/api/v1/data/recipes/RECIPE-ID/run
//!
//! # 5. Index docs for RAG
//! curl -X POST http://localhost:8090/api/v1/rag/index
//!
//! # 6. Evaluate base model
//! curl -X POST http://localhost:8090/api/v1/eval/perplexity \
//!   -d '{"text": "test text for evaluation"}'
//!
//! # 7a. Start LoRA training with preset
//! curl -X POST http://localhost:8090/api/v1/train/start \
//!   -d '{"dataset_id": "DS-ID", "preset": "standard-lora"}'
//!
//! # 7b. Or start with explicit config
//! curl -X POST http://localhost:8090/api/v1/train/start \
//!   -d '{"dataset_id": "DS-ID", "method": "lora",
//!        "config": {"lora_r": 16, "epochs": 3}}'
//!
//! # 7c. Stream training metrics (SSE)
//! curl http://localhost:8090/api/v1/train/runs/RUN-ID/metrics
//!
//! # 7d. Export trained adapter
//! curl -X POST http://localhost:8090/api/v1/train/runs/RUN-ID/export \
//!   -d '{"format": "safetensors", "merge": false}'
//!
//! # 7e. Merge fine-tuned models (TIES/DARE/SLERP)
//! curl -X POST http://localhost:8090/api/v1/models/merge \
//!   -d '{"models": ["run-a", "run-b"], "strategy": "slerp", "interpolation_t": 0.5}'
//!
//! # 8. Create experiment, add runs, compare
//! curl -X POST http://localhost:8090/api/v1/experiments \
//!   -d '{"name": "LoRA tuning"}'
//! curl -X POST http://localhost:8090/api/v1/experiments/EXP-ID/runs \
//!   -d '{"run_id": "RUN-ID"}'
//! curl http://localhost:8090/api/v1/experiments/EXP-ID/compare
//!
//! # 9. Chat with RAG for domain Q&A
//! curl -X POST http://localhost:8090/api/v1/chat/completions \
//!   -d '{"messages": [{"role": "user", "content": "question"}], "rag": true}'
//! ```
//!
//! ## Important: Simulated Training
//!
//! Training metrics are currently **simulated** (cosine decay schedule,
//! not real gradient-based loss). The response includes `"simulated": true`
//! to honestly indicate this. Real entrenar training loop integration is
//! a P2 work item. The API structure is complete and exercised by 38 L2 tests.
//!
//! ## Sovereign Guarantee
//!
//! In Sovereign mode, the entire workflow runs locally:
//! - No cloud APIs, no telemetry, no data egress
//! - Inference via realizar (GGUF/APR models)
//! - Training config via entrenar (LoRA/QLoRA) — metrics simulated for now
//! - Data parsing via alimentar (Arrow-based CSV/JSON schema detection)
//! - RAG via trueno-rag BM25 inverted index
//! - All data stored in ~/.banco/

fn main() {
    println!("Recipe 500-6: Banco Training Workflow");
    println!();
    println!("Upload → Recipe → Train → Eval → Compare");
    println!();
    println!("82 endpoints, 345 L1 + 38 L2 tests, fully sovereign");
    println!();
    println!("Note: training metrics are simulated (response includes simulated=true)");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_recipe_500_6_compiles() {
        super::main();
    }
}
