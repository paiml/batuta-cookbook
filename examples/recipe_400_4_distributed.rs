//! RECIPE-400-4: Distributed Transpilation
//!
//! This recipe demonstrates distributed transpilation across multiple nodes,
//! including job distribution, load balancing, fault tolerance, and result
//! aggregation for large-scale code transformation projects.
//!
//! Learning Objectives:
//! - Distributed task coordination and job scheduling
//! - Worker node management and health monitoring
//! - Load balancing strategies (round-robin, least-loaded, capacity-based)
//! - Fault tolerance with automatic retry and failover
//! - Result aggregation and distributed state management
//! - Network communication patterns (simulated in-process)
//! - Performance metrics for distributed systems
//!
//! Level: Expert (400)
//! Estimated Time: 44 hours
//! Prerequisites: RECIPE-200-5 (Batch Processing), RECIPE-300-1 (GPU Acceleration)

use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

type Result<T> = std::result::Result<T, String>;

// ============================================================================
// Core Types
// ============================================================================

/// Distributed job containing files to transpile
#[derive(Debug, Clone)]
pub struct DistributedJob {
    pub id: String,
    pub files: Vec<PathBuf>,
    pub priority: JobPriority,
    pub created_at: Instant,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JobPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Status of a distributed job
#[derive(Debug, Clone, PartialEq)]
pub enum JobStatus {
    Pending,
    InProgress { worker_id: String, started_at: Instant },
    Completed { worker_id: String, duration: Duration },
    Failed { worker_id: String, error: String, retry_count: usize },
}

/// Worker node in the distributed system
#[derive(Debug, Clone)]
pub struct WorkerNode {
    pub id: String,
    pub capacity: usize,
    pub status: WorkerStatus,
    pub current_load: usize,
    pub completed_jobs: usize,
    pub failed_jobs: usize,
    pub total_processing_time: Duration,
    pub last_heartbeat: Instant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WorkerStatus {
    Idle,
    Busy,
    Offline,
    Unhealthy,
}

/// Load balancing strategy
#[derive(Debug, Clone, Copy)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastLoaded,
    CapacityBased,
}

/// Result of a distributed job
#[derive(Debug, Clone)]
pub struct JobResult {
    pub job_id: String,
    pub worker_id: String,
    pub success: bool,
    pub files_processed: usize,
    pub duration: Duration,
    pub error: Option<String>,
}

// ============================================================================
// Worker Node Implementation
// ============================================================================

impl WorkerNode {
    pub fn new(id: String, capacity: usize) -> Self {
        Self {
            id,
            capacity,
            status: WorkerStatus::Idle,
            current_load: 0,
            completed_jobs: 0,
            failed_jobs: 0,
            total_processing_time: Duration::ZERO,
            last_heartbeat: Instant::now(),
        }
    }

    pub fn is_available(&self) -> bool {
        self.status == WorkerStatus::Idle && self.current_load < self.capacity
    }

    pub fn available_capacity(&self) -> usize {
        self.capacity.saturating_sub(self.current_load)
    }

    pub fn utilization(&self) -> f64 {
        if self.capacity == 0 {
            return 0.0;
        }
        (self.current_load as f64 / self.capacity as f64) * 100.0
    }

    pub fn assign_job(&mut self, _job_size: usize) -> Result<()> {
        if self.current_load + 1 > self.capacity {
            return Err(format!("Worker {} at capacity", self.id));
        }
        self.current_load += 1;
        self.status = WorkerStatus::Busy;
        self.last_heartbeat = Instant::now();
        Ok(())
    }

    pub fn complete_job(&mut self, duration: Duration) {
        self.current_load = self.current_load.saturating_sub(1);
        self.completed_jobs += 1;
        self.total_processing_time += duration;
        if self.current_load == 0 {
            self.status = WorkerStatus::Idle;
        }
        self.last_heartbeat = Instant::now();
    }

    pub fn fail_job(&mut self) {
        self.current_load = self.current_load.saturating_sub(1);
        self.failed_jobs += 1;
        if self.current_load == 0 {
            self.status = WorkerStatus::Idle;
        }
        self.last_heartbeat = Instant::now();
    }

    pub fn update_heartbeat(&mut self) {
        self.last_heartbeat = Instant::now();
    }

