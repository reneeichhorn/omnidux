#[macro_use]
extern crate omnidux_core;
extern crate stretch;

use omnidux_core::scheduler::strategy::{ScheduleStrategy, take_first};
use omnidux_core::task::{TaskHandler, Task};
use omnidux_core::capsule::CapsuleContent;

pub mod node;
pub mod style;

impl_default_capsule! (MyCapsule, usize, usize);

#[derive(Clone)]
pub struct Task1 { uuid: usize }
impl TaskHandler for Task1 {
  fn handle(&self, task: &Task) { println!("Wup wup"); }
}
impl_strategy! (Task1, take_first);

#[derive(Clone)]
pub struct Task2 { uuid: usize }
impl TaskHandler for Task2 {
  fn handle(&self, task: &Task) {}
}

struct Foo;

create_repo! {
  tasks: [
    Task1,
    Task2,
  ],
  capsules: [
    MyCapsule,
  ]
}