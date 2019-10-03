
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
fn component_macro_test() {
  component! {
    name = MyComponent;

    @prop let my_opt_property: usize = 1.0;
    @prop let my_req_property: usize;

    @state let mut my_state: usize = 0.0;

    @bind let my_capsule: MyCapsule = { foo: my_state };
    @bind let my_capsule2: MyCapsule = { bar: my_state };

    let foo = my_state;
    let mut test = foo * 2 * my_state;
    if foo == 1 {
      test = 1.0;
    }

    View {
      Text(label = "foo")
    }
 };
}