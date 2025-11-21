//! # Recipe 100-5: Simple File Transpilation
//!
//! **Level:** 100 (Foundational)
//! **Time Estimate:** 12 hours
//! **Priority:** P1 (High)
//!
//! ## Overview
//!
//! This recipe demonstrates basic file-level transpilation from Python to Rust.
//! It handles simple function definitions, basic types, and common operations.
//! This is a foundational example showing transpilation patterns that will be
//! extended in more advanced recipes.
//!
//! ## Supported Transformations
//!
//! - **Function Definitions:** Python `def` → Rust `fn`
//! - **Basic Types:** `int`, `float`, `str`, `bool` → Rust equivalents
//! - **Return Statements:** Python `return` → Rust `return`
//! - **Arithmetic Operations:** `+`, `-`, `*`, `/`, `%`
//! - **String Literals:** Python strings → Rust strings
//! - **Comments:** Python `#` → Rust `//`
//!
//! ## Limitations
//!
//! This is a *simple* transpilation example focused on teaching core concepts.
//! It does not handle:
//! - Complex control flow (loops, conditionals)
//! - Classes and OOP constructs
//! - Advanced Python features (list comprehensions, decorators, etc.)
//! - Error handling
//! - Type inference (requires explicit type hints)
//!
//! ## Examples
//!
//! Run individual examples with:
//! ```bash
//! cargo run --example recipe_100_5_simple_transpilation
//! ```
//!
//! ## Tests
//!
//! Run tests with:
//! ```bash
//! cargo test --example recipe_100_5_simple_transpilation
//! ```

use batuta_cookbook::{Error, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Represents a Python type annotation
#[derive(Debug, Clone, PartialEq)]
pub enum PythonType {
    /// int type
    Int,
    /// float type
    Float,
    /// str type
    Str,
    /// bool type
    Bool,
    /// Unknown or unspecified type
    Unknown,
}

impl PythonType {
    /// Parse Python type from string
    pub fn from_str(s: &str) -> Self {
        match s.trim() {
            "int" => Self::Int,
            "float" => Self::Float,
            "str" => Self::Str,
            "bool" => Self::Bool,
            _ => Self::Unknown,
        }
    }

    /// Convert to Rust type string
    pub fn to_rust_type(&self) -> &'static str {
        match self {
            Self::Int => "i64",
            Self::Float => "f64",
            Self::Str => "&str",
            Self::Bool => "bool",
            Self::Unknown => "/* unknown type */",
        }
    }
}

/// Represents a function parameter
#[derive(Debug, Clone)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: PythonType,
}

/// Represents a parsed Python function
#[derive(Debug, Clone)]
pub struct Function {
    /// Function name
    pub name: String,
    /// Parameters
    pub parameters: Vec<Parameter>,
    /// Return type
    pub return_type: PythonType,
    /// Function body lines
    pub body: Vec<String>,
}

/// Simple Python to Rust transpiler
pub struct Transpiler {
    /// Type mapping for additional custom types
    type_mapping: HashMap<String, String>,
}

impl Transpiler {
    /// Create a new transpiler
    pub fn new() -> Self {
        Self {
            type_mapping: HashMap::new(),
        }
    }

    /// Add custom type mapping
    pub fn add_type_mapping(&mut self, python_type: String, rust_type: String) {
        self.type_mapping.insert(python_type, rust_type);
    }

    /// Transpile Python source code to Rust
    pub fn transpile(&self, python_code: &str) -> Result<String> {
        let lines: Vec<&str> = python_code.lines().collect();
        let mut rust_code = String::new();

        // Add common imports
        rust_code.push_str("// Auto-generated Rust code from Python\n");
        rust_code.push_str("// Note: This is a simple transpilation and may require manual adjustments\n\n");

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim();

            if line.is_empty() {
                rust_code.push('\n');
                i += 1;
                continue;
            }

            // Handle comments
            if line.starts_with('#') {
                let comment = line.trim_start_matches('#').trim();
                rust_code.push_str(&format!("// {}\n", comment));
                i += 1;
                continue;
            }

            // Handle function definitions
            if line.starts_with("def ") {
                let function = self.parse_function(&lines, &mut i)?;
                let rust_fn = self.transpile_function(&function)?;
                rust_code.push_str(&rust_fn);
                rust_code.push('\n');
                continue;
            }

            i += 1;
        }

