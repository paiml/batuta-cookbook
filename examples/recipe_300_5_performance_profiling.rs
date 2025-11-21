//! # RECIPE-300-5: Performance Profiling
//!
//! **Level:** 300 (Advanced)
//! **Estimated Time:** 22 hours
//! **Prerequisites:** RECIPE-200-4 (Optimization Profiles), RECIPE-300-1 (GPU Acceleration)
//!
//! ## Learning Objectives
//! - Measure execution time and performance metrics
//! - Track memory usage and allocations
//! - Identify performance bottlenecks
//! - Generate profiling reports
//! - Compare performance across implementations
//!
//! ## Concepts Covered
//! - Time measurement and benchmarking
//! - Performance counters and metrics
//! - Memory profiling concepts
//! - Statistical analysis of performance data
//! - Visualization and reporting
//!
//! ## Examples
//! This file demonstrates three approaches:
//! 1. Basic timing and performance measurement
//! 2. Memory usage tracking and profiling
//! 3. Comprehensive performance report generation

use batuta_cookbook::Result;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Performance metric types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetricType {
    /// Execution time measurement
    ExecutionTime,
    /// Memory usage
    MemoryUsage,
    /// Throughput (operations per second)
    Throughput,
    /// CPU utilization
    CpuUsage,
    /// Cache hit rate
    CacheHitRate,
}

impl MetricType {
    pub fn unit(&self) -> &str {
        match self {
            Self::ExecutionTime => "ms",
            Self::MemoryUsage => "MB",
            Self::Throughput => "ops/sec",
            Self::CpuUsage => "%",
            Self::CacheHitRate => "%",
        }
    }
}

/// Performance measurement sample
#[derive(Debug, Clone)]
pub struct PerformanceSample {
    pub metric_type: MetricType,
    pub value: f64,
    pub timestamp: Instant,
}

impl PerformanceSample {
    pub fn new(metric_type: MetricType, value: f64) -> Self {
        Self {
            metric_type,
            value,
            timestamp: Instant::now(),
        }
    }
}

/// Statistical summary of measurements
#[derive(Debug, Clone)]
pub struct Statistics {
    pub count: usize,
    pub mean: f64,
    pub median: f64,
    pub min: f64,
    pub max: f64,
    pub std_dev: f64,
    pub percentile_95: f64,
    pub percentile_99: f64,
}

impl Statistics {
    pub fn from_values(mut values: Vec<f64>) -> Self {
        if values.is_empty() {
            return Self::default();
        }

        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let count = values.len();
        let sum: f64 = values.iter().sum();
        let mean = sum / count as f64;

        let median = if count % 2 == 0 {
            (values[count / 2 - 1] + values[count / 2]) / 2.0
        } else {
            values[count / 2]
        };

        let min = *values.first().unwrap();
        let max = *values.last().unwrap();

        let variance: f64 = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / count as f64;
        let std_dev = variance.sqrt();

        let p95_idx = ((count as f64) * 0.95) as usize;
        let p99_idx = ((count as f64) * 0.99) as usize;

        Self {
            count,
            mean,
            median,
            min,
            max,
            std_dev,
            percentile_95: values[p95_idx.min(count - 1)],
            percentile_99: values[p99_idx.min(count - 1)],
        }
    }
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            median: 0.0,
            min: 0.0,
            max: 0.0,
            std_dev: 0.0,
            percentile_95: 0.0,
            percentile_99: 0.0,
        }
    }
}

/// Performance profiler for tracking metrics
pub struct Profiler {
    samples: HashMap<MetricType, Vec<PerformanceSample>>,
    start_time: Instant,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            samples: HashMap::new(),
            start_time: Instant::now(),
        }
    }

    /// Record a performance sample
    pub fn record(&mut self, metric_type: MetricType, value: f64) {
        let sample = PerformanceSample::new(metric_type, value);
        self.samples.entry(metric_type).or_default().push(sample);
    }

    /// Get statistics for a metric type
    pub fn get_statistics(&self, metric_type: MetricType) -> Option<Statistics> {
        self.samples.get(&metric_type).map(|samples| {
            let values: Vec<f64> = samples.iter().map(|s| s.value).collect();
            Statistics::from_values(values)
        })
    }

    /// Get all recorded metrics
    pub fn get_all_statistics(&self) -> HashMap<MetricType, Statistics> {
        let mut result = HashMap::new();
        for (metric_type, samples) in &self.samples {
            let values: Vec<f64> = samples.iter().map(|s| s.value).collect();
            result.insert(*metric_type, Statistics::from_values(values));
        }
        result
    }

    /// Total profiling duration
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Clear all samples
    pub fn reset(&mut self) {
        self.samples.clear();
        self.start_time = Instant::now();
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Timed scope for automatic timing measurements
pub struct TimedScope<'a> {
    profiler: &'a mut Profiler,
    _name: String,
    start: Instant,
}

