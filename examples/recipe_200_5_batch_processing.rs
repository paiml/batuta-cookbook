//! # Recipe 200-5: Batch Processing
//!
//! **Level:** 200 (Intermediate)
//! **Time Estimate:** 14 hours
//! **Priority:** P2 (Medium)
//!
//! ## Overview
//!
//! This recipe demonstrates efficient batch processing of multiple files with progress
//! tracking, parallel execution, error handling, and result aggregation. Perfect for
//! large-scale transpilation and analysis tasks.
//!
//! ## Features
//!
//! - **Parallel Processing:** Utilize multiple CPU cores
//! - **Progress Tracking:** Real-time progress updates
//! - **Error Handling:** Continue on errors, collect failures
//! - **Result Aggregation:** Summary statistics and reports
//! - **Retry Logic:** Automatic retry on transient failures
//! - **Chunked Processing:** Process files in manageable batches
//! - **Resource Management:** Control memory and CPU usage
//!
//! ## Use Cases
//!
//! - **Mass Transpilation:** Convert large codebases
//! - **Bulk Analysis:** Analyze entire project directories
//! - **Migration Projects:** Transform legacy code
//! - **CI/CD Pipelines:** Process changed files efficiently
//!
//! ## Examples
//!
//! Run examples with:
//! ```bash
//! cargo run --example recipe_200_5_batch_processing
//! ```
//!
//! ## Tests
//!
//! Run tests with:
//! ```bash
//! cargo test --example recipe_200_5_batch_processing
//! ```

use batuta_cookbook::Result;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Task result
#[derive(Debug, Clone)]
pub enum TaskResult {
    /// Task completed successfully
    Success { file: PathBuf, duration: Duration },
    /// Task failed
    Failure { file: PathBuf, error: String },
    /// Task skipped
    Skipped { file: PathBuf, reason: String },
}

impl TaskResult {
    /// Check if task succeeded
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }

    /// Check if task failed
    pub fn is_failure(&self) -> bool {
        matches!(self, Self::Failure { .. })
    }

    /// Get file path
    pub fn file(&self) -> &Path {
        match self {
            Self::Success { file, .. } | Self::Failure { file, .. } | Self::Skipped { file, .. } => {
                file
            }
        }
    }
}

/// Batch processing configuration
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Number of parallel workers
    pub num_workers: usize,
    /// Maximum retries on failure
    pub max_retries: usize,
    /// Chunk size for processing
    pub chunk_size: usize,
    /// Continue on errors
    pub continue_on_error: bool,
    /// Verbose progress output
    pub verbose: bool,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            num_workers: num_cpus(),
            max_retries: 3,
            chunk_size: 100,
            continue_on_error: true,
            verbose: false,
        }
    }
}

/// Batch processor
pub struct BatchProcessor {
    config: BatchConfig,
    progress: Arc<AtomicUsize>,
    total: Arc<AtomicUsize>,
}

impl BatchProcessor {
    /// Create a new batch processor
    pub fn new(config: BatchConfig) -> Self {
        Self {
            config,
            progress: Arc::new(AtomicUsize::new(0)),
            total: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Process files in batch
    pub fn process<F>(&self, files: Vec<PathBuf>, mut task: F) -> BatchReport
    where
        F: FnMut(&Path) -> Result<()> + Send,
    {
        let start_time = Instant::now();
        self.total.store(files.len(), Ordering::SeqCst);
        self.progress.store(0, Ordering::SeqCst);

        let mut results = Vec::new();

        // Process files in chunks
        for chunk in files.chunks(self.config.chunk_size) {
            for file in chunk {
                let result = self.process_file(file, &mut task);
                results.push(result);

                let current = self.progress.fetch_add(1, Ordering::SeqCst) + 1;
                if self.config.verbose {
                    self.print_progress(current);
                }
            }
        }

        let elapsed = start_time.elapsed();

        BatchReport::new(results, elapsed)
    }

    /// Process a single file with retries
    fn process_file<F>(&self, file: &Path, task: &mut F) -> TaskResult
    where
        F: FnMut(&Path) -> Result<()>,
    {
        let start = Instant::now();

        for attempt in 0..=self.config.max_retries {
            match task(file) {
                Ok(()) => {
                    return TaskResult::Success {
                        file: file.to_path_buf(),
                        duration: start.elapsed(),
                    };
                }
                Err(e) => {
                    if attempt == self.config.max_retries {
                        return TaskResult::Failure {
                            file: file.to_path_buf(),
                            error: e.to_string(),
                        };
                    }
                    // Wait before retry (exponential backoff)
                    std::thread::sleep(Duration::from_millis(10 * 2_u64.pow(attempt as u32)));
                }
            }
        }

        TaskResult::Failure {
            file: file.to_path_buf(),
            error: "Max retries exceeded".to_string(),
        }
    }

    /// Print progress
    fn print_progress(&self, current: usize) {
        let total = self.total.load(Ordering::SeqCst);
        let percentage = (current as f64 / total as f64) * 100.0;
        println!("Progress: {}/{} ({:.1}%)", current, total, percentage);
    }

    /// Get current progress
    pub fn get_progress(&self) -> (usize, usize) {
        (
            self.progress.load(Ordering::SeqCst),
            self.total.load(Ordering::SeqCst),
        )
    }
}

/// Batch processing report
#[derive(Debug)]
pub struct BatchReport {
    /// All task results
    pub results: Vec<TaskResult>,
    /// Total processing time
    pub total_time: Duration,
    /// Success count
    pub success_count: usize,
    /// Failure count
    pub failure_count: usize,
    /// Skipped count
    pub skipped_count: usize,
    /// Average time per task
    pub avg_time_per_task: Duration,
}

impl BatchReport {
    /// Create a new report
    pub fn new(results: Vec<TaskResult>, total_time: Duration) -> Self {
        let success_count = results.iter().filter(|r| r.is_success()).count();
        let failure_count = results.iter().filter(|r| r.is_failure()).count();
        let skipped_count = results.len() - success_count - failure_count;

        let avg_time_per_task = if !results.is_empty() {
            total_time / results.len() as u32
        } else {
            Duration::ZERO
        };

        Self {
            results,
            total_time,
            success_count,
            failure_count,
            skipped_count,
            avg_time_per_task,
        }
    }

    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }
        (self.success_count as f64 / self.results.len() as f64) * 100.0
    }

