//! # RECIPE-300-3: Semantic Preserving Transformations
//!
//! **Level:** 300 (Advanced)
//! **Estimated Time:** 30 hours
//! **Prerequisites:** RECIPE-300-2 (AST Manipulation), RECIPE-200-3 (Custom Validation)
//!
//! ## Learning Objectives
//! - Understand semantic equivalence in code transformations
//! - Implement transformations that preserve program behavior
//! - Verify semantic preservation through testing
//! - Apply optimization transformations safely
//! - Use property-based testing for equivalence checking
//!
//! ## Concepts Covered
//! - Semantic equivalence vs syntactic equivalence
//! - Safe code refactoring transformations
//! - Constant folding and dead code elimination
//! - Loop transformations and optimizations
//! - Equivalence testing strategies
//!
//! ## Examples
//! This file demonstrates three approaches:
//! 1. Basic semantic transformations (constant folding, dead code)
//! 2. Advanced transformations (loop unrolling, inlining)
//! 3. Transformation verification and testing

use batuta_cookbook::Result;
use std::collections::{HashMap, HashSet};

/// Represents a simple expression for transformation
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Integer literal
    Int(i64),
    /// Variable reference
    Var(String),
    /// Binary operation
    BinOp {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// Function call
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

/// Statement types
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// Variable assignment
    Assign { name: String, value: Expr },
    /// If statement
    If {
        condition: Expr,
        then_block: Vec<Stmt>,
        else_block: Vec<Stmt>,
    },
    /// Loop statement
    Loop { count: i64, body: Vec<Stmt> },
    /// Expression statement
    Expr(Expr),
}

/// Transformation types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformationType {
    /// Fold constant expressions
    ConstantFolding,
    /// Remove dead code
    DeadCodeElimination,
    /// Unroll loops
    LoopUnrolling,
    /// Inline function calls
    FunctionInlining,
    /// Simplify expressions
    ExpressionSimplification,
}

/// Semantic preservation guarantee level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PreservationLevel {
    /// Guaranteed to preserve semantics
    Guaranteed,
    /// Likely preserves semantics (heuristic)
    Likely,
    /// May change semantics (unsafe)
    Unsafe,
}

/// Transformation result with preservation guarantee
#[derive(Debug, Clone)]
pub struct TransformationResult {
    pub original: Stmt,
    pub transformed: Stmt,
    pub transformation_type: TransformationType,
    pub preservation_level: PreservationLevel,
    pub changes_made: usize,
}

/// Semantic transformer
pub struct SemanticTransformer {
    /// Variables known to be constant
    constant_vars: HashMap<String, i64>,
    /// Dead variables (never read)
    dead_vars: HashSet<String>,
    /// Maximum loop unroll count
    max_unroll: i64,
}

impl SemanticTransformer {
    pub fn new() -> Self {
        Self {
            constant_vars: HashMap::new(),
            dead_vars: HashSet::new(),
            max_unroll: 8,
        }
    }

    pub fn with_max_unroll(mut self, max_unroll: i64) -> Self {
        self.max_unroll = max_unroll;
        self
    }

    /// Apply constant folding transformation
    pub fn constant_fold(&self, expr: Expr) -> Expr {
        match expr {
            Expr::BinOp { op, left, right } => {
                let left_folded = self.constant_fold(*left);
                let right_folded = self.constant_fold(*right);

                // Try to fold if both sides are constants
                if let (Expr::Int(l), Expr::Int(r)) = (&left_folded, &right_folded) {
                    let result = match op {
                        Op::Add => l + r,
                        Op::Sub => l - r,
                        Op::Mul => l * r,
                        Op::Div if *r != 0 => l / r,
                        Op::Div => return Expr::BinOp {
                            op,
                            left: Box::new(left_folded),
                            right: Box::new(right_folded),
                        },
                    };
                    Expr::Int(result)
                } else {
                    Expr::BinOp {
                        op,
                        left: Box::new(left_folded),
                        right: Box::new(right_folded),
                    }
                }
            }
            Expr::Var(name) => {
                // Replace with constant if known
                if let Some(value) = self.constant_vars.get(&name) {
                    Expr::Int(*value)
                } else {
                    Expr::Var(name)
                }
            }
            Expr::Call { name, args } => Expr::Call {
                name,
                args: args.into_iter().map(|a| self.constant_fold(a)).collect(),
            },
            Expr::Int(_) => expr,
        }
    }