impl<'a> TimedScope<'a> {
    pub fn new(profiler: &'a mut Profiler, name: String) -> Self {
        Self {
            profiler,
            _name: name,
            start: Instant::now(),
        }
    }
}

impl<'a> Drop for TimedScope<'a> {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        self.profiler.record(MetricType::ExecutionTime, elapsed.as_secs_f64() * 1000.0);
    }
}

/// Memory tracker for allocation tracking
#[derive(Debug, Clone)]
pub struct MemoryTracker {
    allocations: Vec<usize>,
    deallocations: Vec<usize>,
    peak_usage: usize,
    current_usage: usize,
}

impl MemoryTracker {
    pub fn new() -> Self {
        Self {
            allocations: Vec::new(),
            deallocations: Vec::new(),
            peak_usage: 0,
            current_usage: 0,
        }
    }

    /// Record an allocation
    pub fn allocate(&mut self, size: usize) {
        self.allocations.push(size);
        self.current_usage += size;
        self.peak_usage = self.peak_usage.max(self.current_usage);
    }

    /// Record a deallocation
    pub fn deallocate(&mut self, size: usize) {
        self.deallocations.push(size);
        self.current_usage = self.current_usage.saturating_sub(size);
    }

    /// Get total allocated bytes
    pub fn total_allocated(&self) -> usize {
        self.allocations.iter().sum()
    }

    /// Get total deallocated bytes
    pub fn total_deallocated(&self) -> usize {
        self.deallocations.iter().sum()
    }

    /// Get peak memory usage
    pub fn peak_usage(&self) -> usize {
        self.peak_usage
    }

    /// Get current memory usage
    pub fn current_usage(&self) -> usize {
        self.current_usage
    }

    /// Get average allocation size
    pub fn avg_allocation_size(&self) -> f64 {
        if self.allocations.is_empty() {
            0.0
        } else {
            self.total_allocated() as f64 / self.allocations.len() as f64
        }
    }
}

impl Default for MemoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance report generator
pub struct PerformanceReport {
    pub profiler_stats: HashMap<MetricType, Statistics>,
    pub memory_stats: Option<MemoryStats>,
    pub total_duration: Duration,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_allocated: usize,
    pub peak_usage: usize,
    pub avg_allocation_size: f64,
}

impl PerformanceReport {
    pub fn generate(profiler: &Profiler, memory: Option<&MemoryTracker>) -> Self {
        let profiler_stats = profiler.get_all_statistics();
        let memory_stats = memory.map(|m| MemoryStats {
            total_allocated: m.total_allocated(),
            peak_usage: m.peak_usage(),
            avg_allocation_size: m.avg_allocation_size(),
        });

        let mut recommendations = Vec::new();

        // Analyze execution time
        if let Some(time_stats) = profiler_stats.get(&MetricType::ExecutionTime) {
            if time_stats.std_dev > time_stats.mean * 0.5 {
                recommendations
                    .push("High variance in execution time - investigate inconsistent performance".to_string());
            }
            if time_stats.percentile_99 > time_stats.mean * 3.0 {
                recommendations.push(
                    "P99 latency significantly higher than mean - possible outliers".to_string(),
                );
            }
        }

        // Analyze memory usage
        if let Some(mem) = &memory_stats {
            if mem.peak_usage > mem.total_allocated / 2 {
                recommendations
                    .push("High peak memory usage - consider streaming or chunking".to_string());
            }
        }

        Self {
            profiler_stats,
            memory_stats,
            total_duration: profiler.elapsed(),
            recommendations,
        }
    }

    pub fn print_summary(&self) {
        println!("=== Performance Report ===\n");
        println!("Total Duration: {:?}", self.total_duration);

        for (metric_type, stats) in &self.profiler_stats {
            println!("\n{:?} ({})", metric_type, metric_type.unit());
            println!("  Count: {}", stats.count);
            println!("  Mean: {:.2}", stats.mean);
            println!("  Median: {:.2}", stats.median);
            println!("  Min: {:.2}", stats.min);
            println!("  Max: {:.2}", stats.max);
            println!("  Std Dev: {:.2}", stats.std_dev);
            println!("  P95: {:.2}", stats.percentile_95);
            println!("  P99: {:.2}", stats.percentile_99);
        }

        if let Some(mem) = &self.memory_stats {
            println!("\nMemory Usage:");
            println!("  Total Allocated: {:.2} MB", mem.total_allocated as f64 / 1_048_576.0);
            println!("  Peak Usage: {:.2} MB", mem.peak_usage as f64 / 1_048_576.0);
            println!("  Avg Allocation: {:.2} KB", mem.avg_allocation_size / 1024.0);
        }

        if !self.recommendations.is_empty() {
            println!("\nRecommendations:");
            for (i, rec) in self.recommendations.iter().enumerate() {
                println!("  {}. {}", i + 1, rec);
            }
        }
    }
}

