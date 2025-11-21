//! RECIPE-400-5: ML-Driven Optimization
//!
//! This recipe demonstrates machine learning-driven code optimization using
//! predictive models to suggest optimization strategies, predict performance,
//! and adapt to code patterns automatically.
//!
//! Learning Objectives:
//! - Feature extraction from code (complexity, size, patterns)
//! - ML model training with historical optimization data
//! - Optimization strategy prediction and recommendation
//! - Performance prediction before actual optimization
//! - Model evaluation and accuracy metrics
//! - Transfer learning and model adaptation
//! - Continuous learning from optimization results
//!
//! Level: Expert (400)
//! Estimated Time: 52 hours
//! Prerequisites: RECIPE-200-4 (Optimization Profiles), RECIPE-300-5 (Performance Profiling)

use std::collections::HashMap;
use std::time::Duration;

type Result<T> = std::result::Result<T, String>;

// ============================================================================
// Core Types
// ============================================================================

/// Code features extracted for ML model
#[derive(Debug, Clone)]
pub struct CodeFeatures {
    pub lines_of_code: usize,
    pub cyclomatic_complexity: usize,
    pub function_count: usize,
    pub loop_count: usize,
    pub recursion_depth: usize,
    pub memory_allocations: usize,
    pub io_operations: usize,
    pub dependencies_count: usize,
}

/// Optimization strategy that can be applied
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptimizationStrategy {
    LoopUnrolling,
    Inlining,
    ConstantFolding,
    DeadCodeElimination,
    MemoryPooling,
    Parallelization,
    Vectorization,
    CacheOptimization,
}

/// Historical data point for training
#[derive(Debug, Clone)]
pub struct TrainingExample {
    pub features: CodeFeatures,
    pub strategy: OptimizationStrategy,
    pub speedup: f64,
    pub success: bool,
}

/// Prediction from ML model
#[derive(Debug, Clone)]
pub struct OptimizationPrediction {
    pub strategy: OptimizationStrategy,
    pub confidence: f64,
    pub estimated_speedup: f64,
    pub reasoning: Vec<String>,
}

/// Performance before and after optimization
#[derive(Debug, Clone)]
pub struct PerformanceResult {
    pub baseline_time: Duration,
    pub optimized_time: Duration,
    pub actual_speedup: f64,
    pub memory_saved: usize,
}

// ============================================================================
// Feature Extraction
// ============================================================================

pub struct FeatureExtractor;

impl FeatureExtractor {
    pub fn extract(code: &str) -> CodeFeatures {
        let lines_of_code = code.lines().filter(|l| !l.trim().is_empty()).count();
        let function_count = code.matches("fn ").count();
        let loop_count = code.matches("for ").count() + code.matches("while ").count();
        let recursion_depth = Self::estimate_recursion_depth(code);
        let memory_allocations = code.matches("Vec::new").count()
            + code.matches("Box::new").count()
            + code.matches(".to_string()").count();
        let io_operations = code.matches("read").count() + code.matches("write").count();
        let dependencies_count = code.matches("use ").count();

        // Simplified cyclomatic complexity: 1 + number of decision points
        let complexity = 1
            + code.matches("if ").count()
            + code.matches("match ").count()
            + code.matches("while ").count()
            + code.matches("for ").count();

        CodeFeatures {
            lines_of_code,
            cyclomatic_complexity: complexity,
            function_count,
            loop_count,
            recursion_depth,
            memory_allocations,
            io_operations,
            dependencies_count,
        }
    }

    fn estimate_recursion_depth(code: &str) -> usize {
        // Simple heuristic: count recursive function calls
        let mut max_depth = 0;
        for line in code.lines() {
            if line.contains("fn ") && line.contains("self.") {
                max_depth += 1;
            }
        }
        max_depth
    }
}

// ============================================================================
// ML Model (Simplified Decision Tree)
// ============================================================================

pub struct MlOptimizer {
    training_data: Vec<TrainingExample>,
    strategy_scores: HashMap<OptimizationStrategy, f64>,
    feature_weights: FeatureWeights,
}

#[derive(Debug, Clone)]
pub struct FeatureWeights {
    pub complexity_weight: f64,
    pub loop_weight: f64,
    pub memory_weight: f64,
    pub io_weight: f64,
}

