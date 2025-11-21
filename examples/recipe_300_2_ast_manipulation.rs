//! # RECIPE-300-2: AST Manipulation
//!
//! **Level:** 300 (Advanced)
//! **Estimated Time:** 28 hours
//! **Prerequisites:** RECIPE-100-5 (Simple Transpilation), RECIPE-200-3 (Custom Validation)
//!
//! ## Learning Objectives
//! - Parse source code into Abstract Syntax Trees (AST)
//! - Navigate and query AST nodes
//! - Transform AST structures programmatically
//! - Generate code from modified ASTs
//! - Preserve semantic meaning during transformations
//!
//! ## Concepts Covered
//! - AST node representation and traversal
//! - Visitor pattern for AST processing
//! - Tree transformation algorithms
//! - Code generation from AST
//! - Pattern matching on AST structures
//!
//! ## Examples
//! This file demonstrates three approaches:
//! 1. Basic AST parsing and traversal
//! 2. AST transformations (refactoring, optimization)
//! 3. Code generation from modified AST

use batuta_cookbook::Result;
use std::collections::HashMap;
use std::fmt;

/// AST node types
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    /// Program root
    Program(Vec<AstNode>),
    /// Function definition
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<AstNode>,
    },
    /// Variable declaration
    VarDecl {
        name: String,
        value: Box<AstNode>,
    },
    /// Assignment expression
    Assignment {
        target: String,
        value: Box<AstNode>,
    },
    /// Binary operation
    BinaryOp {
        op: BinaryOperator,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    /// Function call
    Call {
        function: String,
        args: Vec<AstNode>,
    },
    /// If statement
    If {
        condition: Box<AstNode>,
        then_branch: Vec<AstNode>,
        else_branch: Option<Vec<AstNode>>,
    },
    /// Return statement
    Return(Box<AstNode>),
    /// Identifier reference
    Identifier(String),
    /// Literal values
    Literal(LiteralValue),
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Less,
    Greater,
    And,
    Or,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Equal => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
            Self::Less => write!(f, "<"),
            Self::Greater => write!(f, ">"),
            Self::And => write!(f, "&&"),
            Self::Or => write!(f, "||"),
        }
    }
}

/// Literal value types
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

/// AST visitor trait for traversing nodes
pub trait AstVisitor {
    fn visit_node(&mut self, node: &AstNode) -> Result<()> {
        match node {
            AstNode::Program(nodes) => {
                for n in nodes {
                    self.visit_node(n)?;
                }
            }
            AstNode::Function { body, .. } => {
                for n in body {
                    self.visit_node(n)?;
                }
            }
            AstNode::VarDecl { value, .. } => {
                self.visit_node(value)?;
            }
            AstNode::Assignment { value, .. } => {
                self.visit_node(value)?;
            }
            AstNode::BinaryOp { left, right, .. } => {
                self.visit_node(left)?;
                self.visit_node(right)?;
            }
            AstNode::Call { args, .. } => {
                for arg in args {
                    self.visit_node(arg)?;
                }
            }
            AstNode::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_node(condition)?;
                for n in then_branch {
                    self.visit_node(n)?;
                }
                if let Some(else_nodes) = else_branch {
                    for n in else_nodes {
                        self.visit_node(n)?;
                    }
                }
            }
            AstNode::Return(expr) => {
                self.visit_node(expr)?;
            }
            AstNode::Identifier(_) | AstNode::Literal(_) => {}
        }
        Ok(())
    }
}

/// AST analyzer for collecting statistics
pub struct AstAnalyzer {
    pub function_count: usize,
    pub var_count: usize,
    pub call_count: usize,
    pub max_depth: usize,
}

impl AstAnalyzer {
    pub fn new() -> Self {
        Self {
            function_count: 0,
            var_count: 0,
            call_count: 0,
            max_depth: 0,
        }
    }

    pub fn analyze(&mut self, ast: &AstNode) -> Result<()> {
        self.visit_with_depth(ast, 0)
    }