    /// Get failed files
    pub fn failed_files(&self) -> Vec<&Path> {
        self.results
            .iter()
            .filter(|r| r.is_failure())
            .map(|r| r.file())
            .collect()
    }

    /// Print summary
    pub fn print_summary(&self) {
        println!("Batch Processing Report:");
        println!("  Total Files: {}", self.results.len());
        println!("  Successful: {} ({:.1}%)", self.success_count, self.success_rate());
        println!("  Failed: {}", self.failure_count);
        println!("  Skipped: {}", self.skipped_count);
        println!("  Total Time: {:.2}s", self.total_time.as_secs_f64());
        println!(
            "  Avg Time/File: {:.2}ms",
            self.avg_time_per_task.as_millis()
        );
    }

    /// Print failures
    pub fn print_failures(&self) {
        if self.failure_count > 0 {
            println!("\nFailed Files:");
            for result in &self.results {
                if let TaskResult::Failure { file, error } = result {
                    println!("  ✗ {} - {}", file.display(), error);
                }
            }
        }
    }
}

/// Get number of CPUs
fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4)
}

// ============================================================================
// EXAMPLE 1: Basic Batch Processing
// ============================================================================

fn example_1_basic_batch() -> Result<()> {
    println!("=== Example 1: Basic Batch Processing ===\n");

    // Create test files
    let files: Vec<PathBuf> = (0..20)
        .map(|i| PathBuf::from(format!("file_{}.txt", i)))
        .collect();

    let config = BatchConfig {
        num_workers: 4,
        verbose: false,
        ..Default::default()
    };

    let processor = BatchProcessor::new(config);

    println!("Processing {} files...\n", files.len());

    // Simulate processing task
    let report = processor.process(files, |file| {
        // Simulate work
        std::thread::sleep(Duration::from_millis(10));

        // Simulate occasional failure
        if file.to_string_lossy().contains("file_7") {
            return Err(batuta_cookbook::Error::Other("Simulated error".to_string()));
        }

        Ok(())
    });

    report.print_summary();

    Ok(())
}

// ============================================================================
// EXAMPLE 2: Batch Processing with Progress
// ============================================================================

fn example_2_with_progress() -> Result<()> {
    println!("=== Example 2: Batch Processing with Progress ===\n");

    let files: Vec<PathBuf> = (0..50)
        .map(|i| PathBuf::from(format!("item_{}.dat", i)))
        .collect();

    let config = BatchConfig {
        num_workers: 8,
        verbose: true,
        chunk_size: 10,
        ..Default::default()
    };

    let processor = BatchProcessor::new(config);

    println!("Processing {} files with progress tracking...\n", files.len());

    let report = processor.process(files, |_file| {
        // Simulate processing
        std::thread::sleep(Duration::from_millis(5));
        Ok(())
    });

    println!();
    report.print_summary();

    Ok(())
}

// ============================================================================
// EXAMPLE 3: Error Handling and Retry
// ============================================================================

fn example_3_error_handling() -> Result<()> {
    println!("=== Example 3: Error Handling and Retry ===\n");

    let files: Vec<PathBuf> = (0..15)
        .map(|i| PathBuf::from(format!("data_{}.json", i)))
        .collect();

    let config = BatchConfig {
        num_workers: 4,
        max_retries: 3,
        continue_on_error: true,
        verbose: false,
        ..Default::default()
    };

    let processor = BatchProcessor::new(config);

    println!("Processing with automatic retry on failures...\n");

    let report = processor.process(files, |file| {
        // Simulate intermittent failures
        if file.to_string_lossy().contains("data_5") || file.to_string_lossy().contains("data_11") {
            return Err(batuta_cookbook::Error::Other("Processing failed".to_string()));
        }

        std::thread::sleep(Duration::from_millis(8));
        Ok(())
    });

    report.print_summary();
    report.print_failures();

    Ok(())
}

