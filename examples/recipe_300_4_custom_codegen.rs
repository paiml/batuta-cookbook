//! # RECIPE-300-4: Custom Code Generation
//!
//! **Level:** 300 (Advanced)
//! **Estimated Time:** 26 hours
//! **Prerequisites:** RECIPE-300-2 (AST Manipulation), RECIPE-200-2 (Incremental Transpilation)
//!
//! ## Learning Objectives
//! - Design template-based code generators
//! - Support multiple output languages (Rust, Python, TypeScript)
//! - Implement variable substitution and formatting
//! - Generate idiomatic code for each target language
//! - Validate generated code structure
//!
//! ## Concepts Covered
//! - Template engines and variable interpolation
//! - Language-specific code patterns and idioms
//! - Code generation from specifications
//! - Type mapping across languages
//! - Formatting and pretty-printing
//!
//! ## Examples
//! This file demonstrates three approaches:
//! 1. Basic template-based generation for multiple languages
//! 2. Struct/class generation with fields and methods
//! 3. Function generation with type signatures

use batuta_cookbook::Result;
use std::fmt::Write as FmtWrite;

/// Target programming language for code generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetLanguage {
    Rust,
    Python,
    TypeScript,
    Go,
}

impl TargetLanguage {
    /// Get file extension for this language
    pub fn extension(&self) -> &str {
        match self {
            Self::Rust => "rs",
            Self::Python => "py",
            Self::TypeScript => "ts",
            Self::Go => "go",
        }
    }

    /// Get comment syntax for this language
    pub fn comment_prefix(&self) -> &str {
        match self {
            Self::Rust => "//",
            Self::Python => "#",
            Self::TypeScript => "//",
            Self::Go => "//",
        }
    }
}

/// Type information for code generation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeInfo {
    pub name: String,
    pub is_optional: bool,
    pub is_array: bool,
}

impl TypeInfo {
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_optional: false,
            is_array: false,
        }
    }

    pub fn optional(mut self) -> Self {
        self.is_optional = true;
        self
    }

    pub fn array(mut self) -> Self {
        self.is_array = true;
        self
    }

    /// Convert to language-specific type string
    pub fn to_language_type(&self, lang: TargetLanguage) -> String {
        let base_type = match lang {
            TargetLanguage::Rust => match self.name.as_str() {
                "string" => "String",
                "int" => "i64",
                "float" => "f64",
                "bool" => "bool",
                other => other,
            },
            TargetLanguage::Python => match self.name.as_str() {
                "string" => "str",
                "int" => "int",
                "float" => "float",
                "bool" => "bool",
                other => other,
            },
            TargetLanguage::TypeScript => match self.name.as_str() {
                "string" => "string",
                "int" => "number",
                "float" => "number",
                "bool" => "boolean",
                other => other,
            },
            TargetLanguage::Go => match self.name.as_str() {
                "string" => "string",
                "int" => "int64",
                "float" => "float64",
                "bool" => "bool",
                other => other,
            },
        };

        let mut result = base_type.to_string();

        if self.is_array {
            result = match lang {
                TargetLanguage::Rust => format!("Vec<{}>", result),
                TargetLanguage::Python => format!("list[{}]", result),
                TargetLanguage::TypeScript => format!("{}[]", result),
                TargetLanguage::Go => format!("[]{}", result),
            };
        }

        if self.is_optional {
            result = match lang {
                TargetLanguage::Rust => format!("Option<{}>", result),
                TargetLanguage::Python => format!("Optional[{}]", result),
                TargetLanguage::TypeScript => format!("{} | null", result),
                TargetLanguage::Go => format!("*{}", result),
            };
        }

        result
    }
}

/// Field definition for struct/class generation
#[derive(Debug, Clone)]
pub struct FieldSpec {
    pub name: String,
    pub type_info: TypeInfo,
    pub default_value: Option<String>,
    pub doc_comment: Option<String>,
}

impl FieldSpec {
    pub fn new(name: String, type_info: TypeInfo) -> Self {
        Self {
            name,
            type_info,
            default_value: None,
            doc_comment: None,
        }
    }

    pub fn with_default(mut self, value: String) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn with_doc(mut self, doc: String) -> Self {
        self.doc_comment = Some(doc);
        self
    }
}

/// Function parameter specification
#[derive(Debug, Clone)]
pub struct ParamSpec {
    pub name: String,
    pub type_info: TypeInfo,
}

