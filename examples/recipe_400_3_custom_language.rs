//! # RECIPE-400-3: Custom Language Support
//!
//! **Level:** 400 (Expert)
//! **Estimated Time:** 48 hours
//! **Priority:** P3
//! **Prerequisites:** RECIPE-300-2 (AST Manipulation), RECIPE-300-4 (Custom Code Generation)
//!
//! ## Learning Objectives
//! - Define custom programming language specifications
//! - Implement basic lexical analysis (tokenization)
//! - Create simple parsers for custom syntax
//! - Register and manage multiple language definitions
//! - Implement custom transpilation rules
//!
//! ## Concepts Covered
//! - Language specification and grammar definitions
//! - Token types and lexical analysis
//! - Parsing strategies (recursive descent)
//! - Language feature detection
//! - Custom transformation rules
//! - Language registry and plugin architecture
//!
//! ## Examples
//! This file demonstrates three language implementations:
//! 1. Simple DSL (Domain-Specific Language) with basic syntax
//! 2. Extended language with custom operators
//! 3. Language registry and multi-language support

use batuta_cookbook::Result;
use std::collections::HashMap;

/// Token types for lexical analysis
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    /// Identifier (variable, function name)
    Identifier(String),
    /// Integer literal
    Integer(i64),
    /// String literal
    String(String),
    /// Keyword
    Keyword(String),
    /// Operator
    Operator(String),
    /// Symbol (parentheses, braces, etc.)
    Symbol(char),
    /// End of file
    Eof,
}

/// Token with position information
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self {
            token_type,
            line,
            column,
        }
    }
}

/// Language feature flags
#[derive(Debug, Clone)]
pub struct LanguageFeatures {
    pub has_classes: bool,
    pub has_generics: bool,
    pub has_pattern_matching: bool,
    pub has_async: bool,
    pub has_macros: bool,
    pub is_statically_typed: bool,
}

impl Default for LanguageFeatures {
    fn default() -> Self {
        Self {
            has_classes: false,
            has_generics: false,
            has_pattern_matching: false,
            has_async: false,
            has_macros: false,
            is_statically_typed: false,
        }
    }
}

/// Custom language specification
#[derive(Debug, Clone)]
pub struct LanguageSpec {
    pub name: String,
    pub version: String,
    pub file_extensions: Vec<String>,
    pub keywords: Vec<String>,
    pub operators: Vec<String>,
    pub features: LanguageFeatures,
    pub comment_syntax: CommentSyntax,
}

#[derive(Debug, Clone)]
pub struct CommentSyntax {
    pub single_line: Option<String>,
    pub multi_line_start: Option<String>,
    pub multi_line_end: Option<String>,
}

impl LanguageSpec {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            file_extensions: Vec::new(),
            keywords: Vec::new(),
            operators: Vec::new(),
            features: LanguageFeatures::default(),
            comment_syntax: CommentSyntax {
                single_line: None,
                multi_line_start: None,
                multi_line_end: None,
            },
        }
    }

    pub fn with_extension(mut self, ext: String) -> Self {
        self.file_extensions.push(ext);
        self
    }

    pub fn with_keyword(mut self, keyword: String) -> Self {
        self.keywords.push(keyword);
        self
    }

    pub fn with_operator(mut self, op: String) -> Self {
        self.operators.push(op);
        self
    }

    pub fn with_features(mut self, features: LanguageFeatures) -> Self {
        self.features = features;
        self
    }

    pub fn with_single_line_comment(mut self, syntax: String) -> Self {
        self.comment_syntax.single_line = Some(syntax);
        self
    }

    pub fn is_keyword(&self, word: &str) -> bool {
        self.keywords.iter().any(|k| k == word)
    }

    pub fn is_operator(&self, op: &str) -> bool {
        self.operators.iter().any(|o| o == op)
    }
}

/// Simple lexer for tokenization
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    spec: LanguageSpec,
}

impl Lexer {
    pub fn new(input: String, spec: LanguageSpec) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            spec,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            self.skip_whitespace();