impl Default for FeatureWeights {
    fn default() -> Self {
        Self {
            complexity_weight: 1.0,
            loop_weight: 1.5,
            memory_weight: 1.2,
            io_weight: 0.8,
        }
    }
}

impl MlOptimizer {
    pub fn new() -> Self {
        Self {
            training_data: Vec::new(),
            strategy_scores: HashMap::new(),
            feature_weights: FeatureWeights::default(),
        }
    }

    pub fn train(&mut self, examples: Vec<TrainingExample>) -> Result<TrainingMetrics> {
        self.training_data.extend(examples);

        // Calculate success rates for each strategy
        let mut strategy_stats: HashMap<OptimizationStrategy, (usize, usize, f64)> = HashMap::new();

        for example in &self.training_data {
            let (successes, total, speedup_sum) = strategy_stats
                .entry(example.strategy)
                .or_insert((0, 0, 0.0));

            *total += 1;
            *speedup_sum += example.speedup;
            if example.success {
                *successes += 1;
            }
        }

        // Update strategy scores based on success rate and average speedup
        for (strategy, (successes, total, speedup_sum)) in strategy_stats {
            let success_rate = successes as f64 / total as f64;
            let avg_speedup = speedup_sum / total as f64;
            self.strategy_scores.insert(strategy, success_rate * avg_speedup);
        }

        // Update feature weights based on correlation analysis
        self.update_feature_weights();

        Ok(TrainingMetrics {
            examples_processed: self.training_data.len(),
            strategies_learned: self.strategy_scores.len(),
            average_accuracy: self.calculate_accuracy(),
        })
    }

    pub fn predict(&self, features: &CodeFeatures) -> Vec<OptimizationPrediction> {
        let mut predictions = Vec::new();

        // Score each strategy based on code features
        for (&strategy, &base_score) in &self.strategy_scores {
            let feature_score = self.calculate_feature_score(features, strategy);
            let confidence = (base_score * feature_score).min(1.0);
            let estimated_speedup = self.estimate_speedup(features, strategy);

            predictions.push(OptimizationPrediction {
                strategy,
                confidence,
                estimated_speedup,
                reasoning: self.generate_reasoning(features, strategy),
            });
        }

        // Sort by confidence and estimated speedup
        predictions.sort_by(|a, b| {
            let score_a = a.confidence * a.estimated_speedup;
            let score_b = b.confidence * b.estimated_speedup;
            score_b.partial_cmp(&score_a).unwrap()
        });

        predictions
    }

    pub fn recommend(&self, features: &CodeFeatures) -> OptimizationPrediction {
        let predictions = self.predict(features);
        predictions.into_iter().next()
            .unwrap_or_else(|| OptimizationPrediction {
                strategy: OptimizationStrategy::DeadCodeElimination,
                confidence: 0.5,
                estimated_speedup: 1.1,
                reasoning: vec!["Default recommendation".to_string()],
            })
    }

    fn calculate_feature_score(&self, features: &CodeFeatures, strategy: OptimizationStrategy) -> f64 {
        match strategy {
            OptimizationStrategy::LoopUnrolling => {
                if features.loop_count > 0 {
                    (features.loop_count as f64 * self.feature_weights.loop_weight).min(2.0)
                } else {
                    0.1
                }
            }
            OptimizationStrategy::Inlining => {
                if features.function_count > 10 && features.lines_of_code < 500 {
                    1.5
                } else if features.function_count > 0 {
                    0.8
                } else {
                    0.2
                }
            }
            OptimizationStrategy::ConstantFolding => {
                if features.cyclomatic_complexity < 10 {
                    1.0 + (10 - features.cyclomatic_complexity) as f64 * 0.1
                } else {
                    0.5
                }
            }
            OptimizationStrategy::DeadCodeElimination => {
                if features.lines_of_code > 200 {
                    1.2
                } else {
                    0.6
                }
            }
            OptimizationStrategy::MemoryPooling => {
                if features.memory_allocations > 10 {
                    (features.memory_allocations as f64 * self.feature_weights.memory_weight * 0.1).min(2.0)
                } else {
                    0.3
                }
            }
            OptimizationStrategy::Parallelization => {
                if features.loop_count > 2 && features.io_operations < 5 {
                    1.8
                } else {
                    0.4
                }
            }
            OptimizationStrategy::Vectorization => {
                if features.loop_count > 0 && features.lines_of_code > 100 {
                    1.6
                } else {
                    0.3
                }
            }
            OptimizationStrategy::CacheOptimization => {
                if features.memory_allocations > 5 && features.loop_count > 0 {
                    1.4
                } else {
                    0.5
                }
            }
        }
    }

