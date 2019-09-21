pub mod context;
pub mod strategy;

#[macro_export]
macro_rules! schedule_task {
  ($context:ident, $repo:ident, $task:ident) => {
    {
      use std::sync::Arc;
      use omnidux_core::task::LocalSchedulable;

      // Safe because uuid is only set once in its ifetime.
      let repo_uuid = unsafe { $repo::uuid };
      let uuid = repo_uuid + $repo::$task::get_local_handler_uuid();

      $context.schedule::<$repo::$task>(Task {
        execution_targets: None,
        uuid: uuid,
        payload: Arc::new(Some(0usize)),
      });
    }
  };
}