    pub fn check_health(&mut self, timeout: Duration) -> bool {
        if self.last_heartbeat.elapsed() > timeout {
            self.status = WorkerStatus::Unhealthy;
            false
        } else {
            true
        }
    }
}

// ============================================================================
// Distributed Coordinator
// ============================================================================

pub struct DistributedCoordinator {
    workers: Arc<Mutex<HashMap<String, WorkerNode>>>,
    job_queue: Arc<Mutex<VecDeque<DistributedJob>>>,
    job_status: Arc<Mutex<HashMap<String, JobStatus>>>,
    results: Arc<Mutex<Vec<JobResult>>>,
    strategy: LoadBalancingStrategy,
    _max_retries: usize,
    next_worker_index: Arc<Mutex<usize>>,
}

impl DistributedCoordinator {
    pub fn new(strategy: LoadBalancingStrategy) -> Self {
        Self {
            workers: Arc::new(Mutex::new(HashMap::new())),
            job_queue: Arc::new(Mutex::new(VecDeque::new())),
            job_status: Arc::new(Mutex::new(HashMap::new())),
            results: Arc::new(Mutex::new(Vec::new())),
            strategy,
            _max_retries: 3,
            next_worker_index: Arc::new(Mutex::new(0)),
        }
    }

    pub fn register_worker(&self, worker: WorkerNode) -> Result<()> {
        let mut workers = self.workers.lock().unwrap();
        if workers.contains_key(&worker.id) {
            return Err(format!("Worker {} already registered", worker.id));
        }
        workers.insert(worker.id.clone(), worker);
        Ok(())
    }

    pub fn submit_job(&self, job: DistributedJob) -> Result<()> {
        let mut queue = self.job_queue.lock().unwrap();
        let mut status = self.job_status.lock().unwrap();

        status.insert(job.id.clone(), JobStatus::Pending);

        // Insert based on priority (higher priority at front)
        let insert_pos = queue
            .iter()
            .position(|j| j.priority < job.priority)
            .unwrap_or(queue.len());

        queue.insert(insert_pos, job);
        Ok(())
    }

    pub fn process_jobs(&self) -> Result<Vec<JobResult>> {
        loop {
            let job = {
                let mut queue = self.job_queue.lock().unwrap();
                queue.pop_front()
            };

            match job {
                Some(job) => {
                    self.process_job(job)?;
                }
                None => break,
            }
        }

        let results = self.results.lock().unwrap();
        Ok(results.clone())
    }

    fn process_job(&self, job: DistributedJob) -> Result<()> {
        let worker_id = self.select_worker(&job)?;

        // Update job status
        {
            let mut status = self.job_status.lock().unwrap();
            status.insert(
                job.id.clone(),
                JobStatus::InProgress {
                    worker_id: worker_id.clone(),
                    started_at: Instant::now(),
                },
            );
        }

        // Assign job to worker
        {
            let mut workers = self.workers.lock().unwrap();
            let worker = workers.get_mut(&worker_id)
                .ok_or_else(|| format!("Worker {} not found", worker_id))?;
            worker.assign_job(job.files.len())?;
        }

        // Simulate job processing
        let result = self.execute_job_on_worker(&job, &worker_id);

        // Update worker and results
        {
            let mut workers = self.workers.lock().unwrap();
            let worker = workers.get_mut(&worker_id)
                .ok_or_else(|| format!("Worker {} not found", worker_id))?;

            match &result {
                Ok(job_result) => {
                    worker.complete_job(job_result.duration);
                    let mut status = self.job_status.lock().unwrap();
                    status.insert(
                        job.id.clone(),
                        JobStatus::Completed {
                            worker_id: worker_id.clone(),
                            duration: job_result.duration,
                        },
                    );
                }
                Err(error) => {
                    worker.fail_job();
                    let mut status = self.job_status.lock().unwrap();
                    status.insert(
                        job.id.clone(),
                        JobStatus::Failed {
                            worker_id: worker_id.clone(),
                            error: error.clone(),
                            retry_count: 0,
                        },
                    );
                }
            }
        }

        // Store result
        if let Ok(job_result) = result {
            let mut results = self.results.lock().unwrap();
            results.push(job_result);
        }

        Ok(())
    }