    fn estimate_speedup(&self, features: &CodeFeatures, strategy: OptimizationStrategy) -> f64 {
        let base_speedup = self.strategy_scores.get(&strategy).copied().unwrap_or(1.1);
        let feature_factor = self.calculate_feature_score(features, strategy);

        (base_speedup * feature_factor).max(1.0).min(10.0)
    }

    fn generate_reasoning(&self, features: &CodeFeatures, strategy: OptimizationStrategy) -> Vec<String> {
        let mut reasoning = Vec::new();

        match strategy {
            OptimizationStrategy::LoopUnrolling => {
                if features.loop_count > 0 {
                    reasoning.push(format!("Code contains {} loops that could benefit from unrolling", features.loop_count));
                }
                if features.loop_count > 5 {
                    reasoning.push("High loop count indicates significant unrolling potential".to_string());
                }
            }
            OptimizationStrategy::Inlining => {
                if features.function_count > 10 {
                    reasoning.push(format!("{} functions detected, inlining small functions could reduce call overhead", features.function_count));
                }
            }
            OptimizationStrategy::MemoryPooling => {
                if features.memory_allocations > 10 {
                    reasoning.push(format!("{} memory allocations detected, pooling could reduce allocation overhead", features.memory_allocations));
                }
            }
            OptimizationStrategy::Parallelization => {
                if features.loop_count > 2 {
                    reasoning.push("Multiple loops detected, suitable for parallel execution".to_string());
                }
                if features.io_operations < 5 {
                    reasoning.push("Low I/O operations, good for CPU-bound parallelization".to_string());
                }
            }
            _ => {
                reasoning.push(format!("Strategy {:?} recommended based on code patterns", strategy));
            }
        }

        if reasoning.is_empty() {
            reasoning.push("General optimization recommended".to_string());
        }

        reasoning
    }

    fn update_feature_weights(&mut self) {
        // Simplified weight update based on success patterns
        let mut loop_successes = 0;
        let mut loop_total = 0;

        for example in &self.training_data {
            if example.features.loop_count > 0 {
                loop_total += 1;
                if example.success {
                    loop_successes += 1;
                }
            }
        }

        if loop_total > 0 {
            let loop_success_rate = loop_successes as f64 / loop_total as f64;
            self.feature_weights.loop_weight = 1.0 + loop_success_rate;
        }
    }

    fn calculate_accuracy(&self) -> f64 {
        if self.training_data.is_empty() {
            return 0.0;
        }

        let successes = self.training_data.iter()
            .filter(|e| e.success)
            .count();

        (successes as f64 / self.training_data.len() as f64) * 100.0
    }

    pub fn evaluate(&self, test_data: &[TrainingExample]) -> EvaluationMetrics {
        let mut correct_predictions = 0;
        let mut total_predictions = 0;
        let mut speedup_errors = Vec::new();

        for example in test_data {
            total_predictions += 1;
            let prediction = self.recommend(&example.features);

            if prediction.strategy == example.strategy {
                correct_predictions += 1;
            }

            let error = (prediction.estimated_speedup - example.speedup).abs();
            speedup_errors.push(error);
        }

        let accuracy = if total_predictions > 0 {
            (correct_predictions as f64 / total_predictions as f64) * 100.0
        } else {
            0.0
        };

        let mae = if !speedup_errors.is_empty() {
            speedup_errors.iter().sum::<f64>() / speedup_errors.len() as f64
        } else {
            0.0
        };

        EvaluationMetrics {
            accuracy,
            correct_predictions,
            total_predictions,
            mean_absolute_error: mae,
        }
    }
}

// ============================================================================
// Metrics
// ============================================================================