            if self.position >= self.input.len() {
                break;
            }

            let ch = self.current_char();

            if ch.is_alphabetic() || ch == '_' {
                tokens.push(self.read_identifier()?);
            } else if ch.is_numeric() {
                tokens.push(self.read_number()?);
            } else if ch == '"' {
                tokens.push(self.read_string()?);
            } else if self.is_operator_start(ch) {
                tokens.push(self.read_operator()?);
            } else if self.is_symbol(ch) {
                tokens.push(self.read_symbol()?);
            } else {
                return Err(batuta_cookbook::Error::Other(format!(
                    "Unexpected character '{}' at line {}, column {}",
                    ch, self.line, self.column
                )));
            }
        }

        tokens.push(Token::new(TokenType::Eof, self.line, self.column));
        Ok(tokens)
    }

    fn current_char(&self) -> char {
        self.input[self.position]
    }

    fn advance(&mut self) {
        if self.position < self.input.len() && self.input[self.position] == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.current_char().is_whitespace() {
            self.advance();
        }
    }

    fn read_identifier(&mut self) -> Result<Token> {
        let line = self.line;
        let column = self.column;
        let mut identifier = String::new();

        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let token_type = if self.spec.is_keyword(&identifier) {
            TokenType::Keyword(identifier)
        } else {
            TokenType::Identifier(identifier)
        };

        Ok(Token::new(token_type, line, column))
    }

    fn read_number(&mut self) -> Result<Token> {
        let line = self.line;
        let column = self.column;
        let mut number = String::new();

        while self.position < self.input.len() && self.current_char().is_numeric() {
            number.push(self.current_char());
            self.advance();
        }

        let value = number.parse::<i64>().map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to parse number: {}", e))
        })?;

        Ok(Token::new(TokenType::Integer(value), line, column))
    }

    fn read_string(&mut self) -> Result<Token> {
        let line = self.line;
        let column = self.column;
        self.advance(); // Skip opening quote

        let mut string = String::new();
        while self.position < self.input.len() && self.current_char() != '"' {
            string.push(self.current_char());
            self.advance();
        }

        if self.position >= self.input.len() {
            return Err(batuta_cookbook::Error::Other(
                "Unterminated string literal".to_string(),
            ));
        }

        self.advance(); // Skip closing quote

        Ok(Token::new(TokenType::String(string), line, column))
    }

    fn read_operator(&mut self) -> Result<Token> {
        let line = self.line;
        let column = self.column;
        let mut op = String::new();

        // Try to read multi-character operators
        while self.position < self.input.len() && self.is_operator_start(self.current_char()) {
            op.push(self.current_char());
            if !self.spec.is_operator(&op) {
                // Backtrack if not a valid operator
                op.pop();
                break;
            }
            self.advance();
        }

        if op.is_empty() {
            op.push(self.current_char());
            self.advance();
        }

        Ok(Token::new(TokenType::Operator(op), line, column))
    }

    fn read_symbol(&mut self) -> Result<Token> {
        let line = self.line;
        let column = self.column;
        let symbol = self.current_char();
        self.advance();

        Ok(Token::new(TokenType::Symbol(symbol), line, column))
    }

    fn is_operator_start(&self, ch: char) -> bool {
        matches!(ch, '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' | '&' | '|')
    }

    fn is_symbol(&self, ch: char) -> bool {
        matches!(ch, '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | '.')
    }
}

/// Language registry for managing multiple languages
pub struct LanguageRegistry {
    languages: HashMap<String, LanguageSpec>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        Self {
            languages: HashMap::new(),
        }
    }

    pub fn register(&mut self, spec: LanguageSpec) -> Result<()> {
        if self.languages.contains_key(&spec.name) {
            return Err(batuta_cookbook::Error::Other(format!(
                "Language '{}' is already registered",
                spec.name
            )));
        }

        self.languages.insert(spec.name.clone(), spec);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&LanguageSpec> {
        self.languages.get(name)
    }

    pub fn get_by_extension(&self, ext: &str) -> Option<&LanguageSpec> {
        self.languages
            .values()
            .find(|spec| spec.file_extensions.contains(&ext.to_string()))
    }

    pub fn list_languages(&self) -> Vec<String> {
        self.languages.keys().cloned().collect()
    }

    pub fn count(&self) -> usize {
        self.languages.len()
    }
}

impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Transpilation rule for custom languages
#[derive(Debug, Clone)]
pub struct TranspilationRule {
    pub source_pattern: String,
    pub target_pattern: String,
    pub description: String,
}

impl TranspilationRule {
    pub fn new(source: String, target: String, desc: String) -> Self {
        Self {
            source_pattern: source,
            target_pattern: target,
            description: desc,
        }
    }
}

/// Custom language transpiler
pub struct CustomTranspiler {
    source_lang: LanguageSpec,
    _target_lang: String,
    rules: Vec<TranspilationRule>,
}

impl CustomTranspiler {
    pub fn new(source_lang: LanguageSpec, target_lang: String) -> Self {
        Self {
            source_lang,
            _target_lang: target_lang,
            rules: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: TranspilationRule) {
        self.rules.push(rule);
    }

    pub fn transpile(&self, source: &str) -> Result<String> {
        let mut lexer = Lexer::new(source.to_string(), self.source_lang.clone());
        let tokens = lexer.tokenize()?;

        // Simple token-based transpilation
        let mut result = String::new();
        for token in tokens {
            match &token.token_type {
                TokenType::Keyword(kw) => {
                    result.push_str(&self.transpile_keyword(kw));
                    result.push(' ');
                }
                TokenType::Identifier(id) => {
                    result.push_str(id);
                    result.push(' ');
                }
                TokenType::Integer(n) => {
                    result.push_str(&n.to_string());
                    result.push(' ');
                }
                TokenType::String(s) => {
                    result.push('"');
                    result.push_str(s);
                    result.push('"');
                    result.push(' ');
                }
                TokenType::Operator(op) => {
                    result.push_str(op);
                    result.push(' ');
                }
                TokenType::Symbol(sym) => {
                    result.push(*sym);
                    result.push(' ');
                }
                TokenType::Eof => {}
            }
        }

        Ok(result.trim().to_string())
    }

    fn transpile_keyword(&self, keyword: &str) -> String {
        // Apply transformation rules
        for rule in &self.rules {
            if rule.source_pattern == keyword {
                return rule.target_pattern.clone();
            }
        }
        keyword.to_string()
    }
}

//
// Example 1: Simple DSL definition and parsing
//
pub fn example_1_simple_dsl() -> Result<()> {
    println!("=== Example 1: Simple DSL Definition ===\n");

    // Define a simple calculator DSL
    let spec = LanguageSpec::new("CalcLang".to_string(), "1.0".to_string())
        .with_extension("calc".to_string())
        .with_keyword("let".to_string())
        .with_keyword("print".to_string())
        .with_operator("+".to_string())
        .with_operator("-".to_string())
        .with_operator("*".to_string())
        .with_operator("/".to_string())
        .with_operator("=".to_string())
        .with_single_line_comment("#".to_string());

    println!("Language: {} v{}", spec.name, spec.version);
    println!("Extensions: {:?}", spec.file_extensions);
    println!("Keywords: {:?}", spec.keywords);
    println!();

    // Tokenize sample code
    let code = "let x = 10 + 5";
    println!("Tokenizing: {}", code);

    let mut lexer = Lexer::new(code.to_string(), spec);
    let tokens = lexer.tokenize()?;

    println!("\nTokens:");
    for token in &tokens {
        if !matches!(token.token_type, TokenType::Eof) {
            println!("  {:?} at {}:{}", token.token_type, token.line, token.column);
        }
    }

    Ok(())
}

//
// Example 2: Language registry with multiple languages
//
pub fn example_2_language_registry() -> Result<()> {
    println!("\n=== Example 2: Language Registry ===\n");

    let mut registry = LanguageRegistry::new();

    // Register multiple languages
    let python_like = LanguageSpec::new("PyLike".to_string(), "1.0".to_string())
        .with_extension("pyl".to_string())
        .with_keyword("def".to_string())
        .with_keyword("class".to_string())
        .with_keyword("if".to_string())
        .with_features(LanguageFeatures {
            has_classes: true,
            is_statically_typed: false,
            ..Default::default()
        });

    let rust_like = LanguageSpec::new("RustLike".to_string(), "1.0".to_string())
        .with_extension("rsl".to_string())
        .with_keyword("fn".to_string())
        .with_keyword("struct".to_string())
        .with_keyword("impl".to_string())
        .with_features(LanguageFeatures {
            has_generics: true,
            has_pattern_matching: true,
            is_statically_typed: true,
            ..Default::default()
        });

    registry.register(python_like)?;
    registry.register(rust_like)?;

    println!("Registered languages:");
    for lang in registry.list_languages() {
        println!("  - {}", lang);
    }

    println!("\nLookup by extension '.pyl':");
    if let Some(lang) = registry.get_by_extension("pyl") {
        println!("  Found: {} (classes: {}, typed: {})",
            lang.name,
            lang.features.has_classes,
            lang.features.is_statically_typed
        );
    }

    Ok(())
}

//
// Example 3: Custom transpilation with rules
//
pub fn example_3_custom_transpilation() -> Result<()> {
    println!("\n=== Example 3: Custom Transpilation ===\n");

    let source_lang = LanguageSpec::new("SimpleLang".to_string(), "1.0".to_string())
        .with_keyword("func".to_string())
        .with_keyword("var".to_string())
        .with_keyword("return".to_string())
        .with_operator("=".to_string())
        .with_operator("+".to_string());

    let mut transpiler = CustomTranspiler::new(source_lang, "rust".to_string());

    // Add transpilation rules
    transpiler.add_rule(TranspilationRule::new(
        "func".to_string(),
        "fn".to_string(),
        "Function declaration".to_string(),
    ));
    transpiler.add_rule(TranspilationRule::new(
        "var".to_string(),
        "let".to_string(),
        "Variable declaration".to_string(),
    ));

    let source = "func add ( x , y ) { return x + y }";
    println!("Source code: {}", source);

    let result = transpiler.transpile(source)?;
    println!("Transpiled: {}", result);

    Ok(())
}

fn main() -> Result<()> {
    example_1_simple_dsl()?;
    example_2_language_registry()?;
    example_3_custom_transpilation()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_spec_creation() {
        let spec = LanguageSpec::new("Test".to_string(), "1.0".to_string());
        assert_eq!(spec.name, "Test");
        assert_eq!(spec.version, "1.0");
    }

    #[test]
    fn test_language_spec_with_keyword() {
        let spec = LanguageSpec::new("Test".to_string(), "1.0".to_string())
            .with_keyword("func".to_string());

        assert!(spec.is_keyword("func"));
        assert!(!spec.is_keyword("var"));
    }

    #[test]
    fn test_lexer_tokenize_identifier() {
        let spec = LanguageSpec::new("Test".to_string(), "1.0".to_string());
        let mut lexer = Lexer::new("hello".to_string(), spec);

        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // identifier + EOF

        match &tokens[0].token_type {
            TokenType::Identifier(id) => assert_eq!(id, "hello"),
            _ => panic!("Expected identifier"),
        }
    }

    #[test]
    fn test_lexer_tokenize_number() {
        let spec = LanguageSpec::new("Test".to_string(), "1.0".to_string());
        let mut lexer = Lexer::new("42".to_string(), spec);

        let tokens = lexer.tokenize().unwrap();
        match &tokens[0].token_type {
            TokenType::Integer(n) => assert_eq!(*n, 42),
            _ => panic!("Expected integer"),
        }
    }

    #[test]
    fn test_lexer_tokenize_string() {
        let spec = LanguageSpec::new("Test".to_string(), "1.0".to_string());
        let mut lexer = Lexer::new("\"hello\"".to_string(), spec);

        let tokens = lexer.tokenize().unwrap();
        match &tokens[0].token_type {
            TokenType::String(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_lexer_tokenize_keyword() {
        let spec = LanguageSpec::new("Test".to_string(), "1.0".to_string())
            .with_keyword("let".to_string());
        let mut lexer = Lexer::new("let x".to_string(), spec);

        let tokens = lexer.tokenize().unwrap();
        match &tokens[0].token_type {
            TokenType::Keyword(kw) => assert_eq!(kw, "let"),
            _ => panic!("Expected keyword"),
        }
    }

    #[test]
    fn test_lexer_tokenize_operator() {
        let spec = LanguageSpec::new("Test".to_string(), "1.0".to_string())
            .with_operator("+".to_string());
        let mut lexer = Lexer::new("1 + 2".to_string(), spec);

        let tokens = lexer.tokenize().unwrap();
        match &tokens[1].token_type {
            TokenType::Operator(op) => assert_eq!(op, "+"),
            _ => panic!("Expected operator"),
        }
    }

    #[test]
    fn test_language_registry_register() {
        let mut registry = LanguageRegistry::new();
        let spec = LanguageSpec::new("Test".to_string(), "1.0".to_string());

        assert!(registry.register(spec).is_ok());
        assert_eq!(registry.count(), 1);
    }

    #[test]
    fn test_language_registry_duplicate() {
        let mut registry = LanguageRegistry::new();
        let spec1 = LanguageSpec::new("Test".to_string(), "1.0".to_string());
        let spec2 = LanguageSpec::new("Test".to_string(), "2.0".to_string());

        registry.register(spec1).unwrap();
        assert!(registry.register(spec2).is_err());
    }

    #[test]
    fn test_language_registry_get() {
        let mut registry = LanguageRegistry::new();
        let spec = LanguageSpec::new("Test".to_string(), "1.0".to_string());
        registry.register(spec).unwrap();

        assert!(registry.get("Test").is_some());
        assert!(registry.get("NonExistent").is_none());
    }

    #[test]
    fn test_language_registry_get_by_extension() {
        let mut registry = LanguageRegistry::new();
        let spec = LanguageSpec::new("Test".to_string(), "1.0".to_string())
            .with_extension("test".to_string());
        registry.register(spec).unwrap();

        assert!(registry.get_by_extension("test").is_some());
        assert!(registry.get_by_extension("other").is_none());
    }

    #[test]
    fn test_transpilation_rule() {
        let rule = TranspilationRule::new(
            "func".to_string(),
            "fn".to_string(),
            "Function".to_string(),
        );

        assert_eq!(rule.source_pattern, "func");
        assert_eq!(rule.target_pattern, "fn");
    }

    #[test]
    fn test_custom_transpiler_creation() {
        let spec = LanguageSpec::new("Test".to_string(), "1.0".to_string());
        let transpiler = CustomTranspiler::new(spec, "rust".to_string());

        assert_eq!(transpiler.rules.len(), 0);
    }

    #[test]
    fn test_custom_transpiler_with_rules() {
        let spec = LanguageSpec::new("Test".to_string(), "1.0".to_string())
            .with_keyword("func".to_string());

        let mut transpiler = CustomTranspiler::new(spec, "rust".to_string());
        transpiler.add_rule(TranspilationRule::new(
            "func".to_string(),
            "fn".to_string(),
            "Function".to_string(),
        ));

        assert_eq!(transpiler.rules.len(), 1);
    }

    #[test]
    fn test_language_features_default() {
        let features = LanguageFeatures::default();
        assert!(!features.has_classes);
        assert!(!features.is_statically_typed);
    }

    #[test]
    fn test_token_creation() {
        let token = Token::new(TokenType::Integer(42), 1, 1);
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }
}
