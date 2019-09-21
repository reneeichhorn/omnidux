use std::any::Any;

use crate::task::Task;
use crate::config::Schedule;

pub trait Repository {
  fn as_any(&self) -> &dyn Any;
  fn get_schedule_config(&self) -> Schedule;
  fn handle_schedule(&self, task: &Task);
  fn has_ownership(&self, uuid: usize) -> bool;
}

#[macro_export]
macro_rules! create_task_handler {
  ($($name:ident),* ,) => {
      |handlers: &mut Vec<Box<dyn TaskHandler + Sync + Send>>, counter: &mut usize| {
        $(
          println!("Registered task handler for uuid {uuid}", uuid = *counter);
          handlers.push(
            Box::new($name { uuid: *counter }),
          );
          *counter += 1;
        )*
      }
  }
}

#[macro_export]
macro_rules! create_task_traits {
  // Tail
  (@step $_idx:expr,) => {};
  // Counter
  (@step $idx:expr, $head:ident, $($tail:ident,)*) => {
    impl LocalSchedulable for $head {
      fn get_local_handler_uuid() -> usize {
        $idx
      }
    }

    create_task_traits!(@step $idx + 1usize, $($tail,)*);
  };
  ($($name:ident),* ,) => {
    create_task_traits!(@step 0usize, $($name,)*);
  }
}

#[macro_export]
macro_rules! create_capsule_def {
  ($($name:ident),* ,) => {
    pub struct Capsules {
      $(
        pub $name: std::sync::Arc<$name>,
      )*
    }
  };
}

#[macro_export]
macro_rules! create_capsule_holder {
  ($($name:ident),* ,) => {
    Capsules {
      $(
        $name: std::sync::Arc::new($name::new()),
      )*
    }
  }
}

#[macro_export]
macro_rules! create_repo {
  { tasks: $tasks:tt, capsules: $capsules:tt } => {
    use omnidux_core::config::Schedule;
    use omnidux_core::task::LocalSchedulable;

    /// A identifier that is unique across multiple repos.
    pub static mut uuid: usize = 0;

    // Struct that hols all capsules.
    create_capsule_def! $capsules;

    pub struct Repository {
      pub schedule_config: Schedule,
      pub start_index: usize,
      pub handlers: Vec<Box<dyn TaskHandler + Sync + Send>>,
      pub capsules: Capsules,
    }

    impl Repository {
      pub fn new(schedule: Schedule, counter: &mut usize) -> Repository {
        // Assign uuid to static mutable, safe in this context as it happens once in the main thread.
        unsafe { uuid = *counter };

        let initial = *counter;
        let mut handlers = Vec::new();
        let factory = create_task_handler! $tasks;
        factory(&mut handlers, counter);

        let capsules = create_capsule_holder! $capsules;

        Repository { 
          schedule_config: schedule,
          start_index: initial,
          handlers: handlers,
          capsules: capsules,
        }
      }
    }

    impl omnidux_core::repo::Repository for Repository {
      fn as_any(&self) -> &dyn std::any::Any {
        self
      }

      fn handle_schedule(&self, task: &Task) {
        let local_uuid = self.start_index - task.uuid;
        self.handlers.get(local_uuid).unwrap().handle(task);
      }

      fn has_ownership(&self, inner_uuid: usize) -> bool {
        self.start_index <= inner_uuid && inner_uuid <= (self.start_index + self.handlers.len())
      }
      
      fn get_schedule_config(&self) -> Schedule {
        self.schedule_config.clone()
      }
    }

    // Traits for tasks
    create_task_traits! $tasks;
  };
}

#[macro_export]
macro_rules! repo_get {
  ($context:ident, $repo:ident) => {
    {
      use std::any::Any;
      let repo_uuid = unsafe { $repo::uuid };
      let repo = $context.get_repo(repo_uuid).as_any().downcast_ref::<$repo::Repository>().unwrap();
      repo
    }
  };
}
