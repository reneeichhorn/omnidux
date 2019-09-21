// Written content.
component! {
  // Options.
  name = MyComponent; // Component must have a name.

  // Props definitions.
  @prop let my_property1: usize; // Prop can be required.
  @prop let my_property2: usize = 32.0; // Prop can be optional and have a default value.

  // State definitions.
  @state let mut my_state1: usize = 0.0; // Local component state.
  @state let mut my_state2: usize = 1.0; // Local component state.

  // Binding definitions.
  @bind let my_capsule1: MyCapsule1 = { ..key.. }; // Binds a capsule for a specific key.
  @bind let my_capsule2: MyCapsule2 = { ..key.. }; // Binds a capsule for a specific key.

  // Lifecycle definitions.
  @lifecycle willMount() {} // Executed when the component is mounted.
  @lifecycle willUnmount() {} // Executed when the component is unmounted.
  @lifecycle shouldUpdate() -> bool {} // Executed before a component is rendered to decided whether rerendering is needed.

  // Rendering section (may contain any rust code).
  let mut foo = 1;
  if foo == 1 {
    foo = my_property1; // may access component prop like a normal variable.
  }
  let foo2 = my_state1; // may access compoent state like a normal variable. 
  let foo3 = my_capsule1; // may access bound state like a normal variable.

  //my_state1 = 32.0; // ERROR! state is not allowed to be changed while rendering.
  //my_capsule1 = 32.0; // ERROR! bound state is not allowed to be changed while rendering.

  View { // May have only have the component name and children (if no props are required).
    CustomComponent(label = "hello", value = my_capsule1), // Component props can be set by name.
    CustomComponent(label = "world"), // Not all props have to be set.
    CustomView(label = "!") { // Can also have optionally children
      if my_state1 == 32.0 { // Can contain conditional
        CustomComponent(label = "1"), 
        CustomComponent(label = "2"),
      }
      CustomComponent(label = "3"),
      match my_capsule2 { // Can also match content.
        MyEnum::val => { // Matching leaf.
          CustomComponent(label = "xyz"), // Content only rendered when matched.
          CustomComponent(label = "zyx"), // Content only rendered when matched.
        },
        _ => { // Non-matching leaf
          CustomComponent(label = "error"), // Content only if nothing rendered.
        }
      }
      CustomComponent(label = "4"),
      for var in my_state1 { // Can contain loop
        CustomComponent(key = var.id, label = "x"), // Repeated content (must contain key)
      }
      {...my_state}  // Can merge array of other components into the node.
    }
  }
}

// Content after procedural macro.
struct MyComponentProps {
  myProperty: usize,
  myProperty2: usize
}

struct MyComponentState {
  my_capsule1: Option<MyCapsule1::Val>,
  my_capsule2: Option<MyCapsule2::Val>,
  my_state1: usize,
  my_state2: usize,
}

struct MyComponentInternalState {
  generated_keys: HashMap<usize, usize>,
  key_root: usize,
  key_1: usize,
  key_2: usize,
  key_3: usize,
  key_3_1: usize,
  key_3_2: usize,
  key_3_3: usize,
  key_3_4: usize,
  key_3_5: usize,
  key_3_6: usize,
  key_3_7: usize,
  key_3_8_x: Vec<usize>,
}

pub fn initialize_my_component() {
  MyComponentInternalState {
    generated_keys: HashMap::new(),
    key_root: 0,
    key_1: 0,
    key_2: 0,
    key_3: 0,
    key_3_1: 0,
    key_3_2: 0,
    key_3_3: 0,
    key_3_4: 0,
    key_3_5: 0,
    key_3_6: 0,
    key_3_7: 0,
    key_3_8_x: Vec::new(),
  }
}

pub fn render_my_component(
  context: &Context,
  props: &MyComponentProps,
  state: &mut MyComponentState,
  internal: &MyComponentInternalState,
) {
  // Untransformed code
  let mut foo = 1;
  if foo == 1 {
    foo = my_property1;
  }
  let foo2 = my_state1; 
  let foo3 = my_capsule1;

  // Transformed render tree
  // -------------------------------------
  mount_or_render!(View, internal.key_root, {});
  mount_or_render_child!(
    internal.key_root,
    0,
    CustomComponent,
    internal.key_1, 
    { label: "hello", value = state.my_capsule1 }
  );
  mount_or_render_child!(
    internal.key_root,
    1,
    CustomComponent,
    internal.key_2, 
    { label: "world" }
  );
  mount_or_render_child!(
    internal.key_root,
    2,
    CustomView,
    internal.key_3, 
    { label: "!" }
  );

  if my_state1 == 32.0 {
    mount_or_render_child!(
      internal.key_3,
      3,
      CustomComponent,
      internal.key_3_1, 
      { label: "1" }
    );
    mount_or_render_child!(
      internal.key_3,
      4,
      CustomComponent,
      internal.key_3_2, 
      { label: "2" }
    );
  }

  mount_or_render_child!(
    internal.key_3,
    5,
    CustomComponent,
    internal.key_3_3, 
    { label: "3" }
  );

  match my_capsule2 {
    MyEnum::val => {
      mount_or_render_child!(
        internal.key_3,
        6,
        CustomComponent,
        internal.key_3_4, 
        { label: "xyz" }
      );
      mount_or_render_child!(
        internal.key_3,
        7,
        CustomComponent,
        internal.key_3_5, 
        { label: "zyx" }
      );
    },
    _ => { // Non-matching leaf
      mount_or_render_child!(
        internal.key_3,
        8,
        CustomComponent,
        internal.key_3_6, 
        { label: "error" }
      );
    }
  }

  mount_or_render_child!(
    internal.key_3,
    9,
    CustomComponent,
    internal.key_3_7, 
    { label: "4" }
  );

  let mut index1 = 10;
  for var in my_state1 {
    mount_or_render_child_loop!(
      internal.key_3_8_x,
      var.id,
      index1,
      CustomComponent,
      internal.key_3_6, 
      { label: "x" }
    );
    index1 += 1;
  }
}