    fn visit_with_depth(&mut self, node: &AstNode, depth: usize) -> Result<()> {
        self.max_depth = self.max_depth.max(depth);

        match node {
            AstNode::Program(nodes) => {
                for n in nodes {
                    self.visit_with_depth(n, depth + 1)?;
                }
            }
            AstNode::Function { body, .. } => {
                self.function_count += 1;
                for n in body {
                    self.visit_with_depth(n, depth + 1)?;
                }
            }
            AstNode::VarDecl { value, .. } => {
                self.var_count += 1;
                self.visit_with_depth(value, depth + 1)?;
            }
            AstNode::Assignment { value, .. } => {
                self.visit_with_depth(value, depth + 1)?;
            }
            AstNode::BinaryOp { left, right, .. } => {
                self.visit_with_depth(left, depth + 1)?;
                self.visit_with_depth(right, depth + 1)?;
            }
            AstNode::Call { args, .. } => {
                self.call_count += 1;
                for arg in args {
                    self.visit_with_depth(arg, depth + 1)?;
                }
            }
            AstNode::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_with_depth(condition, depth + 1)?;
                for n in then_branch {
                    self.visit_with_depth(n, depth + 1)?;
                }
                if let Some(else_nodes) = else_branch {
                    for n in else_nodes {
                        self.visit_with_depth(n, depth + 1)?;
                    }
                }
            }
            AstNode::Return(expr) => {
                self.visit_with_depth(expr, depth + 1)?;
            }
            AstNode::Identifier(_) | AstNode::Literal(_) => {}
        }
        Ok(())
    }
}

impl Default for AstAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// AST transformer for code refactoring
pub struct AstTransformer {
    /// Variable rename map (old -> new)
    renames: HashMap<String, String>,
}

impl AstTransformer {
    pub fn new() -> Self {
        Self {
            renames: HashMap::new(),
        }
    }

    /// Add a variable rename rule
    pub fn add_rename(&mut self, old_name: String, new_name: String) {
        self.renames.insert(old_name, new_name);
    }

    /// Transform AST applying all rules
    pub fn transform(&self, node: AstNode) -> AstNode {
        match node {
            AstNode::Program(nodes) => {
                AstNode::Program(nodes.into_iter().map(|n| self.transform(n)).collect())
            }
            AstNode::Function { name, params, body } => AstNode::Function {
                name: self.rename_if_needed(&name),
                params: params.iter().map(|p| self.rename_if_needed(p)).collect(),
                body: body.into_iter().map(|n| self.transform(n)).collect(),
            },
            AstNode::VarDecl { name, value } => AstNode::VarDecl {
                name: self.rename_if_needed(&name),
                value: Box::new(self.transform(*value)),
            },
            AstNode::Assignment { target, value } => AstNode::Assignment {
                target: self.rename_if_needed(&target),
                value: Box::new(self.transform(*value)),
            },
            AstNode::BinaryOp { op, left, right } => AstNode::BinaryOp {
                op,
                left: Box::new(self.transform(*left)),
                right: Box::new(self.transform(*right)),
            },
            AstNode::Call { function, args } => AstNode::Call {
                function: self.rename_if_needed(&function),
                args: args.into_iter().map(|a| self.transform(a)).collect(),
            },
            AstNode::If {
                condition,
                then_branch,
                else_branch,
            } => AstNode::If {
                condition: Box::new(self.transform(*condition)),
                then_branch: then_branch.into_iter().map(|n| self.transform(n)).collect(),
                else_branch: else_branch
                    .map(|nodes| nodes.into_iter().map(|n| self.transform(n)).collect()),
            },
            AstNode::Return(expr) => AstNode::Return(Box::new(self.transform(*expr))),
            AstNode::Identifier(name) => AstNode::Identifier(self.rename_if_needed(&name)),
            AstNode::Literal(_) => node,
        }
    }

    fn rename_if_needed(&self, name: &str) -> String {
        self.renames.get(name).cloned().unwrap_or_else(|| name.to_string())
    }
}

impl Default for AstTransformer {
    fn default() -> Self {
        Self::new()
    }
}