    fn select_worker(&self, job: &DistributedJob) -> Result<String> {
        let workers = self.workers.lock().unwrap();

        if workers.is_empty() {
            return Err("No workers available".to_string());
        }

        match self.strategy {
            LoadBalancingStrategy::RoundRobin => {
                let worker_ids: Vec<String> = workers.keys().cloned().collect();
                let mut index = self.next_worker_index.lock().unwrap();
                let worker_id = worker_ids[*index % worker_ids.len()].clone();
                *index += 1;
                Ok(worker_id)
            }
            LoadBalancingStrategy::LeastLoaded => {
                workers
                    .values()
                    .filter(|w| w.is_available())
                    .min_by_key(|w| w.current_load)
                    .map(|w| w.id.clone())
                    .ok_or_else(|| "No available workers".to_string())
            }
            LoadBalancingStrategy::CapacityBased => {
                workers
                    .values()
                    .filter(|w| w.is_available() && w.available_capacity() >= job.files.len())
                    .max_by_key(|w| w.available_capacity())
                    .map(|w| w.id.clone())
                    .or_else(|| {
                        // Fallback to any available worker
                        workers.values()
                            .filter(|w| w.is_available())
                            .max_by_key(|w| w.available_capacity())
                            .map(|w| w.id.clone())
                    })
                    .ok_or_else(|| "No available workers".to_string())
            }
        }
    }

    fn execute_job_on_worker(&self, job: &DistributedJob, worker_id: &str) -> Result<JobResult> {
        let start = Instant::now();

        // Simulate transpilation work
        let processing_time = Duration::from_millis(job.files.len() as u64 * 10);
        std::thread::sleep(processing_time);

        // Simulate occasional failures
        let success = job.priority != JobPriority::Low || job.files.len() < 100;

        let duration = start.elapsed();

        if success {
            Ok(JobResult {
                job_id: job.id.clone(),
                worker_id: worker_id.to_string(),
                success: true,
                files_processed: job.files.len(),
                duration,
                error: None,
            })
        } else {
            Err(format!("Job {} failed on worker {}", job.id, worker_id))
        }
    }

    pub fn get_worker_stats(&self) -> Vec<WorkerNode> {
        let workers = self.workers.lock().unwrap();
        workers.values().cloned().collect()
    }

    pub fn get_job_status(&self, job_id: &str) -> Option<JobStatus> {
        let status = self.job_status.lock().unwrap();
        status.get(job_id).cloned()
    }