#[derive(Debug, Clone)]
pub struct TrainingMetrics {
    pub examples_processed: usize,
    pub strategies_learned: usize,
    pub average_accuracy: f64,
}

#[derive(Debug, Clone)]
pub struct EvaluationMetrics {
    pub accuracy: f64,
    pub correct_predictions: usize,
    pub total_predictions: usize,
    pub mean_absolute_error: f64,
}

// ============================================================================
// Transfer Learning
// ============================================================================

pub struct TransferLearner {
    source_model: MlOptimizer,
    target_domain: String,
}

impl TransferLearner {
    pub fn new(source_model: MlOptimizer, target_domain: String) -> Self {
        Self {
            source_model,
            target_domain,
        }
    }

    pub fn adapt(&mut self, target_examples: Vec<TrainingExample>) -> Result<AdaptationMetrics> {
        // Fine-tune the model with target domain data
        let initial_accuracy = self.source_model.calculate_accuracy();

        self.source_model.train(target_examples.clone())?;

        let final_accuracy = self.source_model.calculate_accuracy();
        let improvement = final_accuracy - initial_accuracy;

        Ok(AdaptationMetrics {
            domain: self.target_domain.clone(),
            initial_accuracy,
            final_accuracy,
            improvement,
            examples_used: target_examples.len(),
        })
    }

    pub fn predict(&self, features: &CodeFeatures) -> Vec<OptimizationPrediction> {
        self.source_model.predict(features)
    }
}

#[derive(Debug, Clone)]
pub struct AdaptationMetrics {
    pub domain: String,
    pub initial_accuracy: f64,
    pub final_accuracy: f64,
    pub improvement: f64,
    pub examples_used: usize,
}

// ============================================================================
// Examples
// ============================================================================

fn main() -> Result<()> {
    println!("=== Example 1: Feature Extraction and Training ===\n");
    example_feature_extraction()?;

    println!("\n=== Example 2: Optimization Prediction ===\n");
    example_optimization_prediction()?;

    println!("\n=== Example 3: Transfer Learning ===\n");
    example_transfer_learning()?;

    Ok(())
}

fn example_feature_extraction() -> Result<()> {
    let sample_code = r#"
fn calculate_sum(data: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..data.len() {
        if data[i] > 0 {
            sum += data[i];
        }
    }
    sum
}

fn process_data(input: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for value in input {
        if value > 10 {
            result.push(value * 2);
        }
    }
    result
}
"#;

    let features = FeatureExtractor::extract(sample_code);

    println!("Extracted Features:");
    println!("  Lines of Code: {}", features.lines_of_code);
    println!("  Cyclomatic Complexity: {}", features.cyclomatic_complexity);
    println!("  Function Count: {}", features.function_count);
    println!("  Loop Count: {}", features.loop_count);
    println!("  Memory Allocations: {}", features.memory_allocations);

    // Create training data
    let training_examples = vec![
        TrainingExample {
            features: features.clone(),
            strategy: OptimizationStrategy::LoopUnrolling,
            speedup: 1.8,
            success: true,
        },
        TrainingExample {
            features: features.clone(),
            strategy: OptimizationStrategy::Vectorization,
            speedup: 2.1,
            success: true,
        },
    ];

    let mut optimizer = MlOptimizer::new();
    let metrics = optimizer.train(training_examples)?;

    println!("\nTraining Metrics:");
    println!("  Examples Processed: {}", metrics.examples_processed);
    println!("  Strategies Learned: {}", metrics.strategies_learned);
    println!("  Average Accuracy: {:.1}%", metrics.average_accuracy);

    Ok(())
}