    /// Transform statement with semantic preservation
    pub fn transform_stmt(
        &self,
        stmt: Stmt,
        trans_type: TransformationType,
    ) -> TransformationResult {
        let original = stmt.clone();
        let mut changes = 0;

        let transformed = match trans_type {
            TransformationType::ConstantFolding => self.apply_constant_folding(stmt, &mut changes),
            TransformationType::DeadCodeElimination => {
                self.apply_dead_code_elimination(stmt, &mut changes)
            }
            TransformationType::LoopUnrolling => self.apply_loop_unrolling(stmt, &mut changes),
            TransformationType::ExpressionSimplification => {
                self.apply_expression_simplification(stmt, &mut changes)
            }
            TransformationType::FunctionInlining => {
                // Placeholder for function inlining
                stmt
            }
        };

        TransformationResult {
            original,
            transformed,
            transformation_type: trans_type,
            preservation_level: self.get_preservation_level(trans_type),
            changes_made: changes,
        }
    }

    fn apply_constant_folding(&self, stmt: Stmt, changes: &mut usize) -> Stmt {
        match stmt {
            Stmt::Assign { name, value } => {
                let folded = self.constant_fold(value.clone());
                if folded != value {
                    *changes += 1;
                }
                Stmt::Assign {
                    name,
                    value: folded,
                }
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
            } => {
                let folded_cond = self.constant_fold(condition);
                Stmt::If {
                    condition: folded_cond,
                    then_block: then_block
                        .into_iter()
                        .map(|s| self.apply_constant_folding(s, changes))
                        .collect(),
                    else_block: else_block
                        .into_iter()
                        .map(|s| self.apply_constant_folding(s, changes))
                        .collect(),
                }
            }
            Stmt::Loop { count, body } => Stmt::Loop {
                count,
                body: body
                    .into_iter()
                    .map(|s| self.apply_constant_folding(s, changes))
                    .collect(),
            },
            Stmt::Expr(expr) => {
                let folded = self.constant_fold(expr.clone());
                if folded != expr {
                    *changes += 1;
                }
                Stmt::Expr(folded)
            }
        }
    }

    fn apply_dead_code_elimination(&self, stmt: Stmt, changes: &mut usize) -> Stmt {
        match stmt {
            Stmt::If {
                condition,
                then_block,
                else_block,
            } => {
                // Check if condition is constant
                if let Expr::Int(val) = condition {
                    *changes += 1;
                    if val != 0 {
                        // Condition is always true, keep only then branch
                        return if then_block.len() == 1 {
                            then_block.into_iter().next().unwrap()
                        } else {
                            Stmt::If {
                                condition: Expr::Int(1),
                                then_block,
                                else_block: vec![],
                            }
                        };
                    } else {
                        // Condition is always false, keep only else branch
                        return if else_block.len() == 1 {
                            else_block.into_iter().next().unwrap()
                        } else if else_block.is_empty() {
                            // No else block, statement does nothing
                            Stmt::Expr(Expr::Int(0))
                        } else {
                            Stmt::If {
                                condition: Expr::Int(0),
                                then_block: vec![],
                                else_block,
                            }
                        };
                    }
                }
                Stmt::If {
                    condition,
                    then_block: then_block
                        .into_iter()
                        .map(|s| self.apply_dead_code_elimination(s, changes))
                        .collect(),
                    else_block: else_block
                        .into_iter()
                        .map(|s| self.apply_dead_code_elimination(s, changes))
                        .collect(),
                }
            }
            other => other,
        }
    }