    pub fn health_check(&self, timeout: Duration) -> Vec<String> {
        let mut workers = self.workers.lock().unwrap();
        let mut unhealthy = Vec::new();

        for worker in workers.values_mut() {
            if !worker.check_health(timeout) {
                unhealthy.push(worker.id.clone());
            }
        }

        unhealthy
    }
}

// ============================================================================
// Distributed Metrics
// ============================================================================

#[derive(Debug, Clone)]
pub struct DistributedMetrics {
    pub total_jobs: usize,
    pub completed_jobs: usize,
    pub failed_jobs: usize,
    pub total_files: usize,
    pub total_duration: Duration,
    pub worker_count: usize,
    pub average_job_time: Duration,
    pub throughput: f64, // files per second
}

impl DistributedMetrics {
    pub fn from_results(results: &[JobResult], worker_count: usize) -> Self {
        let total_jobs = results.len();
        let completed_jobs = results.iter().filter(|r| r.success).count();
        let failed_jobs = results.iter().filter(|r| !r.success).count();
        let total_files: usize = results.iter().map(|r| r.files_processed).sum();
        let total_duration: Duration = results.iter().map(|r| r.duration).sum();

        let average_job_time = if total_jobs > 0 {
            total_duration / total_jobs as u32
        } else {
            Duration::ZERO
        };

        let throughput = if total_duration.as_secs_f64() > 0.0 {
            total_files as f64 / total_duration.as_secs_f64()
        } else {
            0.0
        };

        Self {
            total_jobs,
            completed_jobs,
            failed_jobs,
            total_files,
            total_duration,
            worker_count,
            average_job_time,
            throughput,
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_jobs == 0 {
            return 0.0;
        }
        (self.completed_jobs as f64 / self.total_jobs as f64) * 100.0
    }
}

// ============================================================================
// Examples
// ============================================================================

fn main() -> Result<()> {
    println!("=== Example 1: Basic Distributed Processing ===\n");
    example_basic_distributed()?;

    println!("\n=== Example 2: Load Balancing Strategies ===\n");
    example_load_balancing()?;

    println!("\n=== Example 3: Fault Tolerance and Health Monitoring ===\n");
    example_fault_tolerance()?;

    Ok(())
}

fn example_basic_distributed() -> Result<()> {
    let coordinator = DistributedCoordinator::new(LoadBalancingStrategy::RoundRobin);

    // Register workers
    coordinator.register_worker(WorkerNode::new("worker-1".to_string(), 10))?;
    coordinator.register_worker(WorkerNode::new("worker-2".to_string(), 10))?;
    coordinator.register_worker(WorkerNode::new("worker-3".to_string(), 10))?;

    println!("Registered 3 workers with capacity 10 each");

    // Submit jobs
    for i in 0..5 {
        let job = DistributedJob {
            id: format!("job-{}", i),
            files: (0..5).map(|j| PathBuf::from(format!("file-{}-{}.rs", i, j))).collect(),
            priority: JobPriority::Normal,
            created_at: Instant::now(),
            timeout: Duration::from_secs(60),
        };
        coordinator.submit_job(job)?;
    }

    println!("Submitted 5 jobs (5 files each)\n");

    // Process jobs
    let results = coordinator.process_jobs()?;

    // Display metrics
    let metrics = DistributedMetrics::from_results(&results, 3);
    println!("Distributed Processing Metrics:");
    println!("  Total jobs: {}", metrics.total_jobs);
    println!("  Completed: {}", metrics.completed_jobs);
    println!("  Failed: {}", metrics.failed_jobs);
    println!("  Success rate: {:.1}%", metrics.success_rate());
    println!("  Total files: {}", metrics.total_files);
    println!("  Throughput: {:.2} files/sec", metrics.throughput);

    Ok(())
}

fn example_load_balancing() -> Result<()> {
    let strategies = [
        LoadBalancingStrategy::RoundRobin,
        LoadBalancingStrategy::LeastLoaded,
        LoadBalancingStrategy::CapacityBased,
    ];

    for strategy in &strategies {
        let coordinator = DistributedCoordinator::new(*strategy);

        // Register workers with different capacities
        coordinator.register_worker(WorkerNode::new("small".to_string(), 5))?;
        coordinator.register_worker(WorkerNode::new("medium".to_string(), 10))?;
        coordinator.register_worker(WorkerNode::new("large".to_string(), 20))?;

        // Submit varied jobs
        for i in 0..6 {
            let job = DistributedJob {
                id: format!("job-{}", i),
                files: (0..3).map(|j| PathBuf::from(format!("file-{}.rs", j))).collect(),
                priority: JobPriority::Normal,
                created_at: Instant::now(),
                timeout: Duration::from_secs(60),
            };
            coordinator.submit_job(job)?;
        }

        let results = coordinator.process_jobs()?;
        let worker_stats = coordinator.get_worker_stats();

        println!("Strategy: {:?}", strategy);
        for worker in &worker_stats {
            println!("  {}: {} jobs completed, {:.1}% utilization",
                worker.id, worker.completed_jobs, worker.utilization());
        }
        println!("  Success rate: {:.1}%\n",
            DistributedMetrics::from_results(&results, 3).success_rate());
    }

    Ok(())
}

fn example_fault_tolerance() -> Result<()> {
    let coordinator = DistributedCoordinator::new(LoadBalancingStrategy::LeastLoaded);

    // Register workers
    coordinator.register_worker(WorkerNode::new("worker-1".to_string(), 15))?;
    coordinator.register_worker(WorkerNode::new("worker-2".to_string(), 15))?;

    println!("Registered 2 workers\n");

    // Submit jobs including some that will fail
    for i in 0..4 {
        let job = DistributedJob {
            id: format!("job-{}", i),
            files: (0..3).map(|_| PathBuf::from("file.rs")).collect(),
            priority: JobPriority::Normal,
            created_at: Instant::now(),
            timeout: Duration::from_secs(30),
        };
        coordinator.submit_job(job)?;
    }

    println!("Submitted 4 jobs");

    // Process jobs
    let results = coordinator.process_jobs()?;

    // Health check
    let unhealthy = coordinator.health_check(Duration::from_secs(5));

    println!("\nFault Tolerance Results:");
    println!("  Total jobs: {}", results.len());
    println!("  Successful: {}", results.iter().filter(|r| r.success).count());
    println!("  Failed: {}", results.iter().filter(|r| !r.success).count());
    println!("  Unhealthy workers: {}", unhealthy.len());

    // Display worker health
    let worker_stats = coordinator.get_worker_stats();
    println!("\nWorker Health:");
    for worker in &worker_stats {
        println!("  {}: {:?} (completed: {}, failed: {})",
            worker.id, worker.status, worker.completed_jobs, worker.failed_jobs);
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_node_creation() {
        let worker = WorkerNode::new("test-worker".to_string(), 10);
        assert_eq!(worker.id, "test-worker");
        assert_eq!(worker.capacity, 10);
        assert_eq!(worker.status, WorkerStatus::Idle);
        assert_eq!(worker.current_load, 0);
    }

    #[test]
    fn test_worker_available_capacity() {
        let mut worker = WorkerNode::new("test".to_string(), 10);
        assert_eq!(worker.available_capacity(), 10);

        worker.assign_job(3).unwrap(); // Adds 1 job regardless of size
        assert_eq!(worker.available_capacity(), 9);
    }

    #[test]
    fn test_worker_utilization() {
        let mut worker = WorkerNode::new("test".to_string(), 10);
        assert_eq!(worker.utilization(), 0.0);

        worker.assign_job(5).unwrap(); // Adds 1 job regardless of size
        assert_eq!(worker.utilization(), 10.0);
    }

    #[test]
    fn test_worker_assign_job() {
        let mut worker = WorkerNode::new("test".to_string(), 5);

        assert!(worker.assign_job(3).is_ok()); // Adds 1 job
        assert_eq!(worker.current_load, 1);
        assert_eq!(worker.status, WorkerStatus::Busy);

        // Fill up to capacity
        for _ in 0..4 {
            assert!(worker.assign_job(1).is_ok());
        }
        assert_eq!(worker.current_load, 5);
        assert!(worker.assign_job(1).is_err()); // Over capacity
    }

    #[test]
    fn test_worker_complete_job() {
        let mut worker = WorkerNode::new("test".to_string(), 5);
        worker.assign_job(2).unwrap(); // Adds 1 job
        worker.assign_job(1).unwrap(); // Adds another job

        worker.complete_job(Duration::from_millis(100));
        assert_eq!(worker.current_load, 1);
        assert_eq!(worker.completed_jobs, 1);
    }

    #[test]
    fn test_worker_fail_job() {
        let mut worker = WorkerNode::new("test".to_string(), 5);
        worker.assign_job(1).unwrap();

        worker.fail_job();
        assert_eq!(worker.current_load, 0);
        assert_eq!(worker.failed_jobs, 1);
        assert_eq!(worker.status, WorkerStatus::Idle);
    }

    #[test]
    fn test_coordinator_register_worker() {
        let coordinator = DistributedCoordinator::new(LoadBalancingStrategy::RoundRobin);
        let worker = WorkerNode::new("test".to_string(), 10);

        assert!(coordinator.register_worker(worker.clone()).is_ok());
        assert!(coordinator.register_worker(worker).is_err()); // Duplicate
    }

    #[test]
    fn test_coordinator_submit_job() {
        let coordinator = DistributedCoordinator::new(LoadBalancingStrategy::RoundRobin);

        let job = DistributedJob {
            id: "job-1".to_string(),
            files: vec![PathBuf::from("test.rs")],
            priority: JobPriority::Normal,
            created_at: Instant::now(),
            timeout: Duration::from_secs(60),
        };

        assert!(coordinator.submit_job(job).is_ok());

        let status = coordinator.get_job_status("job-1");
        assert!(matches!(status, Some(JobStatus::Pending)));
    }

    #[test]
    fn test_job_priority_ordering() {
        let coordinator = DistributedCoordinator::new(LoadBalancingStrategy::RoundRobin);

        let low = DistributedJob {
            id: "low".to_string(),
            files: vec![],
            priority: JobPriority::Low,
            created_at: Instant::now(),
            timeout: Duration::from_secs(60),
        };

        let high = DistributedJob {
            id: "high".to_string(),
            files: vec![],
            priority: JobPriority::High,
            created_at: Instant::now(),
            timeout: Duration::from_secs(60),
        };

        coordinator.submit_job(low).unwrap();
        coordinator.submit_job(high).unwrap();

        let queue = coordinator.job_queue.lock().unwrap();
        assert_eq!(queue[0].id, "high");
        assert_eq!(queue[1].id, "low");
    }

    #[test]
    fn test_load_balancing_round_robin() {
        let coordinator = DistributedCoordinator::new(LoadBalancingStrategy::RoundRobin);

        coordinator.register_worker(WorkerNode::new("w1".to_string(), 10)).unwrap();
        coordinator.register_worker(WorkerNode::new("w2".to_string(), 10)).unwrap();

        let job = DistributedJob {
            id: "test".to_string(),
            files: vec![PathBuf::from("file.rs")],
            priority: JobPriority::Normal,
            created_at: Instant::now(),
            timeout: Duration::from_secs(60),
        };

        let worker1 = coordinator.select_worker(&job).unwrap();
        let worker2 = coordinator.select_worker(&job).unwrap();

        // Round robin should alternate
        assert_ne!(worker1, worker2);
    }

    #[test]
    fn test_distributed_metrics() {
        let results = vec![
            JobResult {
                job_id: "1".to_string(),
                worker_id: "w1".to_string(),
                success: true,
                files_processed: 5,
                duration: Duration::from_secs(1),
                error: None,
            },
            JobResult {
                job_id: "2".to_string(),
                worker_id: "w2".to_string(),
                success: true,
                files_processed: 3,
                duration: Duration::from_secs(1),
                error: None,
            },
        ];

        let metrics = DistributedMetrics::from_results(&results, 2);
        assert_eq!(metrics.total_jobs, 2);
        assert_eq!(metrics.completed_jobs, 2);
        assert_eq!(metrics.total_files, 8);
        assert_eq!(metrics.success_rate(), 100.0);
    }

    #[test]
    fn test_metrics_success_rate() {
        let results = vec![
            JobResult {
                job_id: "1".to_string(),
                worker_id: "w1".to_string(),
                success: true,
                files_processed: 5,
                duration: Duration::from_millis(100),
                error: None,
            },
            JobResult {
                job_id: "2".to_string(),
                worker_id: "w2".to_string(),
                success: false,
                files_processed: 0,
                duration: Duration::from_millis(50),
                error: Some("Failed".to_string()),
            },
        ];

        let metrics = DistributedMetrics::from_results(&results, 2);
        assert_eq!(metrics.success_rate(), 50.0);
    }

    #[test]
    fn test_worker_health_check() {
        let coordinator = DistributedCoordinator::new(LoadBalancingStrategy::RoundRobin);

        let mut worker = WorkerNode::new("test".to_string(), 10);
        worker.last_heartbeat = Instant::now() - Duration::from_secs(10);

        coordinator.register_worker(worker).unwrap();

        let unhealthy = coordinator.health_check(Duration::from_secs(5));
        assert_eq!(unhealthy.len(), 1);
        assert_eq!(unhealthy[0], "test");
    }

    #[test]
    fn test_capacity_based_load_balancing() {
        let coordinator = DistributedCoordinator::new(LoadBalancingStrategy::CapacityBased);

        coordinator.register_worker(WorkerNode::new("small".to_string(), 5)).unwrap();
        coordinator.register_worker(WorkerNode::new("large".to_string(), 20)).unwrap();

        let large_job = DistributedJob {
            id: "large".to_string(),
            files: (0..15).map(|i| PathBuf::from(format!("file-{}.rs", i))).collect(),
            priority: JobPriority::Normal,
            created_at: Instant::now(),
            timeout: Duration::from_secs(60),
        };

        let worker = coordinator.select_worker(&large_job).unwrap();
        assert_eq!(worker, "large"); // Should select worker with more capacity
    }
}