/// AST code generator
pub struct CodeGenerator {
    indent_level: usize,
    indent_size: usize,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            indent_size: 4,
        }
    }

    pub fn generate(&mut self, ast: &AstNode) -> String {
        self.generate_node(ast)
    }

    fn generate_node(&mut self, node: &AstNode) -> String {
        match node {
            AstNode::Program(nodes) => nodes
                .iter()
                .map(|n| self.generate_node(n))
                .collect::<Vec<_>>()
                .join("\n"),
            AstNode::Function { name, params, body } => {
                let indent = self.indent();
                let params_str = params.join(", ");
                let mut result = format!("{}fn {}({}) {{\n", indent, name, params_str);
                self.indent_level += 1;
                for stmt in body {
                    result.push_str(&self.generate_node(stmt));
                    result.push('\n');
                }
                self.indent_level -= 1;
                result.push_str(&format!("{}}}", indent));
                result
            }
            AstNode::VarDecl { name, value } => {
                format!("{}let {} = {};", self.indent(), name, self.generate_expr(value))
            }
            AstNode::Assignment { target, value } => {
                format!("{}{} = {};", self.indent(), target, self.generate_expr(value))
            }
            AstNode::Call { function, args } => {
                let args_str = args
                    .iter()
                    .map(|a| self.generate_expr(a))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}{}({});", self.indent(), function, args_str)
            }
            AstNode::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let indent = self.indent();
                let mut result = format!("{}if {} {{\n", indent, self.generate_expr(condition));
                self.indent_level += 1;
                for stmt in then_branch {
                    result.push_str(&self.generate_node(stmt));
                    result.push('\n');
                }
                self.indent_level -= 1;
                result.push_str(&format!("{}}}", indent));
                if let Some(else_nodes) = else_branch {
                    result.push_str(" else {\n");
                    self.indent_level += 1;
                    for stmt in else_nodes {
                        result.push_str(&self.generate_node(stmt));
                        result.push('\n');
                    }
                    self.indent_level -= 1;
                    result.push_str(&format!("{}}}", indent));
                }
                result
            }
            AstNode::Return(expr) => {
                format!("{}return {};", self.indent(), self.generate_expr(expr))
            }
            _ => self.generate_expr(node),
        }
    }

    fn generate_expr(&self, node: &AstNode) -> String {
        match node {
            AstNode::Identifier(name) => name.clone(),
            AstNode::Literal(lit) => match lit {
                LiteralValue::Integer(n) => n.to_string(),
                LiteralValue::Float(f) => f.to_string(),
                LiteralValue::String(s) => format!("\"{}\"", s),
                LiteralValue::Boolean(b) => b.to_string(),
                LiteralValue::Null => "null".to_string(),
            },
            AstNode::BinaryOp { op, left, right } => {
                format!(
                    "({} {} {})",
                    self.generate_expr(left),
                    op,
                    self.generate_expr(right)
                )
            }
            AstNode::Call { function, args } => {
                let args_str = args
                    .iter()
                    .map(|a| self.generate_expr(a))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", function, args_str)
            }
            _ => String::new(),
        }
    }

    fn indent(&self) -> String {
        " ".repeat(self.indent_level * self.indent_size)
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

//
// Example 1: Basic AST parsing and traversal
//
pub fn example_1_ast_traversal() -> Result<()> {
    println!("=== Example 1: AST Traversal and Analysis ===\n");

    // Create a sample AST for a simple function
    let ast = AstNode::Program(vec![AstNode::Function {
        name: "calculate_sum".to_string(),
        params: vec!["a".to_string(), "b".to_string()],
        body: vec![
            AstNode::VarDecl {
                name: "result".to_string(),
                value: Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(AstNode::Identifier("a".to_string())),
                    right: Box::new(AstNode::Identifier("b".to_string())),
                }),
            },
            AstNode::Return(Box::new(AstNode::Identifier("result".to_string()))),
        ],
    }]);

    // Analyze the AST
    let mut analyzer = AstAnalyzer::new();
    analyzer.analyze(&ast)?;

    println!("AST Statistics:");
    println!("  Functions: {}", analyzer.function_count);
    println!("  Variables: {}", analyzer.var_count);
    println!("  Function calls: {}", analyzer.call_count);
    println!("  Max depth: {}", analyzer.max_depth);

    Ok(())
}

//
// Example 2: AST transformation
//
pub fn example_2_ast_transformation() -> Result<()> {
    println!("\n=== Example 2: AST Transformation ===\n");

    // Create AST with old naming
    let ast = AstNode::Program(vec![AstNode::Function {
        name: "foo".to_string(),
        params: vec!["x".to_string()],
        body: vec![
            AstNode::VarDecl {
                name: "temp".to_string(),
                value: Box::new(AstNode::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(AstNode::Identifier("x".to_string())),
                    right: Box::new(AstNode::Literal(LiteralValue::Integer(2))),
                }),
            },
            AstNode::Return(Box::new(AstNode::Identifier("temp".to_string()))),
        ],
    }]);

    println!("Original AST:");
    let mut codegen = CodeGenerator::new();
    println!("{}\n", codegen.generate(&ast));

    // Transform: rename variables
    let mut transformer = AstTransformer::new();
    transformer.add_rename("foo".to_string(), "double_value".to_string());
    transformer.add_rename("x".to_string(), "input".to_string());
    transformer.add_rename("temp".to_string(), "doubled".to_string());

    let transformed = transformer.transform(ast);

    println!("Transformed AST:");
    let mut codegen2 = CodeGenerator::new();
    println!("{}", codegen2.generate(&transformed));

    Ok(())
}