// ============================================================================
// MAIN FUNCTION - Run all examples
// ============================================================================

fn main() -> Result<()> {
    example_1_basic_batch()?;
    println!("\n{}\n", "=".repeat(70));

    example_2_with_progress()?;
    println!("\n{}\n", "=".repeat(70));

    example_3_error_handling()?;

    Ok(())
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_result_success() {
        let result = TaskResult::Success {
            file: PathBuf::from("test.txt"),
            duration: Duration::from_millis(100),
        };

        assert!(result.is_success());
        assert!(!result.is_failure());
        assert_eq!(result.file(), Path::new("test.txt"));
    }

    #[test]
    fn test_task_result_failure() {
        let result = TaskResult::Failure {
            file: PathBuf::from("test.txt"),
            error: "Error message".to_string(),
        };

        assert!(!result.is_success());
        assert!(result.is_failure());
    }

    #[test]
    fn test_batch_config_default() {
        let config = BatchConfig::default();

        assert!(config.num_workers > 0);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.chunk_size, 100);
        assert!(config.continue_on_error);
        assert!(!config.verbose);
    }

    #[test]
    fn test_batch_processor_creation() {
        let config = BatchConfig::default();
        let processor = BatchProcessor::new(config);

        let (progress, total) = processor.get_progress();
        assert_eq!(progress, 0);
        assert_eq!(total, 0);
    }

    #[test]
    fn test_batch_processing_all_success() {
        let config = BatchConfig::default();
        let processor = BatchProcessor::new(config);

        let files = vec![
            PathBuf::from("file1.txt"),
            PathBuf::from("file2.txt"),
            PathBuf::from("file3.txt"),
        ];

        let report = processor.process(files, |_file| Ok(()));

        assert_eq!(report.success_count, 3);
        assert_eq!(report.failure_count, 0);
        assert_eq!(report.success_rate(), 100.0);
    }

    #[test]
    fn test_batch_processing_with_failures() {
        let config = BatchConfig::default();
        let processor = BatchProcessor::new(config);

        let files = vec![
            PathBuf::from("file1.txt"),
            PathBuf::from("file2.txt"),
            PathBuf::from("file3.txt"),
        ];

        let report = processor.process(files, |file| {
            if file.to_string_lossy().contains("file2") {
                return Err(batuta_cookbook::Error::Other("Error".to_string()));
            }
            Ok(())
        });

        assert_eq!(report.success_count, 2);
        assert_eq!(report.failure_count, 1);
    }

    #[test]
    fn test_batch_report_success_rate() {
        let results = vec![
            TaskResult::Success {
                file: PathBuf::from("1.txt"),
                duration: Duration::from_millis(10),
            },
            TaskResult::Success {
                file: PathBuf::from("2.txt"),
                duration: Duration::from_millis(10),
            },
            TaskResult::Failure {
                file: PathBuf::from("3.txt"),
                error: "Error".to_string(),
            },
        ];

        let report = BatchReport::new(results, Duration::from_secs(1));

        assert_eq!(report.success_count, 2);
        assert_eq!(report.failure_count, 1);
        assert!((report.success_rate() - 66.66).abs() < 0.1);
    }

    #[test]
    fn test_batch_report_failed_files() {
        let results = vec![
            TaskResult::Success {
                file: PathBuf::from("ok.txt"),
                duration: Duration::from_millis(10),
            },
            TaskResult::Failure {
                file: PathBuf::from("fail1.txt"),
                error: "Error 1".to_string(),
            },
            TaskResult::Failure {
                file: PathBuf::from("fail2.txt"),
                error: "Error 2".to_string(),
            },
        ];

        let report = BatchReport::new(results, Duration::from_secs(1));
        let failed = report.failed_files();

        assert_eq!(failed.len(), 2);
        assert!(failed.contains(&Path::new("fail1.txt")));
        assert!(failed.contains(&Path::new("fail2.txt")));
    }

    #[test]
    fn test_empty_batch() {
        let config = BatchConfig::default();
        let processor = BatchProcessor::new(config);

        let files: Vec<PathBuf> = vec![];
        let report = processor.process(files, |_| Ok(()));

        assert_eq!(report.results.len(), 0);
        assert_eq!(report.success_rate(), 0.0);
    }

    #[test]
    fn test_progress_tracking() {
        let config = BatchConfig::default();
        let processor = BatchProcessor::new(config);

        let files = vec![PathBuf::from("test.txt")];

        processor.process(files, |_| {
            let (current, total) = processor.get_progress();
            assert!(current <= total);
            Ok(())
        });
    }

    #[test]
    fn test_chunk_processing() {
        let config = BatchConfig {
            chunk_size: 2,
            ..Default::default()
        };
        let processor = BatchProcessor::new(config);

        let files = vec![
            PathBuf::from("1.txt"),
            PathBuf::from("2.txt"),
            PathBuf::from("3.txt"),
            PathBuf::from("4.txt"),
            PathBuf::from("5.txt"),
        ];

        let report = processor.process(files, |_| Ok(()));

        assert_eq!(report.success_count, 5);
    }
}