impl ParamSpec {
    pub fn new(name: String, type_info: TypeInfo) -> Self {
        Self { name, type_info }
    }
}

/// Function specification for code generation
#[derive(Debug, Clone)]
pub struct FunctionSpec {
    pub name: String,
    pub params: Vec<ParamSpec>,
    pub return_type: Option<TypeInfo>,
    pub body: String,
    pub doc_comment: Option<String>,
}

impl FunctionSpec {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Vec::new(),
            return_type: None,
            body: String::new(),
            doc_comment: None,
        }
    }

    pub fn with_param(mut self, param: ParamSpec) -> Self {
        self.params.push(param);
        self
    }

    pub fn with_return(mut self, return_type: TypeInfo) -> Self {
        self.return_type = Some(return_type);
        self
    }

    pub fn with_body(mut self, body: String) -> Self {
        self.body = body;
        self
    }

    pub fn with_doc(mut self, doc: String) -> Self {
        self.doc_comment = Some(doc);
        self
    }
}

/// Struct/class specification for code generation
#[derive(Debug, Clone)]
pub struct StructSpec {
    pub name: String,
    pub fields: Vec<FieldSpec>,
    pub methods: Vec<FunctionSpec>,
    pub doc_comment: Option<String>,
}

impl StructSpec {
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: Vec::new(),
            methods: Vec::new(),
            doc_comment: None,
        }
    }

    pub fn with_field(mut self, field: FieldSpec) -> Self {
        self.fields.push(field);
        self
    }

    pub fn with_method(mut self, method: FunctionSpec) -> Self {
        self.methods.push(method);
        self
    }

    pub fn with_doc(mut self, doc: String) -> Self {
        self.doc_comment = Some(doc);
        self
    }
}

/// Code generator for multiple languages
pub struct CodeGenerator {
    target_language: TargetLanguage,
    _indent_size: usize,
}

impl CodeGenerator {
    pub fn new(target_language: TargetLanguage) -> Self {
        Self {
            target_language,
            _indent_size: 4,
        }
    }

    /// Generate a struct/class from specification
    pub fn generate_struct(&self, spec: &StructSpec) -> Result<String> {
        let mut output = String::new();

        // Add doc comment
        if let Some(doc) = &spec.doc_comment {
            self.write_doc_comment(&mut output, doc)?;
        }

        match self.target_language {
            TargetLanguage::Rust => self.generate_rust_struct(&mut output, spec)?,
            TargetLanguage::Python => self.generate_python_class(&mut output, spec)?,
            TargetLanguage::TypeScript => self.generate_typescript_class(&mut output, spec)?,
            TargetLanguage::Go => self.generate_go_struct(&mut output, spec)?,
        }

        Ok(output)
    }

    /// Generate a function from specification
    pub fn generate_function(&self, spec: &FunctionSpec) -> Result<String> {
        let mut output = String::new();

        // Add doc comment
        if let Some(doc) = &spec.doc_comment {
            self.write_doc_comment(&mut output, doc)?;
        }

        match self.target_language {
            TargetLanguage::Rust => self.generate_rust_function(&mut output, spec)?,
            TargetLanguage::Python => self.generate_python_function(&mut output, spec)?,
            TargetLanguage::TypeScript => self.generate_typescript_function(&mut output, spec)?,
            TargetLanguage::Go => self.generate_go_function(&mut output, spec)?,
        }

        Ok(output)
    }

    fn write_doc_comment(&self, output: &mut String, doc: &str) -> Result<()> {
        let prefix = self.target_language.comment_prefix();
        for line in doc.lines() {
            writeln!(output, "{} {}", prefix, line).map_err(|e| {
                batuta_cookbook::Error::Other(format!("Failed to write doc comment: {}", e))
            })?;
        }
        Ok(())
    }

    fn generate_rust_struct(&self, output: &mut String, spec: &StructSpec) -> Result<()> {
        writeln!(output, "#[derive(Debug, Clone)]").map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;
        writeln!(output, "pub struct {} {{", spec.name).map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        for field in &spec.fields {
            if let Some(doc) = &field.doc_comment {
                writeln!(output, "    /// {}", doc).map_err(|e| {
                    batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
                })?;
            }
            writeln!(
                output,
                "    pub {}: {},",
                field.name,
                field.type_info.to_language_type(self.target_language)
            )
            .map_err(|e| batuta_cookbook::Error::Other(format!("Failed to write: {}", e)))?;
        }

        writeln!(output, "}}").map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        // Generate methods
        if !spec.methods.is_empty() {
            writeln!(output, "\nimpl {} {{", spec.name).map_err(|e| {
                batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
            })?;

            for method in &spec.methods {
                self.generate_rust_method(output, method)?;
            }

            writeln!(output, "}}").map_err(|e| {
                batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
            })?;
        }

        Ok(())
    }

