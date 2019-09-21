use crate::config::Schedule;

/// Trait that is required to be implement on each task
/// The module `omnidux_core::scheduler::strategy` contains multiple
/// easy to use strategy implementation that can be implemented for your task
/// using the `impl_strategy!` macro.
/// 
/// The schedule strategy defines where a task is supposed to be executed.
/// The strategy implementation gets the schedule configuration of the owning repo
/// and is then able to decide on which threads the task is executed.
pub trait ScheduleStrategy {
  fn find_preferred_target (schedule: &Schedule) -> Vec<usize>; 
}

/// Schedule strategy that will always schedule the task on the first available
/// thread.
pub fn take_first(_schedule: &Schedule) -> Vec<usize> {
  vec![0]
}

#[macro_export]
macro_rules! impl_strategy {
  ($task:ident, $fn:ident) => {
    impl ScheduleStrategy for $task {
      fn find_preferred_target (schedule: &omnidux_core::config::Schedule) -> Vec<usize> {
        $fn(schedule)
      } 
    }
  };
}
