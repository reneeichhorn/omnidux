#[macro_use]
extern crate omnidux_core;
extern crate omnidux_sys_shadow_renderer;

use std::sync::Arc;
use omnidux_core::threads::Thread;
use omnidux_core::task::Task;
use omnidux_core::config;
use omnidux_core::repo::Repository;
use omnidux_core::scheduler::context::Context;
use omnidux_core::capsule::CapsuleContent;

pub fn main () {
  // Parsing configuration.
  let config_str = include_str!("../../../omnidux.ios.yaml");
  let config = config::build_config_from_str(config_str).unwrap();

  // Initialize all repos
  let mut counter = 0usize;
  let repos: Vec<Arc<dyn Repository + Send + Sync>> = vec![
    Arc::new(
      omnidux_sys_shadow_renderer::Repository::new(config.setup[0].clone(), &mut counter)
    ),
  ];

  // Create threads.
  let mut threads = Vec::new();
  let mut senders = Vec::new();

  for (i, thread) in config.threads.iter().enumerate() {
    let thread = Thread::new(i, thread.clone());
    senders.push(thread.create_sender());
    threads.push(thread);
  }

  // Create context
  let context = Context::new(repos, senders);

  // Spawn threads
  for thread in &mut threads {
    thread.spawn(&context);
  }

  // Schedule dummy task.
  schedule_task!(context, omnidux_sys_shadow_renderer, Task1);

  // Block main thread.
  for thread in &mut threads {
    thread.block(&context);
  }
}