    fn generate_rust_method(&self, output: &mut String, spec: &FunctionSpec) -> Result<()> {
        if let Some(doc) = &spec.doc_comment {
            writeln!(output, "    /// {}", doc).map_err(|e| {
                batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
            })?;
        }

        let params: Vec<String> = spec
            .params
            .iter()
            .map(|p| {
                format!(
                    "{}: {}",
                    p.name,
                    p.type_info.to_language_type(self.target_language)
                )
            })
            .collect();

        let return_type = spec
            .return_type
            .as_ref()
            .map(|t| format!(" -> {}", t.to_language_type(self.target_language)))
            .unwrap_or_default();

        writeln!(
            output,
            "    pub fn {}(&self{}{}){} {{",
            spec.name,
            if params.is_empty() { "" } else { ", " },
            params.join(", "),
            return_type
        )
        .map_err(|e| batuta_cookbook::Error::Other(format!("Failed to write: {}", e)))?;

        writeln!(output, "        {}", spec.body).map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;
        writeln!(output, "    }}").map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        Ok(())
    }

    fn generate_python_class(&self, output: &mut String, spec: &StructSpec) -> Result<()> {
        writeln!(output, "class {}:", spec.name).map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        // __init__ method
        writeln!(output, "    def __init__(self):").map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        for field in &spec.fields {
            let default = field.default_value.as_deref().unwrap_or("None");
            writeln!(output, "        self.{} = {}", field.name, default).map_err(|e| {
                batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
            })?;
        }

        // Methods
        for method in &spec.methods {
            writeln!(output).map_err(|e| {
                batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
            })?;
            self.generate_python_method(output, method)?;
        }

        Ok(())
    }

    fn generate_python_method(&self, output: &mut String, spec: &FunctionSpec) -> Result<()> {
        let params: Vec<String> = spec
            .params
            .iter()
            .map(|p| format!("{}: {}", p.name, p.type_info.to_language_type(self.target_language)))
            .collect();

        let return_annotation = spec
            .return_type
            .as_ref()
            .map(|t| format!(" -> {}", t.to_language_type(self.target_language)))
            .unwrap_or_default();

        writeln!(
            output,
            "    def {}(self{}{}){}:",
            spec.name,
            if params.is_empty() { "" } else { ", " },
            params.join(", "),
            return_annotation
        )
        .map_err(|e| batuta_cookbook::Error::Other(format!("Failed to write: {}", e)))?;

        writeln!(output, "        {}", spec.body).map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        Ok(())
    }

    fn generate_typescript_class(&self, output: &mut String, spec: &StructSpec) -> Result<()> {
        writeln!(output, "class {} {{", spec.name).map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        // Fields
        for field in &spec.fields {
            writeln!(
                output,
                "    {}: {};",
                field.name,
                field.type_info.to_language_type(self.target_language)
            )
            .map_err(|e| batuta_cookbook::Error::Other(format!("Failed to write: {}", e)))?;
        }

        // Constructor
        writeln!(output, "\n    constructor() {{").map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        for field in &spec.fields {
            let default = field.default_value.as_deref().unwrap_or("null");
            writeln!(output, "        this.{} = {};", field.name, default).map_err(|e| {
                batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
            })?;
        }

        writeln!(output, "    }}").map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        // Methods
        for method in &spec.methods {
            writeln!(output).map_err(|e| {
                batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
            })?;
            self.generate_typescript_method(output, method)?;
        }

        writeln!(output, "}}").map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        Ok(())
    }

    fn generate_typescript_method(&self, output: &mut String, spec: &FunctionSpec) -> Result<()> {
        let params: Vec<String> = spec
            .params
            .iter()
            .map(|p| format!("{}: {}", p.name, p.type_info.to_language_type(self.target_language)))
            .collect();

        let return_type = spec
            .return_type
            .as_ref()
            .map(|t| format!(": {}", t.to_language_type(self.target_language)))
            .unwrap_or_else(|| ": void".to_string());

        writeln!(
            output,
            "    {}({}){} {{",
            spec.name,
            params.join(", "),
            return_type
        )
        .map_err(|e| batuta_cookbook::Error::Other(format!("Failed to write: {}", e)))?;

        writeln!(output, "        {}", spec.body).map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;
        writeln!(output, "    }}").map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        Ok(())
    }