    fn apply_loop_unrolling(&self, stmt: Stmt, changes: &mut usize) -> Stmt {
        match stmt {
            Stmt::Loop { count, body } => {
                if count <= self.max_unroll && count > 0 {
                    *changes += 1;
                    // Unroll the loop
                    let mut unrolled = Vec::new();
                    for _ in 0..count {
                        unrolled.extend(body.clone());
                    }
                    // Return a compound statement (using if with always-true condition)
                    Stmt::If {
                        condition: Expr::Int(1),
                        then_block: unrolled,
                        else_block: vec![],
                    }
                } else {
                    Stmt::Loop { count, body }
                }
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
            } => Stmt::If {
                condition,
                then_block: then_block
                    .into_iter()
                    .map(|s| self.apply_loop_unrolling(s, changes))
                    .collect(),
                else_block: else_block
                    .into_iter()
                    .map(|s| self.apply_loop_unrolling(s, changes))
                    .collect(),
            },
            other => other,
        }
    }

    fn apply_expression_simplification(&self, stmt: Stmt, changes: &mut usize) -> Stmt {
        match stmt {
            Stmt::Assign { name, value } => {
                let simplified = self.simplify_expr(value.clone(), changes);
                Stmt::Assign {
                    name,
                    value: simplified,
                }
            }
            Stmt::Expr(expr) => {
                let simplified = self.simplify_expr(expr, changes);
                Stmt::Expr(simplified)
            }
            other => other,
        }
    }

    fn simplify_expr(&self, expr: Expr, changes: &mut usize) -> Expr {
        match expr {
            Expr::BinOp { op, left, right } => {
                let left_simp = self.simplify_expr(*left, changes);
                let right_simp = self.simplify_expr(*right, changes);

                // Simplifications: x + 0 = x, x * 1 = x, x * 0 = 0, etc.
                match (&left_simp, op, &right_simp) {
                    (_, Op::Add, Expr::Int(0)) => {
                        *changes += 1;
                        left_simp
                    }
                    (Expr::Int(0), Op::Add, _) => {
                        *changes += 1;
                        right_simp
                    }
                    (_, Op::Mul, Expr::Int(1)) => {
                        *changes += 1;
                        left_simp
                    }
                    (Expr::Int(1), Op::Mul, _) => {
                        *changes += 1;
                        right_simp
                    }
                    (_, Op::Mul, Expr::Int(0)) | (Expr::Int(0), Op::Mul, _) => {
                        *changes += 1;
                        Expr::Int(0)
                    }
                    _ => Expr::BinOp {
                        op,
                        left: Box::new(left_simp),
                        right: Box::new(right_simp),
                    },
                }
            }
            other => other,
        }
    }

    fn get_preservation_level(&self, trans_type: TransformationType) -> PreservationLevel {
        match trans_type {
            TransformationType::ConstantFolding
            | TransformationType::ExpressionSimplification => PreservationLevel::Guaranteed,
            TransformationType::DeadCodeElimination | TransformationType::LoopUnrolling => {
                PreservationLevel::Likely
            }
            TransformationType::FunctionInlining => PreservationLevel::Unsafe,
        }
    }

    /// Mark a variable as constant
    pub fn mark_constant(&mut self, name: String, value: i64) {
        self.constant_vars.insert(name, value);
    }

    /// Mark a variable as dead (unused)
    pub fn mark_dead(&mut self, name: String) {
        self.dead_vars.insert(name);
    }
}

impl Default for SemanticTransformer {
    fn default() -> Self {
        Self::new()
    }
}

/// Equivalence checker for verifying transformations
pub struct EquivalenceChecker {
    /// Test cases for verification
    test_cases: Vec<HashMap<String, i64>>,
}

impl EquivalenceChecker {
    pub fn new() -> Self {
        Self {
            test_cases: vec![],
        }
    }

