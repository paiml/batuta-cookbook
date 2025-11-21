# Contributing to Batuta Cookbook

Thank you for your interest in contributing to the Batuta Cookbook!

## Core Principle: EXTREME TDD

**Every recipe MUST be runnable code.** No pseudo-code, no hypothetical examples.

```bash
# Your recipe must pass these checks:
cargo run --example recipe_name   # Must run without errors
cargo test recipe_name             # Must have tests
cargo clippy --example recipe_name # Must pass linting
```

## Quick Start

1. **Fork the repository**
2. **Create a new recipe:**
   ```bash
   make new-recipe LEVEL=100 NUM=6 NAME=my_recipe
   ```
3. **Implement the recipe** following the template
4. **Run quality checks:**
   ```bash
   make quick  # Fast checks
   ```
5. **Submit a pull request**

## Recipe Checklist

Before submitting a recipe PR:

- [ ] Recipe compiles: `cargo build --example recipe_name`
- [ ] Recipe runs: `cargo run --example recipe_name`
- [ ] Tests pass: `cargo test recipe_name`
- [ ] Coverage >90%: `cargo llvm-cov --example recipe_name`
- [ ] Clippy clean: `cargo clippy --example recipe_name`
- [ ] Formatted: `cargo fmt --check`
- [ ] Documentation complete:
  - [ ] Recipe metadata (level, category, difficulty)
  - [ ] Description (what it demonstrates)
  - [ ] Prerequisites
  - [ ] Learning objectives
  - [ ] Related papers (if applicable)
- [ ] At least 3 examples demonstrating different aspects
- [ ] Tests covering all examples
- [ ] Property-based tests (if applicable)
- [ ] Benchmarks (for optimization recipes)

## Recipe Structure

Every recipe must follow this template:

```rust
//! Recipe XXX-Y: Title
//!
//! **Level:** [100|200|300|400]
//! **Category:** [Analysis|Transpilation|Optimization|Validation|End-to-End]
//! **Difficulty:** [Basic|Intermediate|Advanced|Expert]
//!
//! ## Description
//!
//! [2-3 sentence description]
//!
//! ## Prerequisites
//!
//! - [List prerequisites]
//!
//! ## Learning Objectives
//!
//! - [What users will learn]
//!
//! ## Run This Recipe
//!
//! ```bash
//! cargo run --example recipe_XXX_Y_name
//! ```

#![allow(clippy::print_stdout)]
#![allow(clippy::unwrap_used)]

fn main() {
    println!("=== Recipe XXX-Y: Title ===\n");
    example_1();
    example_2();
    example_3();
}

// ... Examples ...

#[cfg(test)]
mod tests {
    // ... Tests for each example ...
}
```

## Code Quality Standards

- **Coverage:** >90% for Level 100-200, >85% for Level 300-400
- **Mutation Score:** >80% kill rate
- **Complexity:** Max cyclomatic complexity of 15
- **No TODOs:** Remove all TODOs before submitting
- **No unsafe:** Examples should not contain unsafe code

## Testing Requirements

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_runs() {
        example_1();  // Should not panic
    }

    #[test]
    fn test_functionality() {
        let result = some_function();
        assert!(result.is_ok());
    }
}
```

### Property-Based Tests

```rust
#[cfg(all(test, feature = "proptest"))]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_property(input in any::<u32>()) {
            // Test invariant holds for all inputs
        }
    }
}
```

### Integration Tests

For complex recipes, add integration tests in `tests/`:

```rust
// tests/test_recipe_xxx_y.rs
use batuta_cookbook::*;

#[test]
fn test_full_workflow() {
    // Test complete workflow
}
```

## Benchmarks (Level 300-400)

Recipes claiming performance improvements MUST include benchmarks:

```rust
#[cfg(all(test, feature = "bench"))]
mod benches {
    use super::*;
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    fn bench_operation(c: &mut Criterion) {
        c.bench_function("recipe_xxx_y", |b| {
            b.iter(|| {
                operation(black_box(input))
            });
        });
    }

    criterion_group!(benches, bench_operation);
    criterion_main!(benches);
}
```

## Documentation

- Use clear, concise language
- Include code examples in doc comments
- Link to related papers when applicable
- Explain *why*, not just *what*

## Git Workflow

1. Create a feature branch: `git checkout -b recipe-xxx-y-my-recipe`
2. Make your changes
3. Run quality checks: `make ci`
4. Commit with descriptive message:
   ```
   feat: Add Recipe XXX-Y: My Recipe Title

   - Implements [feature]
   - Includes [number] tests
   - Coverage: [percentage]%
   ```
5. Push and create PR

## Pre-Commit Hooks

Install the pre-commit hook for automatic checks:

```bash
make install-hooks
```

This will run on every commit:
- Format check
- Clippy
- Unit tests
- Examples compilation

## Questions?

- Open an [issue](https://github.com/paiml/batuta-cookbook/issues)
- Join [Discord](https://discord.gg/pragmatic-ai-labs)
- Read the [specification](docs/specifications/cookbook-recipes-book.md)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