        Ok(rust_code)
    }

    /// Parse a Python function from source lines
    fn parse_function(&self, lines: &[&str], index: &mut usize) -> Result<Function> {
        let line = lines[*index].trim();

        // Parse function signature: def name(params) -> return_type:
        let without_def = line
            .strip_prefix("def ")
            .ok_or_else(|| Error::TranspilationError("Expected 'def' keyword".to_string()))?;

        let paren_pos = without_def
            .find('(')
            .ok_or_else(|| Error::TranspilationError("Expected '(' in function".to_string()))?;

        let name = without_def[..paren_pos].trim().to_string();

        // Extract parameters
        let params_end = without_def
            .find(')')
            .ok_or_else(|| Error::TranspilationError("Expected ')' in function".to_string()))?;

        let params_str = &without_def[paren_pos + 1..params_end];
        let parameters = self.parse_parameters(params_str)?;

        // Extract return type
        let return_type = if let Some(arrow_pos) = without_def.find("->") {
            let return_str = &without_def[arrow_pos + 2..];
            let return_str = return_str.trim().trim_end_matches(':').trim();
            PythonType::from_str(return_str)
        } else {
            PythonType::Unknown
        };

        // Parse function body
        *index += 1;
        let mut body = Vec::new();

        while *index < lines.len() {
            let body_line = lines[*index];

            // Check if line is indented (part of function body)
            if body_line.is_empty() || body_line.starts_with("    ") || body_line.starts_with('\t')
            {
                if !body_line.trim().is_empty() {
                    body.push(body_line.trim().to_string());
                }
                *index += 1;
            } else {
                break;
            }
        }

        Ok(Function {
            name,
            parameters,
            return_type,
            body,
        })
    }

    /// Parse function parameters
    fn parse_parameters(&self, params_str: &str) -> Result<Vec<Parameter>> {
        if params_str.trim().is_empty() {
            return Ok(Vec::new());
        }

        let mut parameters = Vec::new();

        for param in params_str.split(',') {
            let param = param.trim();
            if param.is_empty() {
                continue;
            }

            // Parse "name: type" format
            if let Some(colon_pos) = param.find(':') {
                let name = param[..colon_pos].trim().to_string();
                let type_str = param[colon_pos + 1..].trim();
                let param_type = PythonType::from_str(type_str);

                parameters.push(Parameter { name, param_type });
            } else {
                // No type annotation
                parameters.push(Parameter {
                    name: param.to_string(),
                    param_type: PythonType::Unknown,
                });
            }
        }

        Ok(parameters)
    }

    /// Transpile a function to Rust
    fn transpile_function(&self, function: &Function) -> Result<String> {
        let mut rust_fn = String::new();

        // Function signature
        rust_fn.push_str("pub fn ");
        rust_fn.push_str(&function.name);
        rust_fn.push('(');

        // Parameters
        for (i, param) in function.parameters.iter().enumerate() {
            if i > 0 {
                rust_fn.push_str(", ");
            }
            rust_fn.push_str(&param.name);
            rust_fn.push_str(": ");
            rust_fn.push_str(param.param_type.to_rust_type());
        }

        rust_fn.push(')');

        // Return type
        if function.return_type != PythonType::Unknown {
            rust_fn.push_str(" -> ");
            rust_fn.push_str(function.return_type.to_rust_type());
        }

        rust_fn.push_str(" {\n");

        // Function body
        for line in &function.body {
            let rust_line = self.transpile_statement(line)?;
            rust_fn.push_str("    ");
            rust_fn.push_str(&rust_line);
            rust_fn.push('\n');
        }

        rust_fn.push_str("}\n");

        Ok(rust_fn)
    }

    /// Transpile a single statement
    fn transpile_statement(&self, statement: &str) -> Result<String> {
        let statement = statement.trim();

        // Handle return statements
        if let Some(expr) = statement.strip_prefix("return ") {
            return Ok(format!("return {};", self.transpile_expression(expr)?));
        }

        // Handle comments
        if statement.starts_with('#') {
            let comment = statement.trim_start_matches('#').trim();
            return Ok(format!("// {}", comment));
        }

        // Handle variable assignments
        if let Some(eq_pos) = statement.find('=') {
            let var_name = statement[..eq_pos].trim();
            let value = statement[eq_pos + 1..].trim();
            return Ok(format!(
                "let {} = {};",
                var_name,
                self.transpile_expression(value)?
            ));
        }

        // Default: pass through with semicolon
        Ok(format!("{};", self.transpile_expression(statement)?))
    }

    /// Transpile an expression
    fn transpile_expression(&self, expr: &str) -> Result<String> {
        let expr = expr.trim();

        // Handle string literals
        if (expr.starts_with('"') && expr.ends_with('"'))
            || (expr.starts_with('\'') && expr.ends_with('\''))
        {
            // Python and Rust both use quotes, but Rust prefers &str
            return Ok(expr.to_string());
        }

        // Handle boolean literals
        if expr == "True" {
            return Ok("true".to_string());
        }
        if expr == "False" {
            return Ok("false".to_string());
        }

        // Handle None → Option::None (simplified)
        if expr == "None" {
            return Ok("None".to_string());
        }

        // For simple cases, pass through
        // In a real transpiler, this would handle operators, function calls, etc.
        Ok(expr.to_string())
    }

    /// Transpile a file
    pub fn transpile_file(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        let python_code = fs::read_to_string(input_path).map_err(|e| {
            Error::TranspilationError(format!("Failed to read input file: {}", e))
        })?;

        let rust_code = self.transpile(&python_code)?;

        fs::write(output_path, rust_code).map_err(|e| {
            Error::TranspilationError(format!("Failed to write output file: {}", e))
        })?;

        Ok(())
    }
}