/// Benchmark a function multiple times
pub fn benchmark<F>(name: &str, iterations: usize, mut f: F) -> Statistics
where
    F: FnMut(),
{
    let mut durations = Vec::new();

    println!("Benchmarking: {} ({} iterations)", name, iterations);

    for _ in 0..iterations {
        let start = Instant::now();
        f();
        let elapsed = start.elapsed();
        durations.push(elapsed.as_secs_f64() * 1000.0); // Convert to ms
    }

    Statistics::from_values(durations)
}

//
// Example 1: Basic timing measurement
//
pub fn example_1_timing_measurement() -> Result<()> {
    println!("=== Example 1: Basic Timing Measurement ===\n");

    let mut profiler = Profiler::new();

    // Simulate some work and measure it
    for _ in 0..10 {
        let start = Instant::now();

        // Simulate work (sleep not used to keep tests fast)
        let mut sum = 0u64;
        for j in 0..100_000 {
            sum = sum.wrapping_add(j);
        }

        let elapsed = start.elapsed();
        profiler.record(MetricType::ExecutionTime, elapsed.as_secs_f64() * 1000.0);

        // Record fake throughput
        profiler.record(MetricType::Throughput, 100_000.0 / elapsed.as_secs_f64());
    }

    if let Some(stats) = profiler.get_statistics(MetricType::ExecutionTime) {
        println!("Execution Time Statistics:");
        println!("  Mean: {:.3} ms", stats.mean);
        println!("  Median: {:.3} ms", stats.median);
        println!("  Std Dev: {:.3} ms", stats.std_dev);
        println!("  Min/Max: {:.3} / {:.3} ms", stats.min, stats.max);
    }

    if let Some(stats) = profiler.get_statistics(MetricType::Throughput) {
        println!("\nThroughput Statistics:");
        println!("  Mean: {:.0} ops/sec", stats.mean);
        println!("  P95: {:.0} ops/sec", stats.percentile_95);
    }

    Ok(())
}

//
// Example 2: Memory tracking
//
pub fn example_2_memory_tracking() -> Result<()> {
    println!("\n=== Example 2: Memory Tracking ===\n");

    let mut tracker = MemoryTracker::new();

    // Simulate allocations
    tracker.allocate(1024 * 1024); // 1 MB
    tracker.allocate(512 * 1024); // 512 KB
    tracker.allocate(2 * 1024 * 1024); // 2 MB

    println!("After allocations:");
    println!("  Current usage: {:.2} MB", tracker.current_usage() as f64 / 1_048_576.0);
    println!("  Peak usage: {:.2} MB", tracker.peak_usage() as f64 / 1_048_576.0);

    // Deallocate
    tracker.deallocate(512 * 1024);

    println!("\nAfter deallocation:");
    println!("  Current usage: {:.2} MB", tracker.current_usage() as f64 / 1_048_576.0);
    println!("  Peak usage: {:.2} MB", tracker.peak_usage() as f64 / 1_048_576.0);
    println!("  Avg allocation: {:.2} KB", tracker.avg_allocation_size() / 1024.0);

    Ok(())
}

//
// Example 3: Comprehensive performance report
//
pub fn example_3_performance_report() -> Result<()> {
    println!("\n=== Example 3: Comprehensive Performance Report ===\n");

    let mut profiler = Profiler::new();
    let mut memory = MemoryTracker::new();

    // Run multiple iterations with profiling
    for _ in 0..20 {
        let start = Instant::now();

        // Simulate memory allocations
        memory.allocate(100 * 1024); // 100 KB

        // Simulate work
        let mut sum = 0u64;
        for j in 0..50_000 {
            sum = sum.wrapping_add(j);
        }

        let elapsed = start.elapsed();
        profiler.record(MetricType::ExecutionTime, elapsed.as_secs_f64() * 1000.0);
        profiler.record(MetricType::Throughput, 50_000.0 / elapsed.as_secs_f64());
    }

    // Generate and display report
    let report = PerformanceReport::generate(&profiler, Some(&memory));
    report.print_summary();

    Ok(())
}

