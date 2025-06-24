/**
 * Refactored job scheduling system with proper encapsulation and clean architecture.
 * 
 * Key improvements:
 * - Proper encapsulation of internal state
 * - Reduced coupling between components
 * - Leverages Rust's ownership and borrowing system
 * - Follows clean code principles
 */

#[derive(Debug, Clone)]
pub struct Job {
    id: u32,
    description: String,
}

impl Job {
    /// Creates a new job with the given id and description
    pub fn new(id: u32, description: &str) -> Self {
        Self {
            id,
            description: description.to_string(),
        }
    }

    /// Returns formatted job details
    pub fn get_details(&self) -> String {
        format!("Job #{}: {}", self.id, self.description)
    }

    /// Gets the job ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Gets the job description
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// A queue for managing jobs with proper encapsulation
pub struct JobQueue {
    queue: Vec<Job>,
}

impl JobQueue {
    /// Creates a new empty job queue
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }

    /// Adds a new job to the queue
    pub fn add_job(&mut self, id: u32, description: &str) {
        let job = Job::new(id, description);
        self.queue.push(job);
    }

    /// Checks if there are pending jobs in the queue
    pub fn has_pending_jobs(&self) -> bool {
        !self.queue.is_empty()
    }

    /// Removes and returns the next job from the queue
    /// Returns None if the queue is empty
    pub fn take_next_job(&mut self) -> Option<Job> {
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }

    /// Returns the number of pending jobs
    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }
}

impl Default for JobQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// A processor that can handle jobs from a queue
pub struct Processor {
    name: String,
}

impl Processor {
    /// Creates a new processor with the given name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    /// Processes all jobs in the given queue
    /// Returns an error if there are no pending jobs
    pub fn process_jobs(&self, queue: &mut JobQueue) -> Result<(), String> {
        if !queue.has_pending_jobs() {
            return Err("No pending jobs to process".to_string());
        }

        let mut processed_count = 0;
        while let Some(job) = queue.take_next_job() {
            println!("Processor '{}' processing: {}", self.name, job.get_details());
            processed_count += 1;
        }

        println!("Processor '{}' completed processing {} jobs", self.name, processed_count);
        Ok(())
    }

    /// Processes a single job from the queue
    /// Returns the processed job or an error if no jobs are available
    pub fn process_single_job(&self, queue: &mut JobQueue) -> Result<Job, String> {
        match queue.take_next_job() {
            Some(job) => {
                println!("Processor '{}' processing: {}", self.name, job.get_details());
                Ok(job)
            }
            None => Err("No jobs available to process".to_string()),
        }
    }

    /// Gets the processor name
    pub fn name(&self) -> &str {
        &self.name
    }
}

fn main() {
    println!("=== Job Scheduling System Demo ===");
    
    // Create a new job queue and processor
    let mut queue = JobQueue::new();
    let processor = Processor::new("Main Processor");
    
    println!("\n1. Adding jobs to the queue...");
    queue.add_job(1, "Compile source code");
    queue.add_job(2, "Run unit tests");
    queue.add_job(3, "Deploy to staging");
    queue.add_job(4, "Run integration tests");
    queue.add_job(5, "Deploy to production");
    
    println!("Queue has {} pending jobs", queue.pending_count());
    
    println!("\n2. Processing a single job...");
    match processor.process_single_job(&mut queue) {
        Ok(job) => println!("Successfully processed: {}", job.get_details()),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("Queue now has {} pending jobs", queue.pending_count());
    
    println!("\n3. Processing all remaining jobs...");
    match processor.process_jobs(&mut queue) {
        Ok(()) => println!("All jobs completed successfully!"),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("\n4. Trying to process jobs from empty queue...");
    match processor.process_jobs(&mut queue) {
        Ok(()) => println!("Jobs processed successfully!"),
        Err(e) => println!("Expected error: {}", e),
    }
    
    println!("\n=== Demo Complete ===");
}