    fn generate_go_struct(&self, output: &mut String, spec: &StructSpec) -> Result<()> {
        writeln!(output, "type {} struct {{", spec.name).map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        for field in &spec.fields {
            // Capitalize first letter for exported fields
            let field_name = capitalize_first(&field.name);
            writeln!(
                output,
                "    {} {}",
                field_name,
                field.type_info.to_language_type(self.target_language)
            )
            .map_err(|e| batuta_cookbook::Error::Other(format!("Failed to write: {}", e)))?;
        }

        writeln!(output, "}}").map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        Ok(())
    }

    fn generate_rust_function(&self, output: &mut String, spec: &FunctionSpec) -> Result<()> {
        let params: Vec<String> = spec
            .params
            .iter()
            .map(|p| {
                format!(
                    "{}: {}",
                    p.name,
                    p.type_info.to_language_type(self.target_language)
                )
            })
            .collect();

        let return_type = spec
            .return_type
            .as_ref()
            .map(|t| format!(" -> {}", t.to_language_type(self.target_language)))
            .unwrap_or_default();

        writeln!(output, "pub fn {}({}){} {{", spec.name, params.join(", "), return_type).map_err(
            |e| batuta_cookbook::Error::Other(format!("Failed to write: {}", e)),
        )?;

        writeln!(output, "    {}", spec.body).map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;
        writeln!(output, "}}").map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        Ok(())
    }

    fn generate_python_function(&self, output: &mut String, spec: &FunctionSpec) -> Result<()> {
        let params: Vec<String> = spec
            .params
            .iter()
            .map(|p| format!("{}: {}", p.name, p.type_info.to_language_type(self.target_language)))
            .collect();

        let return_annotation = spec
            .return_type
            .as_ref()
            .map(|t| format!(" -> {}", t.to_language_type(self.target_language)))
            .unwrap_or_default();

        writeln!(output, "def {}({}){}:", spec.name, params.join(", "), return_annotation)
            .map_err(|e| batuta_cookbook::Error::Other(format!("Failed to write: {}", e)))?;

        writeln!(output, "    {}", spec.body).map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        Ok(())
    }

    fn generate_typescript_function(
        &self,
        output: &mut String,
        spec: &FunctionSpec,
    ) -> Result<()> {
        let params: Vec<String> = spec
            .params
            .iter()
            .map(|p| format!("{}: {}", p.name, p.type_info.to_language_type(self.target_language)))
            .collect();

        let return_type = spec
            .return_type
            .as_ref()
            .map(|t| format!(": {}", t.to_language_type(self.target_language)))
            .unwrap_or_else(|| ": void".to_string());

        writeln!(
            output,
            "function {}({}){} {{",
            spec.name,
            params.join(", "),
            return_type
        )
        .map_err(|e| batuta_cookbook::Error::Other(format!("Failed to write: {}", e)))?;

        writeln!(output, "    {}", spec.body).map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;
        writeln!(output, "}}").map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        Ok(())
    }

    fn generate_go_function(&self, output: &mut String, spec: &FunctionSpec) -> Result<()> {
        let params: Vec<String> = spec
            .params
            .iter()
            .map(|p| format!("{} {}", p.name, p.type_info.to_language_type(self.target_language)))
            .collect();

        let return_type = spec
            .return_type
            .as_ref()
            .map(|t| format!(" {}", t.to_language_type(self.target_language)))
            .unwrap_or_default();

        writeln!(
            output,
            "func {}({}){} {{",
            capitalize_first(&spec.name),
            params.join(", "),
            return_type
        )
        .map_err(|e| batuta_cookbook::Error::Other(format!("Failed to write: {}", e)))?;

        writeln!(output, "    {}", spec.body).map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;
        writeln!(output, "}}").map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write: {}", e))
        })?;

        Ok(())
    }
}

/// Helper function to capitalize first letter
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