fn main() -> Result<()> {
    example_1_timing_measurement()?;
    example_2_memory_tracking()?;
    example_3_performance_report()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_type_unit() {
        assert_eq!(MetricType::ExecutionTime.unit(), "ms");
        assert_eq!(MetricType::MemoryUsage.unit(), "MB");
        assert_eq!(MetricType::Throughput.unit(), "ops/sec");
    }

    #[test]
    fn test_profiler_creation() {
        let profiler = Profiler::new();
        assert_eq!(profiler.samples.len(), 0);
    }

    #[test]
    fn test_profiler_record() {
        let mut profiler = Profiler::new();
        profiler.record(MetricType::ExecutionTime, 10.5);
        profiler.record(MetricType::ExecutionTime, 12.3);

        let stats = profiler.get_statistics(MetricType::ExecutionTime);
        assert!(stats.is_some());
        let stats = stats.unwrap();
        assert_eq!(stats.count, 2);
    }

    #[test]
    fn test_statistics_from_values() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = Statistics::from_values(values);

        assert_eq!(stats.count, 5);
        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.median, 3.0);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
    }

    #[test]
    fn test_statistics_empty() {
        let values = vec![];
        let stats = Statistics::from_values(values);

        assert_eq!(stats.count, 0);
        assert_eq!(stats.mean, 0.0);
    }

    #[test]
    fn test_memory_tracker_allocate() {
        let mut tracker = MemoryTracker::new();
        tracker.allocate(1024);
        tracker.allocate(2048);

        assert_eq!(tracker.total_allocated(), 3072);
        assert_eq!(tracker.current_usage(), 3072);
        assert_eq!(tracker.peak_usage(), 3072);
    }

    #[test]
    fn test_memory_tracker_deallocate() {
        let mut tracker = MemoryTracker::new();
        tracker.allocate(1024);
        tracker.deallocate(512);

        assert_eq!(tracker.current_usage(), 512);
        assert_eq!(tracker.total_deallocated(), 512);
    }

    #[test]
    fn test_memory_tracker_peak() {
        let mut tracker = MemoryTracker::new();
        tracker.allocate(1000);
        tracker.allocate(2000);
        assert_eq!(tracker.peak_usage(), 3000);

        tracker.deallocate(1000);
        assert_eq!(tracker.peak_usage(), 3000); // Peak doesn't decrease
        assert_eq!(tracker.current_usage(), 2000);
    }

    #[test]
    fn test_memory_tracker_avg_allocation() {
        let mut tracker = MemoryTracker::new();
        tracker.allocate(1000);
        tracker.allocate(2000);
        tracker.allocate(3000);

        assert_eq!(tracker.avg_allocation_size(), 2000.0);
    }

    #[test]
    fn test_profiler_reset() {
        let mut profiler = Profiler::new();
        profiler.record(MetricType::ExecutionTime, 10.0);

        profiler.reset();
        assert_eq!(profiler.samples.len(), 0);
    }

    #[test]
    fn test_profiler_get_all_statistics() {
        let mut profiler = Profiler::new();
        profiler.record(MetricType::ExecutionTime, 10.0);
        profiler.record(MetricType::Throughput, 1000.0);

        let all_stats = profiler.get_all_statistics();
        assert_eq!(all_stats.len(), 2);
        assert!(all_stats.contains_key(&MetricType::ExecutionTime));
        assert!(all_stats.contains_key(&MetricType::Throughput));
    }

    #[test]
    fn test_performance_report_generation() {
        let mut profiler = Profiler::new();
        profiler.record(MetricType::ExecutionTime, 10.0);

        let report = PerformanceReport::generate(&profiler, None);
        assert_eq!(report.profiler_stats.len(), 1);
        assert!(report.memory_stats.is_none());
    }

    #[test]
    fn test_performance_report_with_memory() {
        let mut profiler = Profiler::new();
        let mut memory = MemoryTracker::new();

        profiler.record(MetricType::ExecutionTime, 10.0);
        memory.allocate(1024);

        let report = PerformanceReport::generate(&profiler, Some(&memory));
        assert!(report.memory_stats.is_some());

        let mem_stats = report.memory_stats.unwrap();
        assert_eq!(mem_stats.total_allocated, 1024);
    }

    #[test]
    fn test_benchmark_function() {
        let stats = benchmark("test", 5, || {
            let mut sum = 0u64;
            for i in 0..1000 {
                sum = sum.wrapping_add(i);
            }
        });

        assert_eq!(stats.count, 5);
        assert!(stats.mean > 0.0);
    }

    #[test]
    fn test_statistics_percentiles() {
        let values: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let stats = Statistics::from_values(values);

        assert!(stats.percentile_95 >= 90.0);
        assert!(stats.percentile_99 >= 95.0);
    }
}
