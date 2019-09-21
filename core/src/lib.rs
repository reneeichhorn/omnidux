#[macro_use()]
extern crate serde;
extern crate serde_yaml;
extern crate cargo_toml_builder;
extern crate static_assertions;
extern crate futures;
extern crate evmap;
extern crate proc_macro_hack;
extern crate omnidux_macros;

pub mod config;
pub mod generator;
pub mod task;
pub mod threads;
pub mod uuid;
#[macro_use]
pub mod repo;
#[macro_use]
pub mod scheduler;
pub mod capsule;

// Reexporting macros.
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use omnidux_macros::stylesheet;