    /// Add a test case (variable assignments)
    pub fn add_test_case(&mut self, vars: HashMap<String, i64>) {
        self.test_cases.push(vars);
    }

    /// Check if two expressions are equivalent for all test cases
    pub fn expressions_equivalent(&self, expr1: &Expr, expr2: &Expr) -> bool {
        if self.test_cases.is_empty() {
            // Without test cases, check structural equality
            return expr1 == expr2;
        }

        for test_case in &self.test_cases {
            let eval1 = self.eval_expr(expr1, test_case);
            let eval2 = self.eval_expr(expr2, test_case);

            if eval1 != eval2 {
                return false;
            }
        }

        true
    }

    fn eval_expr(&self, expr: &Expr, vars: &HashMap<String, i64>) -> Option<i64> {
        match expr {
            Expr::Int(n) => Some(*n),
            Expr::Var(name) => vars.get(name).copied(),
            Expr::BinOp { op, left, right } => {
                let l = self.eval_expr(left, vars)?;
                let r = self.eval_expr(right, vars)?;
                Some(match op {
                    Op::Add => l + r,
                    Op::Sub => l - r,
                    Op::Mul => l * r,
                    Op::Div if r != 0 => l / r,
                    Op::Div => return None,
                })
            }
            Expr::Call { .. } => None, // Can't evaluate function calls
        }
    }
}

impl Default for EquivalenceChecker {
    fn default() -> Self {
        Self::new()
    }
}

//
// Example 1: Constant folding transformation
//
pub fn example_1_constant_folding() -> Result<()> {
    println!("=== Example 1: Constant Folding ===\n");

    let transformer = SemanticTransformer::new();

    // Expression with constants
    let expr = Expr::BinOp {
        op: Op::Add,
        left: Box::new(Expr::BinOp {
            op: Op::Mul,
            left: Box::new(Expr::Int(2)),
            right: Box::new(Expr::Int(3)),
        }),
        right: Box::new(Expr::Int(4)),
    };

    let stmt = Stmt::Assign {
        name: "result".to_string(),
        value: expr,
    };

    let result = transformer.transform_stmt(stmt, TransformationType::ConstantFolding);

    println!("Transformation: {:?}", result.transformation_type);
    println!("Preservation: {:?}", result.preservation_level);
    println!("Changes made: {}", result.changes_made);
    println!("Transformed: {:?}", result.transformed);

    Ok(())
}

//
// Example 2: Dead code elimination
//
pub fn example_2_dead_code_elimination() -> Result<()> {
    println!("\n=== Example 2: Dead Code Elimination ===\n");

    let transformer = SemanticTransformer::new();

    // If statement with constant condition
    let stmt = Stmt::If {
        condition: Expr::Int(1), // Always true
        then_block: vec![Stmt::Assign {
            name: "x".to_string(),
            value: Expr::Int(42),
        }],
        else_block: vec![Stmt::Assign {
            name: "x".to_string(),
            value: Expr::Int(0),
        }],
    };

    let result = transformer.transform_stmt(stmt, TransformationType::DeadCodeElimination);

    println!("Preservation: {:?}", result.preservation_level);
    println!("Changes made: {}", result.changes_made);
    println!("Original had both branches");
    println!("Transformed: {:?}", result.transformed);
    println!("(Else branch eliminated because condition is always true)");

    Ok(())
}