fn example_optimization_prediction() -> Result<()> {
    // Create and train model
    let mut optimizer = MlOptimizer::new();

    let training_data = vec![
        TrainingExample {
            features: CodeFeatures {
                lines_of_code: 50,
                cyclomatic_complexity: 5,
                function_count: 3,
                loop_count: 4,
                recursion_depth: 0,
                memory_allocations: 2,
                io_operations: 0,
                dependencies_count: 5,
            },
            strategy: OptimizationStrategy::LoopUnrolling,
            speedup: 1.9,
            success: true,
        },
        TrainingExample {
            features: CodeFeatures {
                lines_of_code: 200,
                cyclomatic_complexity: 15,
                function_count: 12,
                loop_count: 1,
                recursion_depth: 0,
                memory_allocations: 15,
                io_operations: 3,
                dependencies_count: 10,
            },
            strategy: OptimizationStrategy::MemoryPooling,
            speedup: 2.3,
            success: true,
        },
        TrainingExample {
            features: CodeFeatures {
                lines_of_code: 100,
                cyclomatic_complexity: 8,
                function_count: 5,
                loop_count: 3,
                recursion_depth: 0,
                memory_allocations: 5,
                io_operations: 1,
                dependencies_count: 8,
            },
            strategy: OptimizationStrategy::Parallelization,
            speedup: 3.2,
            success: true,
        },
    ];

    optimizer.train(training_data)?;

    // Make predictions
    let test_features = CodeFeatures {
        lines_of_code: 80,
        cyclomatic_complexity: 6,
        function_count: 4,
        loop_count: 5,
        recursion_depth: 0,
        memory_allocations: 3,
        io_operations: 0,
        dependencies_count: 6,
    };

    let predictions = optimizer.predict(&test_features);

    println!("Top 3 Optimization Recommendations:");
    for (i, pred) in predictions.iter().take(3).enumerate() {
        println!("\n{}. {:?}", i + 1, pred.strategy);
        println!("   Confidence: {:.1}%", pred.confidence * 100.0);
        println!("   Estimated Speedup: {:.2}x", pred.estimated_speedup);
        println!("   Reasoning:");
        for reason in &pred.reasoning {
            println!("     - {}", reason);
        }
    }

    Ok(())
}