impl Default for Transpiler {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// EXAMPLE 1: Simple Function Transpilation
// ============================================================================

fn example_1_simple_function() -> Result<()> {
    println!("=== Example 1: Simple Function Transpilation ===\n");

    let python_code = r#"
# Calculate the sum of two numbers
def add(x: int, y: int) -> int:
    return x + y

# Calculate the product of two numbers
def multiply(a: float, b: float) -> float:
    return a * b
"#;

    let transpiler = Transpiler::new();
    let rust_code = transpiler.transpile(python_code)?;

    println!("Python code:");
    println!("{}", python_code);
    println!("\nTranspiled to Rust:");
    println!("{}", rust_code);

    Ok(())
}

// ============================================================================
// EXAMPLE 2: String and Boolean Operations
// ============================================================================

fn example_2_types() -> Result<()> {
    println!("=== Example 2: String and Boolean Operations ===\n");

    let python_code = r#"
def greet(name: str) -> str:
    return "Hello, " + name

def is_valid(flag: bool) -> bool:
    return flag
"#;

    let transpiler = Transpiler::new();
    let rust_code = transpiler.transpile(python_code)?;

    println!("Python code:");
    println!("{}", python_code);
    println!("\nTranspiled to Rust:");
    println!("{}", rust_code);

    Ok(())
}

// ============================================================================
// EXAMPLE 3: File-to-File Transpilation
// ============================================================================

fn example_3_file_transpilation() -> Result<()> {
    println!("=== Example 3: File-to-File Transpilation ===\n");

    // Create temporary Python file
    let temp_dir = std::env::temp_dir();
    let input_path = temp_dir.join("sample.py");
    let output_path = temp_dir.join("sample.rs");

    let python_code = r#"# Sample Python module

def calculate_area(width: float, height: float) -> float:
    return width * height

def double(x: int) -> int:
    return x * 2
"#;

    fs::write(&input_path, python_code).map_err(|e| {
        Error::TranspilationError(format!("Failed to write temp file: {}", e))
    })?;

    // Transpile
    let transpiler = Transpiler::new();
    transpiler.transpile_file(&input_path, &output_path)?;

    // Read and display result
    let rust_code = fs::read_to_string(&output_path).map_err(|e| {
        Error::TranspilationError(format!("Failed to read output file: {}", e))
    })?;

    println!("Input file: {}", input_path.display());
    println!("Output file: {}", output_path.display());
    println!("\nGenerated Rust code:");
    println!("{}", rust_code);

    // Cleanup
    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&output_path);

    Ok(())
}

// ============================================================================
// MAIN FUNCTION - Run all examples
// ============================================================================

