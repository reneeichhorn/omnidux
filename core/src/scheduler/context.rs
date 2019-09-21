use std::sync::Arc;

use crate::repo::Repository;
use crate::threads::ThreadSender;
use crate::task::Task;
use crate::scheduler::strategy::ScheduleStrategy;

#[derive(Clone)]
pub struct Context {
  repos: Vec<Arc<dyn Repository + Send + Sync>>,
  senders: Vec<ThreadSender>,
}

impl Context {
  /// Creates a new context by list of repositories and senders.
  pub fn new(repos: Vec<Arc<dyn Repository + Send + Sync>>, senders: Vec<ThreadSender>) -> Context {
    Context {
      repos: repos,
      senders: senders,
    }
  }

  // Global schedule handle of a task.
  pub fn handle_schedule(&self, thread_uuid: usize, task: &Task) {
    // Skipping execution as it was scheduled for a different thread.
    if !task.execution_targets.as_ref().unwrap().contains(&thread_uuid) {
      return;
    }

    for repo in self.repos.iter() {
      if !repo.has_ownership(task.uuid) {
        continue;
      }

      repo.handle_schedule(task);
      break;
    }
  }

  pub fn schedule<T: ScheduleStrategy>(&self, mut task: Task) {
    // Find all available targets
    let target_repo = self.repos.iter()
      .find(|&x| x.has_ownership(task.uuid)).unwrap();

    // Find preferred targets.
    let targets = T::find_preferred_target(&target_repo.get_schedule_config());
    task.execution_targets = Some(targets);

    // Spread tasks using set senders.
    for sender in self.senders.iter() {
      sender.send_task(task.clone());
    }
  }

  pub fn get_repo(&self, uuid: usize) -> &Arc<dyn Repository + Send + Sync> {
    self.repos.iter()
      .find(|&x| x.has_ownership(uuid)).unwrap() 
  }
}