//
// Example 1: Generate simple struct in multiple languages
//
pub fn example_1_multi_language_struct() -> Result<()> {
    println!("=== Example 1: Multi-Language Struct Generation ===\n");

    let spec = StructSpec::new("User".to_string())
        .with_doc("Represents a user in the system".to_string())
        .with_field(
            FieldSpec::new("name".to_string(), TypeInfo::new("string".to_string()))
                .with_doc("User's full name".to_string()),
        )
        .with_field(FieldSpec::new("age".to_string(), TypeInfo::new("int".to_string())))
        .with_field(
            FieldSpec::new(
                "email".to_string(),
                TypeInfo::new("string".to_string()).optional(),
            )
            .with_doc("Optional email address".to_string()),
        );

    for lang in [
        TargetLanguage::Rust,
        TargetLanguage::Python,
        TargetLanguage::TypeScript,
    ] {
        println!("--- {} ---", format!("{:?}", lang));
        let generator = CodeGenerator::new(lang);
        let code = generator.generate_struct(&spec)?;
        println!("{}\n", code);
    }

    Ok(())
}

//
// Example 2: Generate class with methods
//
pub fn example_2_class_with_methods() -> Result<()> {
    println!("\n=== Example 2: Class with Methods ===\n");

    let spec = StructSpec::new("Counter".to_string())
        .with_field(FieldSpec::new(
            "value".to_string(),
            TypeInfo::new("int".to_string()),
        ))
        .with_method(
            FunctionSpec::new("increment".to_string())
                .with_doc("Increment the counter by 1".to_string())
                .with_body("self.value += 1".to_string()),
        )
        .with_method(
            FunctionSpec::new("get_value".to_string())
                .with_return(TypeInfo::new("int".to_string()))
                .with_body("return self.value".to_string()),
        );

    let generator = CodeGenerator::new(TargetLanguage::Python);
    let code = generator.generate_struct(&spec)?;
    println!("{}", code);

    Ok(())
}

//
// Example 3: Generate function with parameters
//
pub fn example_3_function_generation() -> Result<()> {
    println!("\n=== Example 3: Function Generation ===\n");

    let spec = FunctionSpec::new("calculate_sum".to_string())
        .with_doc("Calculate sum of two numbers".to_string())
        .with_param(ParamSpec::new(
            "a".to_string(),
            TypeInfo::new("int".to_string()),
        ))
        .with_param(ParamSpec::new(
            "b".to_string(),
            TypeInfo::new("int".to_string()),
        ))
        .with_return(TypeInfo::new("int".to_string()))
        .with_body("a + b".to_string());

    for lang in [TargetLanguage::Rust, TargetLanguage::TypeScript] {
        println!("--- {} ---", format!("{:?}", lang));
        let generator = CodeGenerator::new(lang);
        let code = generator.generate_function(&spec)?;
        println!("{}\n", code);
    }

    Ok(())
}

