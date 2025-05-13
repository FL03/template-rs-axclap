/*
    Appellation: sand <example>
    Contrib: @FL03
*/
use template_rs_bin::config::LogLevel;
use tokio_util::task::TaskTracker;

pub type Arcm<T> = std::sync::Arc<std::sync::Mutex<T>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initialize the tracing module
    LogLevel::trace().init_tracing("sand");
    // run the example
    _run().await
}

// Entry point
async fn _run() -> anyhow::Result<()> {
    // Create a worker manager instance
    let mut manager = WorkerManager::new();

    // Add workers to the manager
    manager.add_worker("Alice");
    manager.add_worker("Bob");
    manager.add_worker("Charlie");

    // List all workers
    manager.list_workers();

    // Wait for all workers to complete
    Ok(manager.wait().await)
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(default)]
pub struct Message {
    id: usize,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub tasks: TaskTracker,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(default)]
pub struct Worker {
    #[serde(default = "uuid::Uuid::new_v4")]
    id: uuid::Uuid,
    name: String,
    position: usize,
}

#[derive(Clone, Debug)]
pub struct WorkerManager {
    tracker: TaskTracker,
    workers: Arcm<Vec<Worker>>,
}

mod impl_work {
    use super::{Arcm, Worker, WorkerManager};
    use std::{
        sync::atomic::{AtomicUsize, Ordering},
        time::Duration,
    };
    use tokio_util::task::TaskTracker;
    use uuid::Uuid;

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    impl Worker {
        // Function to create a new worker
        pub fn new(name: impl ToString) -> Self {
            Worker {
                id: Uuid::new_v4(),
                name: name.to_string(),
                position: COUNTER.fetch_add(1, Ordering::SeqCst),
            }
        }

        pub fn with_position(self, position: usize) -> Worker {
            Worker { position, ..self }
        }

        pub fn id(&self) -> Uuid {
            self.id
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn position(&self) -> usize {
            self.position
        }

        pub fn to_shared(&self) -> std::sync::Arc<Self> {
            std::sync::Arc::new(self.clone())
        }

        pub fn into_arcm(self) -> Arcm<Self> {
            std::sync::Mutex::new(self).into()
        }

        // Simulate the work done by this worker
        #[tracing::instrument(fields(worker = %self.name), skip(self), name="run", target="worker")]
        pub async fn run(self) {
            // Simulate some work for demonstration purposes
            tracing::info!("starting some process");
            tokio::time::sleep(Duration::from_secs(2)).await;
            tracing::info!("process completed");
        }
    }

    unsafe impl Send for Worker {}

    unsafe impl Sync for Worker {}
    /*
     ************* Worker Manager *************
     */
    impl WorkerManager {
        // Create a new worker manager
        pub fn new() -> Self {
            WorkerManager {
                tracker: TaskTracker::new(),
                workers: std::sync::Mutex::default().into(),
            }
        }
        /// Add a new worker to the manager and spawn its task
        pub fn add_worker(&mut self, name: &str) {
            // initialize a new worker
            let worker = Worker::new(name);
            // Add the worker to the manager
            self.workers_locked().push(worker.clone());
            // Spawn the worker's task
            self.tracker.spawn(worker.clone().run());
        }
        /// display a list of all workers
        pub fn list_workers(&self) {
            let workers = self.workers_locked();
            for Worker { id, name, position } in workers.iter() {
                println!("Worker ({position}.{id}): {name}");
            }
        }
        /// Wait for all workers to complete
        pub async fn wait(&self) {
            self.tracker.wait().await
        }
        ///
        pub fn workers_locked(&self) -> std::sync::MutexGuard<'_, Vec<Worker>> {
            self.workers.lock().unwrap()
        }

        pub fn workers_owned(&self) -> Vec<Worker> {
            self.workers.lock().unwrap().clone()
        }
    }
}