//
// Example 3: Complex AST with code generation
//
pub fn example_3_complex_ast() -> Result<()> {
    println!("\n=== Example 3: Complex AST Code Generation ===\n");

    // Create a more complex AST
    let ast = AstNode::Program(vec![AstNode::Function {
        name: "max".to_string(),
        params: vec!["a".to_string(), "b".to_string()],
        body: vec![AstNode::If {
            condition: Box::new(AstNode::BinaryOp {
                op: BinaryOperator::Greater,
                left: Box::new(AstNode::Identifier("a".to_string())),
                right: Box::new(AstNode::Identifier("b".to_string())),
            }),
            then_branch: vec![AstNode::Return(Box::new(AstNode::Identifier(
                "a".to_string(),
            )))],
            else_branch: Some(vec![AstNode::Return(Box::new(AstNode::Identifier(
                "b".to_string(),
            )))]),
        }],
    }]);

    // Analyze first
    let mut analyzer = AstAnalyzer::new();
    analyzer.analyze(&ast)?;

    println!("Function complexity:");
    println!("  Statements: {}", analyzer.var_count);
    println!("  Max nesting: {}", analyzer.max_depth);
    println!();

    // Generate code
    println!("Generated code:");
    let mut codegen = CodeGenerator::new();
    println!("{}", codegen.generate(&ast));

    Ok(())
}