//
// Example 3: Loop unrolling and verification
//
pub fn example_3_loop_unrolling() -> Result<()> {
    println!("\n=== Example 3: Loop Unrolling ===\n");

    let transformer = SemanticTransformer::new().with_max_unroll(5);

    // Small loop that can be unrolled
    let stmt = Stmt::Loop {
        count: 3,
        body: vec![Stmt::Assign {
            name: "sum".to_string(),
            value: Expr::BinOp {
                op: Op::Add,
                left: Box::new(Expr::Var("sum".to_string())),
                right: Box::new(Expr::Int(1)),
            },
        }],
    };

    let result = transformer.transform_stmt(stmt.clone(), TransformationType::LoopUnrolling);

    println!("Original loop count: 3");
    println!("Transformation: {:?}", result.transformation_type);
    println!("Changes made: {}", result.changes_made);
    println!(
        "Unrolled: Loop body repeated {} times",
        if result.changes_made > 0 { "3" } else { "still in loop" }
    );

    // Try with a large loop
    let large_loop = Stmt::Loop {
        count: 100,
        body: vec![Stmt::Expr(Expr::Int(1))],
    };

    let result2 = transformer.transform_stmt(large_loop, TransformationType::LoopUnrolling);
    println!("\nLarge loop (100 iterations):");
    println!("Changes made: {} (not unrolled, exceeds max)", result2.changes_made);

    Ok(())
}

