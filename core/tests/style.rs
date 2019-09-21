
extern crate omnidux_core;
extern crate omnidux_sys_shadow_renderer;

use omnidux_core::{stylesheet, component};

#[test]
fn stylesheet_macro_test() {
  let foo = stylesheet! {
    test {
      flex-grow: 1;
      flex-shrink: 2;
    }
  };

  println!("foo {:#?}", foo);
}

#[test]
fn component_macro_test {
  component! {
    name = MyComponent;

    @prop let my_opt_property: usize = 1.0;
    @prop let my_req_property: usize;
  }
}