fn main() -> Result<()> {
    example_1_ast_traversal()?;
    example_2_ast_transformation()?;
    example_3_complex_ast()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_node_creation() {
        let node = AstNode::Literal(LiteralValue::Integer(42));
        assert!(matches!(node, AstNode::Literal(LiteralValue::Integer(42))));
    }

    #[test]
    fn test_binary_operator_display() {
        assert_eq!(format!("{}", BinaryOperator::Add), "+");
        assert_eq!(format!("{}", BinaryOperator::Equal), "==");
        assert_eq!(format!("{}", BinaryOperator::And), "&&");
    }

    #[test]
    fn test_ast_analyzer_empty() {
        let ast = AstNode::Program(vec![]);
        let mut analyzer = AstAnalyzer::new();
        assert!(analyzer.analyze(&ast).is_ok());
        assert_eq!(analyzer.function_count, 0);
        assert_eq!(analyzer.var_count, 0);
    }

    #[test]
    fn test_ast_analyzer_simple_function() {
        let ast = AstNode::Function {
            name: "test".to_string(),
            params: vec![],
            body: vec![AstNode::Return(Box::new(AstNode::Literal(
                LiteralValue::Integer(1),
            )))],
        };

        let mut analyzer = AstAnalyzer::new();
        assert!(analyzer.analyze(&ast).is_ok());
        assert_eq!(analyzer.function_count, 1);
    }

    #[test]
    fn test_ast_analyzer_with_variables() {
        let ast = AstNode::Program(vec![AstNode::VarDecl {
            name: "x".to_string(),
            value: Box::new(AstNode::Literal(LiteralValue::Integer(10))),
        }]);

        let mut analyzer = AstAnalyzer::new();
        assert!(analyzer.analyze(&ast).is_ok());
        assert_eq!(analyzer.var_count, 1);
    }

    #[test]
    fn test_ast_analyzer_depth() {
        let ast = AstNode::Program(vec![AstNode::Function {
            name: "nested".to_string(),
            params: vec![],
            body: vec![AstNode::If {
                condition: Box::new(AstNode::Literal(LiteralValue::Boolean(true))),
                then_branch: vec![AstNode::Return(Box::new(AstNode::Literal(
                    LiteralValue::Integer(1),
                )))],
                else_branch: None,
            }],
        }]);

        let mut analyzer = AstAnalyzer::new();
        assert!(analyzer.analyze(&ast).is_ok());
        assert!(analyzer.max_depth >= 3);
    }

    #[test]
    fn test_transformer_rename() {
        let ast = AstNode::Identifier("old_name".to_string());
        let mut transformer = AstTransformer::new();
        transformer.add_rename("old_name".to_string(), "new_name".to_string());

        let transformed = transformer.transform(ast);
        assert_eq!(transformed, AstNode::Identifier("new_name".to_string()));
    }

    #[test]
    fn test_transformer_no_rename() {
        let ast = AstNode::Identifier("unchanged".to_string());
        let transformer = AstTransformer::new();

        let transformed = transformer.transform(ast.clone());
        assert_eq!(transformed, ast);
    }

    #[test]
    fn test_transformer_function_rename() {
        let ast = AstNode::Function {
            name: "old_func".to_string(),
            params: vec!["param".to_string()],
            body: vec![],
        };

        let mut transformer = AstTransformer::new();
        transformer.add_rename("old_func".to_string(), "new_func".to_string());

        let transformed = transformer.transform(ast);
        match transformed {
            AstNode::Function { name, .. } => {
                assert_eq!(name, "new_func");
            }
            _ => panic!("Expected Function node"),
        }
    }

    #[test]
    fn test_code_generator_literal() {
        let ast = AstNode::Literal(LiteralValue::Integer(42));
        let mut gen = CodeGenerator::new();
        assert_eq!(gen.generate(&ast), "42");
    }

    #[test]
    fn test_code_generator_identifier() {
        let ast = AstNode::Identifier("variable".to_string());
        let mut gen = CodeGenerator::new();
        assert_eq!(gen.generate(&ast), "variable");
    }

    #[test]
    fn test_code_generator_binary_op() {
        let ast = AstNode::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(AstNode::Literal(LiteralValue::Integer(1))),
            right: Box::new(AstNode::Literal(LiteralValue::Integer(2))),
        };

        let mut gen = CodeGenerator::new();
        assert_eq!(gen.generate(&ast), "(1 + 2)");
    }

    #[test]
    fn test_code_generator_var_decl() {
        let ast = AstNode::VarDecl {
            name: "x".to_string(),
            value: Box::new(AstNode::Literal(LiteralValue::Integer(10))),
        };

        let mut gen = CodeGenerator::new();
        let code = gen.generate(&ast);
        assert!(code.contains("let x = 10;"));
    }

    #[test]
    fn test_code_generator_function() {
        let ast = AstNode::Function {
            name: "test".to_string(),
            params: vec!["a".to_string()],
            body: vec![AstNode::Return(Box::new(AstNode::Identifier("a".to_string())))],
        };

        let mut gen = CodeGenerator::new();
        let code = gen.generate(&ast);
        assert!(code.contains("fn test(a)"));
        assert!(code.contains("return a;"));
    }

    #[test]
    fn test_literal_value_equality() {
        assert_eq!(
            LiteralValue::Integer(42),
            LiteralValue::Integer(42)
        );
        assert_ne!(
            LiteralValue::Integer(42),
            LiteralValue::Integer(43)
        );
    }

    #[test]
    fn test_complete_transformation_pipeline() {
        // Create AST
        let ast = AstNode::Program(vec![AstNode::Function {
            name: "calc".to_string(),
            params: vec!["x".to_string()],
            body: vec![
                AstNode::VarDecl {
                    name: "result".to_string(),
                    value: Box::new(AstNode::BinaryOp {
                        op: BinaryOperator::Multiply,
                        left: Box::new(AstNode::Identifier("x".to_string())),
                        right: Box::new(AstNode::Literal(LiteralValue::Integer(2))),
                    }),
                },
                AstNode::Return(Box::new(AstNode::Identifier("result".to_string()))),
            ],
        }]);

        // Analyze
        let mut analyzer = AstAnalyzer::new();
        analyzer.analyze(&ast).unwrap();
        assert_eq!(analyzer.function_count, 1);
        assert_eq!(analyzer.var_count, 1);

        // Transform
        let mut transformer = AstTransformer::new();
        transformer.add_rename("calc".to_string(), "double".to_string());
        let transformed = transformer.transform(ast);

        // Generate code
        let mut codegen = CodeGenerator::new();
        let code = codegen.generate(&transformed);
        assert!(code.contains("fn double"));
    }
}