fn example_transfer_learning() -> Result<()> {
    // Train on source domain (web applications)
    let mut source_optimizer = MlOptimizer::new();

    let web_app_data = vec![
        TrainingExample {
            features: CodeFeatures {
                lines_of_code: 150,
                cyclomatic_complexity: 12,
                function_count: 8,
                loop_count: 2,
                recursion_depth: 0,
                memory_allocations: 10,
                io_operations: 15,
                dependencies_count: 20,
            },
            strategy: OptimizationStrategy::CacheOptimization,
            speedup: 2.5,
            success: true,
        },
    ];

    source_optimizer.train(web_app_data)?;
    println!("Source model trained on web applications");

    // Adapt to target domain (data processing)
    let mut transfer_learner = TransferLearner::new(
        source_optimizer,
        "data-processing".to_string(),
    );

    let data_processing_examples = vec![
        TrainingExample {
            features: CodeFeatures {
                lines_of_code: 120,
                cyclomatic_complexity: 8,
                function_count: 6,
                loop_count: 6,
                recursion_depth: 0,
                memory_allocations: 8,
                io_operations: 2,
                dependencies_count: 12,
            },
            strategy: OptimizationStrategy::Parallelization,
            speedup: 3.8,
            success: true,
        },
    ];

    let adaptation_metrics = transfer_learner.adapt(data_processing_examples)?;

    println!("\nTransfer Learning Results:");
    println!("  Target Domain: {}", adaptation_metrics.domain);
    println!("  Initial Accuracy: {:.1}%", adaptation_metrics.initial_accuracy);
    println!("  Final Accuracy: {:.1}%", adaptation_metrics.final_accuracy);
    println!("  Improvement: {:.1}%", adaptation_metrics.improvement);
    println!("  Examples Used: {}", adaptation_metrics.examples_used);

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_extraction() {
        let code = "fn test() { for i in 0..10 { } }";
        let features = FeatureExtractor::extract(code);

        assert_eq!(features.function_count, 1);
        assert_eq!(features.loop_count, 1);
        assert!(features.lines_of_code > 0);
    }

    #[test]
    fn test_ml_optimizer_creation() {
        let optimizer = MlOptimizer::new();
        assert_eq!(optimizer.training_data.len(), 0);
        assert_eq!(optimizer.strategy_scores.len(), 0);
    }

    #[test]
    fn test_ml_optimizer_training() {
        let mut optimizer = MlOptimizer::new();

        let example = TrainingExample {
            features: CodeFeatures {
                lines_of_code: 50,
                cyclomatic_complexity: 5,
                function_count: 2,
                loop_count: 3,
                recursion_depth: 0,
                memory_allocations: 1,
                io_operations: 0,
                dependencies_count: 5,
            },
            strategy: OptimizationStrategy::LoopUnrolling,
            speedup: 1.5,
            success: true,
        };

        let result = optimizer.train(vec![example]);
        assert!(result.is_ok());

        let metrics = result.unwrap();
        assert_eq!(metrics.examples_processed, 1);
        assert_eq!(metrics.strategies_learned, 1);
    }

    #[test]
    fn test_prediction() {
        let mut optimizer = MlOptimizer::new();

        let training = vec![
            TrainingExample {
                features: CodeFeatures {
                    lines_of_code: 100,
                    cyclomatic_complexity: 10,
                    function_count: 5,
                    loop_count: 5,
                    recursion_depth: 0,
                    memory_allocations: 3,
                    io_operations: 0,
                    dependencies_count: 8,
                },
                strategy: OptimizationStrategy::LoopUnrolling,
                speedup: 2.0,
                success: true,
            },
        ];

        optimizer.train(training).unwrap();

        let features = CodeFeatures {
            lines_of_code: 90,
            cyclomatic_complexity: 8,
            function_count: 4,
            loop_count: 4,
            recursion_depth: 0,
            memory_allocations: 2,
            io_operations: 0,
            dependencies_count: 7,
        };

        let predictions = optimizer.predict(&features);
        assert!(!predictions.is_empty());
        assert!(predictions[0].confidence > 0.0);
    }

    #[test]
    fn test_recommendation() {
        let mut optimizer = MlOptimizer::new();

        let training = vec![
            TrainingExample {
                features: CodeFeatures {
                    lines_of_code: 50,
                    cyclomatic_complexity: 5,
                    function_count: 2,
                    loop_count: 3,
                    recursion_depth: 0,
                    memory_allocations: 1,
                    io_operations: 0,
                    dependencies_count: 5,
                },
                strategy: OptimizationStrategy::LoopUnrolling,
                speedup: 1.5,
                success: true,
            },
        ];

        optimizer.train(training).unwrap();

        let features = CodeFeatures {
            lines_of_code: 45,
            cyclomatic_complexity: 4,
            function_count: 2,
            loop_count: 2,
            recursion_depth: 0,
            memory_allocations: 1,
            io_operations: 0,
            dependencies_count: 4,
        };

        let recommendation = optimizer.recommend(&features);
        assert!(recommendation.confidence > 0.0);
        assert!(recommendation.estimated_speedup >= 1.0);
    }

    #[test]
    fn test_evaluation_metrics() {
        let mut optimizer = MlOptimizer::new();

        let training = vec![
            TrainingExample {
                features: CodeFeatures {
                    lines_of_code: 100,
                    cyclomatic_complexity: 10,
                    function_count: 5,
                    loop_count: 5,
                    recursion_depth: 0,
                    memory_allocations: 3,
                    io_operations: 0,
                    dependencies_count: 8,
                },
                strategy: OptimizationStrategy::LoopUnrolling,
                speedup: 2.0,
                success: true,
            },
        ];

        optimizer.train(training.clone()).unwrap();

        let metrics = optimizer.evaluate(&training);
        assert!(metrics.accuracy >= 0.0 && metrics.accuracy <= 100.0);
        assert_eq!(metrics.total_predictions, 1);
    }

    #[test]
    fn test_feature_score_loop_unrolling() {
        let optimizer = MlOptimizer::new();

        let features = CodeFeatures {
            lines_of_code: 50,
            cyclomatic_complexity: 5,
            function_count: 2,
            loop_count: 5,
            recursion_depth: 0,
            memory_allocations: 1,
            io_operations: 0,
            dependencies_count: 5,
        };

        let score = optimizer.calculate_feature_score(&features, OptimizationStrategy::LoopUnrolling);
        assert!(score > 0.0);
    }

    #[test]
    fn test_feature_score_inlining() {
        let optimizer = MlOptimizer::new();

        let features = CodeFeatures {
            lines_of_code: 200,
            cyclomatic_complexity: 10,
            function_count: 15,
            loop_count: 2,
            recursion_depth: 0,
            memory_allocations: 5,
            io_operations: 1,
            dependencies_count: 10,
        };

        let score = optimizer.calculate_feature_score(&features, OptimizationStrategy::Inlining);
        assert!(score > 0.0);
    }

    #[test]
    fn test_transfer_learner_creation() {
        let source_model = MlOptimizer::new();
        let learner = TransferLearner::new(source_model, "target-domain".to_string());

        assert_eq!(learner.target_domain, "target-domain");
    }

    #[test]
    fn test_transfer_learning_adaptation() {
        let mut source_model = MlOptimizer::new();

        let source_data = vec![
            TrainingExample {
                features: CodeFeatures {
                    lines_of_code: 100,
                    cyclomatic_complexity: 10,
                    function_count: 5,
                    loop_count: 3,
                    recursion_depth: 0,
                    memory_allocations: 5,
                    io_operations: 5,
                    dependencies_count: 10,
                },
                strategy: OptimizationStrategy::CacheOptimization,
                speedup: 2.0,
                success: true,
            },
        ];

        source_model.train(source_data).unwrap();

        let mut learner = TransferLearner::new(source_model, "new-domain".to_string());

        let target_data = vec![
            TrainingExample {
                features: CodeFeatures {
                    lines_of_code: 120,
                    cyclomatic_complexity: 8,
                    function_count: 6,
                    loop_count: 4,
                    recursion_depth: 0,
                    memory_allocations: 4,
                    io_operations: 2,
                    dependencies_count: 8,
                },
                strategy: OptimizationStrategy::Parallelization,
                speedup: 3.0,
                success: true,
            },
        ];

        let result = learner.adapt(target_data);
        assert!(result.is_ok());

        let metrics = result.unwrap();
        assert_eq!(metrics.domain, "new-domain");
        assert_eq!(metrics.examples_used, 1);
    }

    #[test]
    fn test_speedup_estimation() {
        let mut optimizer = MlOptimizer::new();

        optimizer.strategy_scores.insert(OptimizationStrategy::LoopUnrolling, 2.0);

        let features = CodeFeatures {
            lines_of_code: 50,
            cyclomatic_complexity: 5,
            function_count: 2,
            loop_count: 5,
            recursion_depth: 0,
            memory_allocations: 1,
            io_operations: 0,
            dependencies_count: 5,
        };

        let speedup = optimizer.estimate_speedup(&features, OptimizationStrategy::LoopUnrolling);
        assert!(speedup >= 1.0);
        assert!(speedup <= 10.0);
    }

    #[test]
    fn test_reasoning_generation() {
        let optimizer = MlOptimizer::new();

        let features = CodeFeatures {
            lines_of_code: 50,
            cyclomatic_complexity: 5,
            function_count: 2,
            loop_count: 8,
            recursion_depth: 0,
            memory_allocations: 1,
            io_operations: 0,
            dependencies_count: 5,
        };

        let reasoning = optimizer.generate_reasoning(&features, OptimizationStrategy::LoopUnrolling);
        assert!(!reasoning.is_empty());
    }

    #[test]
    fn test_accuracy_calculation() {
        let mut optimizer = MlOptimizer::new();
        assert_eq!(optimizer.calculate_accuracy(), 0.0);

        optimizer.training_data.push(TrainingExample {
            features: CodeFeatures {
                lines_of_code: 50,
                cyclomatic_complexity: 5,
                function_count: 2,
                loop_count: 3,
                recursion_depth: 0,
                memory_allocations: 1,
                io_operations: 0,
                dependencies_count: 5,
            },
            strategy: OptimizationStrategy::LoopUnrolling,
            speedup: 1.5,
            success: true,
        });

        assert_eq!(optimizer.calculate_accuracy(), 100.0);
    }

    #[test]
    fn test_feature_weights_default() {
        let weights = FeatureWeights::default();
        assert_eq!(weights.complexity_weight, 1.0);
        assert_eq!(weights.loop_weight, 1.5);
        assert_eq!(weights.memory_weight, 1.2);
        assert_eq!(weights.io_weight, 0.8);
    }
}