fn main() -> Result<()> {
    example_1_multi_language_struct()?;
    example_2_class_with_methods()?;
    example_3_function_generation()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_language_extension() {
        assert_eq!(TargetLanguage::Rust.extension(), "rs");
        assert_eq!(TargetLanguage::Python.extension(), "py");
        assert_eq!(TargetLanguage::TypeScript.extension(), "ts");
        assert_eq!(TargetLanguage::Go.extension(), "go");
    }

    #[test]
    fn test_type_info_basic() {
        let type_info = TypeInfo::new("string".to_string());
        assert_eq!(type_info.to_language_type(TargetLanguage::Rust), "String");
        assert_eq!(type_info.to_language_type(TargetLanguage::Python), "str");
    }

    #[test]
    fn test_type_info_optional() {
        let type_info = TypeInfo::new("int".to_string()).optional();
        assert_eq!(type_info.to_language_type(TargetLanguage::Rust), "Option<i64>");
        assert_eq!(
            type_info.to_language_type(TargetLanguage::TypeScript),
            "number | null"
        );
    }

    #[test]
    fn test_type_info_array() {
        let type_info = TypeInfo::new("string".to_string()).array();
        assert_eq!(type_info.to_language_type(TargetLanguage::Rust), "Vec<String>");
        assert_eq!(type_info.to_language_type(TargetLanguage::Python), "list[str]");
        assert_eq!(
            type_info.to_language_type(TargetLanguage::TypeScript),
            "string[]"
        );
    }

    #[test]
    fn test_field_spec_creation() {
        let field = FieldSpec::new("name".to_string(), TypeInfo::new("string".to_string()))
            .with_default("\"default\"".to_string())
            .with_doc("A name field".to_string());

        assert_eq!(field.name, "name");
        assert_eq!(field.default_value, Some("\"default\"".to_string()));
        assert_eq!(field.doc_comment, Some("A name field".to_string()));
    }

    #[test]
    fn test_function_spec_builder() {
        let func = FunctionSpec::new("test".to_string())
            .with_param(ParamSpec::new(
                "x".to_string(),
                TypeInfo::new("int".to_string()),
            ))
            .with_return(TypeInfo::new("bool".to_string()))
            .with_body("x > 0".to_string());

        assert_eq!(func.name, "test");
        assert_eq!(func.params.len(), 1);
        assert!(func.return_type.is_some());
        assert_eq!(func.body, "x > 0");
    }

    #[test]
    fn test_struct_spec_builder() {
        let spec = StructSpec::new("Test".to_string())
            .with_field(FieldSpec::new(
                "id".to_string(),
                TypeInfo::new("int".to_string()),
            ))
            .with_method(FunctionSpec::new("get_id".to_string()));

        assert_eq!(spec.name, "Test");
        assert_eq!(spec.fields.len(), 1);
        assert_eq!(spec.methods.len(), 1);
    }

    #[test]
    fn test_generate_rust_struct() {
        let spec = StructSpec::new("Point".to_string())
            .with_field(FieldSpec::new("x".to_string(), TypeInfo::new("int".to_string())))
            .with_field(FieldSpec::new("y".to_string(), TypeInfo::new("int".to_string())));

        let generator = CodeGenerator::new(TargetLanguage::Rust);
        let code = generator.generate_struct(&spec).unwrap();

        assert!(code.contains("pub struct Point"));
        assert!(code.contains("pub x: i64"));
        assert!(code.contains("pub y: i64"));
    }

    #[test]
    fn test_generate_python_class() {
        let spec = StructSpec::new("Person".to_string()).with_field(FieldSpec::new(
            "name".to_string(),
            TypeInfo::new("string".to_string()),
        ));

        let generator = CodeGenerator::new(TargetLanguage::Python);
        let code = generator.generate_struct(&spec).unwrap();

        assert!(code.contains("class Person:"));
        assert!(code.contains("def __init__(self):"));
        assert!(code.contains("self.name"));
    }

    #[test]
    fn test_generate_typescript_class() {
        let spec = StructSpec::new("Config".to_string()).with_field(FieldSpec::new(
            "enabled".to_string(),
            TypeInfo::new("bool".to_string()),
        ));

        let generator = CodeGenerator::new(TargetLanguage::TypeScript);
        let code = generator.generate_struct(&spec).unwrap();

        assert!(code.contains("class Config {"));
        assert!(code.contains("enabled: boolean"));
        assert!(code.contains("constructor()"));
    }

    #[test]
    fn test_generate_rust_function() {
        let spec = FunctionSpec::new("add".to_string())
            .with_param(ParamSpec::new(
                "a".to_string(),
                TypeInfo::new("int".to_string()),
            ))
            .with_param(ParamSpec::new(
                "b".to_string(),
                TypeInfo::new("int".to_string()),
            ))
            .with_return(TypeInfo::new("int".to_string()))
            .with_body("a + b".to_string());

        let generator = CodeGenerator::new(TargetLanguage::Rust);
        let code = generator.generate_function(&spec).unwrap();

        assert!(code.contains("pub fn add"));
        assert!(code.contains("a: i64"));
        assert!(code.contains("b: i64"));
        assert!(code.contains("-> i64"));
    }

    #[test]
    fn test_capitalize_first() {
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first("World"), "World");
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_comment_prefix() {
        assert_eq!(TargetLanguage::Rust.comment_prefix(), "//");
        assert_eq!(TargetLanguage::Python.comment_prefix(), "#");
        assert_eq!(TargetLanguage::TypeScript.comment_prefix(), "//");
        assert_eq!(TargetLanguage::Go.comment_prefix(), "//");
    }

    #[test]
    fn test_struct_with_methods() {
        let spec = StructSpec::new("Counter".to_string())
            .with_field(FieldSpec::new(
                "count".to_string(),
                TypeInfo::new("int".to_string()),
            ))
            .with_method(FunctionSpec::new("increment".to_string()));

        let generator = CodeGenerator::new(TargetLanguage::Rust);
        let code = generator.generate_struct(&spec).unwrap();

        assert!(code.contains("impl Counter"));
        assert!(code.contains("pub fn increment"));
    }

    #[test]
    fn test_optional_and_array_type() {
        let type_info = TypeInfo::new("string".to_string()).optional().array();
        let rust_type = type_info.to_language_type(TargetLanguage::Rust);

        assert!(rust_type.contains("Vec"));
        assert!(rust_type.contains("Option"));
    }
}