fn main() -> Result<()> {
    example_1_constant_folding()?;
    example_2_dead_code_elimination()?;
    example_3_loop_unrolling()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_fold_simple() {
        let transformer = SemanticTransformer::new();
        let expr = Expr::BinOp {
            op: Op::Add,
            left: Box::new(Expr::Int(2)),
            right: Box::new(Expr::Int(3)),
        };

        let result = transformer.constant_fold(expr);
        assert_eq!(result, Expr::Int(5));
    }

    #[test]
    fn test_constant_fold_nested() {
        let transformer = SemanticTransformer::new();
        let expr = Expr::BinOp {
            op: Op::Mul,
            left: Box::new(Expr::BinOp {
                op: Op::Add,
                left: Box::new(Expr::Int(2)),
                right: Box::new(Expr::Int(3)),
            }),
            right: Box::new(Expr::Int(4)),
        };

        let result = transformer.constant_fold(expr);
        assert_eq!(result, Expr::Int(20));
    }

    #[test]
    fn test_constant_fold_with_variable() {
        let transformer = SemanticTransformer::new();
        let expr = Expr::BinOp {
            op: Op::Add,
            left: Box::new(Expr::Var("x".to_string())),
            right: Box::new(Expr::Int(5)),
        };

        let result = transformer.constant_fold(expr);
        // Should not fold because x is not constant
        assert!(matches!(result, Expr::BinOp { .. }));
    }

    #[test]
    fn test_constant_fold_known_variable() {
        let mut transformer = SemanticTransformer::new();
        transformer.mark_constant("x".to_string(), 10);

        let expr = Expr::BinOp {
            op: Op::Add,
            left: Box::new(Expr::Var("x".to_string())),
            right: Box::new(Expr::Int(5)),
        };

        let result = transformer.constant_fold(expr);
        assert_eq!(result, Expr::Int(15));
    }

    #[test]
    fn test_dead_code_elimination_true() {
        let transformer = SemanticTransformer::new();
        let stmt = Stmt::If {
            condition: Expr::Int(1),
            then_block: vec![Stmt::Expr(Expr::Int(42))],
            else_block: vec![Stmt::Expr(Expr::Int(0))],
        };

        let result = transformer.transform_stmt(stmt, TransformationType::DeadCodeElimination);
        assert!(result.changes_made > 0);
    }

    #[test]
    fn test_dead_code_elimination_false() {
        let transformer = SemanticTransformer::new();
        let stmt = Stmt::If {
            condition: Expr::Int(0),
            then_block: vec![Stmt::Expr(Expr::Int(42))],
            else_block: vec![Stmt::Expr(Expr::Int(99))],
        };

        let result = transformer.transform_stmt(stmt, TransformationType::DeadCodeElimination);
        assert!(result.changes_made > 0);
        // Should keep else branch
        assert_eq!(result.transformed, Stmt::Expr(Expr::Int(99)));
    }

    #[test]
    fn test_loop_unrolling_small() {
        let transformer = SemanticTransformer::new();
        let stmt = Stmt::Loop {
            count: 3,
            body: vec![Stmt::Expr(Expr::Int(1))],
        };

        let result = transformer.transform_stmt(stmt, TransformationType::LoopUnrolling);
        assert_eq!(result.changes_made, 1);
    }

    #[test]
    fn test_loop_unrolling_large() {
        let transformer = SemanticTransformer::new();
        let stmt = Stmt::Loop {
            count: 100,
            body: vec![Stmt::Expr(Expr::Int(1))],
        };

        let result = transformer.transform_stmt(stmt, TransformationType::LoopUnrolling);
        assert_eq!(result.changes_made, 0); // Not unrolled
    }

    #[test]
    fn test_expression_simplification() {
        let transformer = SemanticTransformer::new();
        let expr = Expr::BinOp {
            op: Op::Add,
            left: Box::new(Expr::Var("x".to_string())),
            right: Box::new(Expr::Int(0)),
        };

        let mut changes = 0;
        let result = transformer.simplify_expr(expr, &mut changes);
        assert_eq!(result, Expr::Var("x".to_string()));
        assert_eq!(changes, 1);
    }

    #[test]
    fn test_simplify_multiply_by_zero() {
        let transformer = SemanticTransformer::new();
        let expr = Expr::BinOp {
            op: Op::Mul,
            left: Box::new(Expr::Var("x".to_string())),
            right: Box::new(Expr::Int(0)),
        };

        let mut changes = 0;
        let result = transformer.simplify_expr(expr, &mut changes);
        assert_eq!(result, Expr::Int(0));
        assert_eq!(changes, 1);
    }

    #[test]
    fn test_simplify_multiply_by_one() {
        let transformer = SemanticTransformer::new();
        let expr = Expr::BinOp {
            op: Op::Mul,
            left: Box::new(Expr::Var("y".to_string())),
            right: Box::new(Expr::Int(1)),
        };

        let mut changes = 0;
        let result = transformer.simplify_expr(expr, &mut changes);
        assert_eq!(result, Expr::Var("y".to_string()));
        assert_eq!(changes, 1);
    }

    #[test]
    fn test_preservation_levels() {
        let transformer = SemanticTransformer::new();

        assert_eq!(
            transformer.get_preservation_level(TransformationType::ConstantFolding),
            PreservationLevel::Guaranteed
        );
        assert_eq!(
            transformer.get_preservation_level(TransformationType::LoopUnrolling),
            PreservationLevel::Likely
        );
        assert_eq!(
            transformer.get_preservation_level(TransformationType::FunctionInlining),
            PreservationLevel::Unsafe
        );
    }

    #[test]
    fn test_equivalence_checker() {
        let mut checker = EquivalenceChecker::new();
        let mut test_case = HashMap::new();
        test_case.insert("x".to_string(), 5);
        checker.add_test_case(test_case);

        let expr1 = Expr::BinOp {
            op: Op::Add,
            left: Box::new(Expr::Var("x".to_string())),
            right: Box::new(Expr::Int(3)),
        };

        let expr2 = Expr::Int(8);

        assert!(checker.expressions_equivalent(&expr1, &expr2));
    }

    #[test]
    fn test_equivalence_checker_not_equivalent() {
        let mut checker = EquivalenceChecker::new();
        let mut test_case = HashMap::new();
        test_case.insert("x".to_string(), 5);
        checker.add_test_case(test_case);

        let expr1 = Expr::Var("x".to_string());
        let expr2 = Expr::Int(10);

        assert!(!checker.expressions_equivalent(&expr1, &expr2));
    }

    #[test]
    fn test_transformation_result_structure() {
        let transformer = SemanticTransformer::new();
        let stmt = Stmt::Expr(Expr::Int(42));

        let result = transformer.transform_stmt(stmt, TransformationType::ConstantFolding);

        assert_eq!(result.transformation_type, TransformationType::ConstantFolding);
        assert_eq!(result.preservation_level, PreservationLevel::Guaranteed);
    }
}
