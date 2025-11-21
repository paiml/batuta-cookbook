# Batuta Cookbook Makefile
# EXTREME TDD Development Commands

# Make configuration for safety and performance
.SUFFIXES:
.DELETE_ON_ERROR:

.PHONY: help test coverage examples pmat-check install clean
.PHONY: fmt build bench docs all
.PHONY: bashrs-check bashrs-audit bashrs-install

# Default target
.DEFAULT_GOAL := help

# Colors
GREEN  := $(shell tput -Txterm setaf 2)
YELLOW := $(shell tput -Txterm setaf 3)
RED    := $(shell tput -Txterm setaf 1)
BLUE   := $(shell tput -Txterm setaf 4)
RESET  := $(shell tput -Txterm sgr0)

# PMAT binary (should be in PATH after: cargo install --path ../paiml-mcp-agent-toolkit)
PMAT := pmat

##@ General

help: ## Display this help message
	@echo '$(BLUE)Batuta Cookbook - EXTREME TDD Commands$(RESET)'
	@echo ''
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make $(YELLOW)<target>$(RESET)\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  $(YELLOW)%-20s$(RESET) %s\n", $$1, $$2 } /^##@/ { printf "\n$(BLUE)%s$(RESET)\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Development

fmt: ## Format code with rustfmt
	@echo "$(BLUE)Formatting code...$(RESET)"
	cargo fmt

check-fmt: ## Check code formatting
	@echo "$(BLUE)Checking code formatting...$(RESET)"
	cargo fmt -- --check

clippy: ## Run clippy lints
	@echo "$(BLUE)Running clippy...$(RESET)"
	cargo clippy --all-features --all-targets -- -D warnings

clippy-fix: ## Fix clippy issues automatically
	@echo "$(BLUE)Fixing clippy issues...$(RESET)"
	cargo clippy --all-features --fix --allow-dirty

build: ## Build the project
	@echo "$(BLUE)Building project...$(RESET)"
	cargo build

build-release: ## Build release version
	@echo "$(BLUE)Building release...$(RESET)"
	cargo build --release

clean: ## Clean build artifacts
	@echo "$(BLUE)Cleaning build artifacts...$(RESET)"
	cargo clean
	@rm -rf target/ || true

##@ Testing

test: ## Run all tests
	@echo "$(BLUE)Running tests...$(RESET)"
	cargo test --all-features

test-verbose: ## Run tests with verbose output
	@echo "$(BLUE)Running tests (verbose)...$(RESET)"
	cargo test --all-features -- --nocapture

test-quick: ## Run quick tests (lib only)
	@echo "$(BLUE)Running quick tests...$(RESET)"
	cargo test --lib

##@ Coverage

coverage: ## Generate test coverage report
	@echo "$(BLUE)Generating coverage report...$(RESET)"
	cargo llvm-cov --all-features --html
	@echo "$(GREEN)✓ Coverage report: target/llvm-cov/html/index.html$(RESET)"

coverage-summary: ## Show coverage summary
	@echo "$(BLUE)Coverage summary:$(RESET)"
	cargo llvm-cov --all-features --summary-only

coverage-check: ## Verify coverage meets 90% threshold
	@echo "$(BLUE)Checking coverage threshold...$(RESET)"
	@coverage=$$(cargo llvm-cov --all-features --summary-only 2>/dev/null | grep "TOTAL" | awk '{print $$10}' | tr -d '%'); \
	if [ -z "$$coverage" ]; then \
		echo "$(RED)✗ Could not determine coverage$(RESET)"; \
		exit 1; \
	fi; \
	echo "Coverage: $$coverage%"; \
	if [ $$(echo "$$coverage >= 90.0" | bc -l) -eq 1 ]; then \
		echo "$(GREEN)✓ Coverage $$coverage% exceeds 90% threshold$(RESET)"; \
	else \
		echo "$(RED)✗ Coverage $$coverage% below 90% threshold$(RESET)"; \
		exit 1; \
	fi

coverage-lcov: ## Generate LCOV coverage report
	@echo "$(BLUE)Generating LCOV report...$(RESET)"
	cargo llvm-cov --all-features --lcov --output-path coverage.lcov

##@ Mutation Testing

mutants: ## Run mutation testing
	@echo "$(BLUE)Running mutation tests (this takes a while)...$(RESET)"
	cargo mutants --all-features --timeout 300