fn main() -> Result<()> {
    example_1_simple_function()?;
    println!("\n{}\n", "=".repeat(70));

    example_2_types()?;
    println!("\n{}\n", "=".repeat(70));

    example_3_file_transpilation()?;

    Ok(())
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_python_type_conversion() {
        assert_eq!(PythonType::from_str("int"), PythonType::Int);
        assert_eq!(PythonType::from_str("float"), PythonType::Float);
        assert_eq!(PythonType::from_str("str"), PythonType::Str);
        assert_eq!(PythonType::from_str("bool"), PythonType::Bool);

        assert_eq!(PythonType::Int.to_rust_type(), "i64");
        assert_eq!(PythonType::Float.to_rust_type(), "f64");
        assert_eq!(PythonType::Str.to_rust_type(), "&str");
        assert_eq!(PythonType::Bool.to_rust_type(), "bool");
    }

    #[test]
    fn test_parse_parameters() {
        let transpiler = Transpiler::new();

        let params = transpiler.parse_parameters("x: int, y: float").unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].name, "x");
        assert_eq!(params[0].param_type, PythonType::Int);
        assert_eq!(params[1].name, "y");
        assert_eq!(params[1].param_type, PythonType::Float);
    }

    #[test]
    fn test_parse_empty_parameters() {
        let transpiler = Transpiler::new();
        let params = transpiler.parse_parameters("").unwrap();
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn test_transpile_simple_function() {
        let transpiler = Transpiler::new();
        let python = r#"def add(x: int, y: int) -> int:
    return x + y"#;

        let rust = transpiler.transpile(python).unwrap();

        assert!(rust.contains("pub fn add"));
        assert!(rust.contains("x: i64"));
        assert!(rust.contains("y: i64"));
        assert!(rust.contains("-> i64"));
        assert!(rust.contains("return x + y;"));
    }

    #[test]
    fn test_transpile_comment() {
        let transpiler = Transpiler::new();
        let python = "# This is a comment";

        let rust = transpiler.transpile(python).unwrap();

        assert!(rust.contains("// This is a comment"));
    }

    #[test]
    fn test_transpile_boolean_literals() {
        let transpiler = Transpiler::new();

        assert_eq!(
            transpiler.transpile_expression("True").unwrap(),
            "true"
        );
        assert_eq!(
            transpiler.transpile_expression("False").unwrap(),
            "false"
        );
    }

    #[test]
    fn test_transpile_string_literal() {
        let transpiler = Transpiler::new();

        let result = transpiler.transpile_expression(r#""hello""#).unwrap();
        assert_eq!(result, r#""hello""#);
    }

    #[test]
    fn test_transpile_return_statement() {
        let transpiler = Transpiler::new();

        let result = transpiler.transpile_statement("return 42").unwrap();
        assert_eq!(result, "return 42;");
    }

    #[test]
    fn test_transpile_file() {
        let temp_dir = TempDir::new().unwrap();
        let input = temp_dir.path().join("test.py");
        let output = temp_dir.path().join("test.rs");

        let python_code = "def hello() -> str:\n    return \"Hello\"";
        fs::write(&input, python_code).unwrap();

        let transpiler = Transpiler::new();
        transpiler.transpile_file(&input, &output).unwrap();

        let rust_code = fs::read_to_string(&output).unwrap();
        assert!(rust_code.contains("pub fn hello"));
        assert!(rust_code.contains("-> &str"));
    }

    #[test]
    fn test_transpile_multiple_functions() {
        let transpiler = Transpiler::new();
        let python = r#"
def func1(x: int) -> int:
    return x

def func2(y: float) -> float:
    return y
"#;

        let rust = transpiler.transpile(python).unwrap();

        assert!(rust.contains("pub fn func1"));
        assert!(rust.contains("pub fn func2"));
    }

    #[test]
    fn test_custom_type_mapping() {
        let mut transpiler = Transpiler::new();
        transpiler.add_type_mapping("MyType".to_string(), "CustomRustType".to_string());

        // Verify the mapping was added
        assert_eq!(
            transpiler.type_mapping.get("MyType"),
            Some(&"CustomRustType".to_string())
        );
    }

    #[test]
    fn test_function_with_no_params() {
        let transpiler = Transpiler::new();
        let python = r#"def get_answer() -> int:
    return 42"#;

        let rust = transpiler.transpile(python).unwrap();

        assert!(rust.contains("pub fn get_answer()"));
        assert!(rust.contains("-> i64"));
        assert!(rust.contains("return 42;"));
    }

    #[test]
    fn test_parse_function_without_return_type() {
        let transpiler = Transpiler::new();
        let python = r#"def print_hello():
    return "hello""#;

        let rust = transpiler.transpile(python).unwrap();

        assert!(rust.contains("pub fn print_hello()"));
        // Should not have explicit return type annotation
        assert!(!rust.contains("-> i64"));
    }
}
