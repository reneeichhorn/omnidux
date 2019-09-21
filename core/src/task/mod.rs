use std::sync::Arc;
use std::any::Any;

pub type TaskPayload = Arc<dyn Any + Send + Sync>;

#[derive(Clone)]
/// Defines a Task that needs to be fulfilled by a task handler.
pub struct Task {
  /// List of thread uuids that are allowed to execute the task.
  pub execution_targets: Option<Vec<usize>>,
  /// Unique identifier for the task handler.
  pub uuid: usize,
  /// Data that was passed as a argument.
  pub payload: TaskPayload,
}

// Trait for handleable tasks.
pub trait TaskHandler {
  fn handle(&self, task: &Task);
}

// Trait for tasks that can be scheduled within a specific repo. 
pub trait LocalSchedulable {
  fn get_local_handler_uuid() -> usize;
}