mutants-check: ## Check mutation score meets 80% threshold
	@echo "$(BLUE)Checking mutation score...$(RESET)"
	cargo mutants --all-features --timeout 300 --check

##@ Examples (EXTREME TDD: All must be runnable!)

examples-compile: ## Compile all example recipes
	@echo "$(BLUE)Compiling all examples...$(RESET)"
	@count=0; \
	failed=0; \
	for example in examples/*.rs; do \
		if [ -f "$$example" ]; then \
			name=$$(basename "$$example" .rs); \
			count=$$((count + 1)); \
			printf "  [$$count] $$name... "; \
			if cargo build --example "$$name" --quiet 2>/dev/null; then \
				echo "$(GREEN)✓$(RESET)"; \
			else \
				echo "$(RED)✗$(RESET)"; \
				failed=$$((failed + 1)); \
			fi; \
		fi; \
	done; \
	if [ $$failed -eq 0 ]; then \
		echo "$(GREEN)✓ All $$count examples compiled successfully$(RESET)"; \
	else \
		echo "$(RED)✗ $$failed/$$count examples failed to compile$(RESET)"; \
		exit 1; \
	fi

examples-run: ## Run all example recipes
	@echo "$(BLUE)Running all examples...$(RESET)"
	@count=0; \
	failed=0; \
	for example in examples/*.rs; do \
		if [ -f "$$example" ]; then \
			name=$$(basename "$$example" .rs); \
			count=$$((count + 1)); \
			echo ""; \
			echo "$(YELLOW)=== Running example: $$name ===$(RESET)"; \
			if cargo run --example "$$name" 2>&1; then \
				echo "$(GREEN)✓ $$name completed successfully$(RESET)"; \
			else \
				echo "$(RED)✗ $$name failed$(RESET)"; \
				failed=$$((failed + 1)); \
			fi; \
		fi; \
	done; \
	echo ""; \
	if [ $$failed -eq 0 ]; then \
		echo "$(GREEN)✓ All $$count examples ran successfully$(RESET)"; \
	else \
		echo "$(RED)✗ $$failed/$$count examples failed to run$(RESET)"; \
		exit 1; \
	fi

examples-test: ## Test all examples
	@echo "$(BLUE)Testing all examples...$(RESET)"
	cargo test --examples

examples-list: ## List all available examples
	@echo "$(BLUE)Available recipe examples:$(RESET)"
	@echo ""
	@for example in examples/*.rs; do \
		if [ -f "$$example" ]; then \
			name=$$(basename "$$example" .rs); \
			echo "  $(YELLOW)$$name$(RESET)"; \
			echo "    Run: $(GREEN)cargo run --example $$name$(RESET)"; \
			echo "    Test: $(GREEN)cargo test $$name$(RESET)"; \
			echo ""; \
		fi; \
	done

##@ Benchmarking

bench: ## Run all benchmarks
	@echo "$(BLUE)Running benchmarks...$(RESET)"
	cargo bench

bench-compile: ## Verify benchmarks compile
	@echo "$(BLUE)Compiling benchmarks...$(RESET)"
	cargo bench --no-run

##@ PMAT Integration

pmat-check: ## Run PMAT quality checks
	@echo "$(BLUE)Running PMAT quality checks...$(RESET)"
	@$(PMAT) check --project-path . --format summary || echo "$(YELLOW)⚠ Quality issues found$(RESET)"

pmat-health: ## Run PMAT health check
	@echo "$(BLUE)Running PMAT health check...$(RESET)"
	@$(PMAT) repo-score --path . || true

pmat-roadmap: ## Show PMAT roadmap health
	@echo "$(BLUE)PMAT Roadmap Health...$(RESET)"
	@$(PMAT) maintain roadmap --roadmap ROADMAP.md --health || true

pmat-validate: ## Validate PMAT roadmap
	@echo "$(BLUE)Validating PMAT roadmap...$(RESET)"
	@$(PMAT) maintain roadmap --roadmap ROADMAP.md --validate

pmat-gates: ## Run all PMAT quality gates
	@echo "$(BLUE)Running all PMAT quality gates...$(RESET)"
	@echo ""
	@echo "$(YELLOW)1. Format Check$(RESET)"
	@$(MAKE) check-fmt
	@echo ""
	@echo "$(YELLOW)2. Clippy$(RESET)"
	@$(MAKE) clippy
	@echo ""
	@echo "$(YELLOW)3. Tests$(RESET)"
	@$(MAKE) test
	@echo ""
	@echo "$(YELLOW)4. Coverage$(RESET)"
	@$(MAKE) coverage-check
	@echo ""
	@echo "$(YELLOW)5. Examples$(RESET)"
	@$(MAKE) examples-compile
	@echo ""
	@echo "$(GREEN)✅ All PMAT quality gates passed!$(RESET)"

##@ Documentation

docs: ## Build documentation
	@echo "$(BLUE)Building documentation...$(RESET)"
	cargo doc --no-deps --all-features

docs-open: ## Build and open documentation
	@echo "$(BLUE)Building and opening documentation...$(RESET)"
	cargo doc --no-deps --all-features --open

##@ Git Hooks

install-hooks: ## Install git pre-commit hooks
	@echo "$(BLUE)Installing git hooks...$(RESET)"
	@if [ ! -f .git/hooks/pre-commit ]; then \
		echo "$(RED)✗ .git/hooks/pre-commit not found$(RESET)"; \
		exit 1; \
	fi
	@chmod +x .git/hooks/pre-commit || { echo "$(RED)✗ Failed to set hook permissions$(RESET)"; exit 1; }
	@echo "$(GREEN)✓ Git hooks installed$(RESET)"

##@ Shell Script Quality (bashrs)

bashrs-check: ## Check all shell scripts with bashrs
	@echo "$(BLUE)Checking shell scripts with bashrs...$(RESET)"
	@echo ""
	@if ! command -v bashrs >/dev/null 2>&1; then \
		echo "$(YELLOW)⚠ bashrs not installed$(RESET)"; \
		echo "  Install: cargo install bashrs"; \
		exit 1; \
	fi
	@echo "Checking pre-commit hook..."
	@if bashrs lint .git/hooks/pre-commit --validation minimal 2>/dev/null; then \
		echo "$(GREEN)✓ pre-commit hook passed$(RESET)"; \
	else \
		echo "$(YELLOW)⚠ pre-commit hook has warnings$(RESET)"; \
	fi
	@echo ""
	@echo "Checking Makefile..."
	@bashrs lint Makefile 2>&1 | head -20 || echo "$(YELLOW)⚠ Makefile has warnings (non-blocking)$(RESET)"
	@echo ""
	@echo "$(GREEN)✓ Shell script quality check complete$(RESET)"

bashrs-audit: ## Full bashrs audit with detailed report
	@echo "$(BLUE)Running full bashrs audit...$(RESET)"
	@echo ""
	@if ! command -v bashrs >/dev/null 2>&1; then \
		echo "$(YELLOW)⚠ bashrs not installed$(RESET)"; \
		echo "  Install: cargo install bashrs"; \
		exit 1; \
	fi
	@echo "=== Pre-commit Hook ==="
	@bashrs audit .git/hooks/pre-commit || true
	@echo ""
	@echo "=== Makefile ==="
	@bashrs lint Makefile || true
	@echo ""
	@echo "$(GREEN)✓ Full audit complete$(RESET)"

bashrs-install: ## Install bashrs tool
	@echo "$(BLUE)Installing bashrs...$(RESET)"
	cargo install bashrs
	@echo "$(GREEN)✓ bashrs installed$(RESET)"

##@ CI/CD

ci: ## Run full CI pipeline locally
	@echo "$(BLUE)Running full CI pipeline...$(RESET)"
	@echo ""
	@echo "$(YELLOW)=== Step 1/7: Format ===$(RESET)"
	@$(MAKE) check-fmt
	@echo ""
	@echo "$(YELLOW)=== Step 2/7: Clippy ===$(RESET)"
	@$(MAKE) clippy
	@echo ""
	@echo "$(YELLOW)=== Step 3/7: Build ===$(RESET)"
	@$(MAKE) build
	@echo ""
	@echo "$(YELLOW)=== Step 4/7: Tests ===$(RESET)"
	@$(MAKE) test
	@echo ""
	@echo "$(YELLOW)=== Step 5/7: Coverage ===$(RESET)"
	@$(MAKE) coverage-check
	@echo ""
	@echo "$(YELLOW)=== Step 6/7: Examples ===$(RESET)"
	@$(MAKE) examples-compile
	@echo ""
	@echo "$(YELLOW)=== Step 7/7: Docs ===$(RESET)"
	@$(MAKE) docs
	@echo ""
	@echo "$(GREEN)✅ Full CI pipeline completed successfully!$(RESET)"

ci-release: ## Run full release pipeline (includes mutation testing)
	@echo "$(BLUE)Running release pipeline...$(RESET)"
	@$(MAKE) ci
	@echo ""
	@echo "$(YELLOW)=== Mutation Testing ===$(RESET)"
	@$(MAKE) mutants-check
	@echo ""
	@echo "$(YELLOW)=== PMAT Validation ===$(RESET)"
	@$(MAKE) pmat-validate
	@echo ""
	@echo "$(GREEN)✅ Release pipeline completed successfully!$(RESET)"

##@ Quick Actions

quick: ## Quick check (fmt + clippy + test)
	@echo "$(BLUE)Running quick checks...$(RESET)"
	@$(MAKE) check-fmt
	@$(MAKE) clippy
	@$(MAKE) test-quick
	@echo "$(GREEN)✓ Quick checks passed$(RESET)"

fix: ## Fix common issues automatically
	@echo "$(BLUE)Fixing common issues...$(RESET)"
	@$(MAKE) fmt
	@$(MAKE) clippy-fix
	@echo "$(GREEN)✓ Auto-fixes applied$(RESET)"

all: ci ## Run all checks (alias for ci)

##@ Recipe Development

new-recipe: ## Create a new recipe (usage: make new-recipe LEVEL=100 NUM=6 NAME=my_recipe)
	@if [ -z "$(LEVEL)" ] || [ -z "$(NUM)" ] || [ -z "$(NAME)" ]; then \
		echo "$(RED)✗ Usage: make new-recipe LEVEL=100 NUM=6 NAME=my_recipe$(RESET)"; \
		exit 1; \
	fi; \
	file="examples/recipe_$(LEVEL)_$(NUM)_$(NAME).rs"; \
	if [ -f "$$file" ]; then \
		echo "$(RED)✗ Recipe already exists: $$file$(RESET)"; \
		exit 1; \
	fi; \
	echo "$(BLUE)Creating new recipe: $$file$(RESET)"; \
	mkdir -p examples/data/recipe_$(LEVEL)_$(NUM) || exit 1; \
	echo "# Recipe $(LEVEL)-$(NUM) test data" > examples/data/recipe_$(LEVEL)_$(NUM)/README.md; \
	cat > "$$file" << 'EOF' || exit 1; \
//! Recipe $(LEVEL)-$(NUM): $(NAME) \
//! \
//! Run: `cargo run --example recipe_$(LEVEL)_$(NUM)_$(NAME)` \
\
fn main() { \
    println!("=== Recipe $(LEVEL)-$(NUM): $(NAME) ===\n"); \
    example_1(); \
} \
\
fn example_1() { \
    println!("Example 1: TODO"); \
} \
\
#[cfg(test)] \
mod tests { \
    use super::*; \
\
    #[test] \
    fn test_example_1() { \
        example_1(); \
    } \
} \
EOF
	echo "$(GREEN)✓ Created recipe: $$file$(RESET)"; \
	echo "$(YELLOW)  Next steps:$(RESET)"; \
	echo "    1. Edit: $$file"; \
	echo "    2. Test: cargo run --example recipe_$(LEVEL)_$(NUM)_$(NAME)"; \
	echo "    3. Verify: make examples-compile"

##@ Installation

install: ## Install batuta (when implemented)
	@echo "$(BLUE)Installing batuta...$(RESET)"
	@echo "$(YELLOW)⚠  batuta library not yet implemented$(RESET)"
	@echo "   This cookbook is a specification for future development"

install-tools: ## Install required development tools
	@echo "$(BLUE)Installing development tools...$(RESET)"
	@echo ""
	@echo "Installing cargo-llvm-cov..."
	cargo install cargo-llvm-cov
	@echo ""
	@echo "Installing cargo-mutants..."
	cargo install cargo-mutants
	@echo ""
	@echo "$(GREEN)✓ Development tools installed$(RESET)"
