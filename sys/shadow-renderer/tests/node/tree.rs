extern crate insta;
extern crate omnidux_core;
extern crate omnidux_sys_shadow_renderer;

use insta::assert_debug_snapshot;
use omnidux_core::capsule::{Capsule, CapsuleContent};
use omnidux_sys_shadow_renderer::style::{Styles, Dimension};
use omnidux_sys_shadow_renderer::node::capsule::NodeTree;
use omnidux_sys_shadow_renderer::node::capsule::NodeTreeKey::*;
use omnidux_sys_shadow_renderer::node::capsule::NodeTreeAction::*;

#[test]
fn test_tree_mounting() {
  let nodeTree = NodeTree::new();

  let root = nodeTree.request_content(&RootNode).unwrap().id();
  nodeTree.set_content(ExNode(root), CapsuleContent::Some(SetStyles(Styles {
    width: Dimension::Points(600.0),
    height: Dimension::Points(400.0),
    ..Default::default()
  })));

  let flex1 = Styles {
    flex_basis: Dimension::Points(0.0),
    flex_grow: 1.0,
    flex_shrink: 1.0,
    ..Default::default()
  };

  let sub_1 = nodeTree.request_content(&NewNode(root, flex1));
  let sub_2 = nodeTree.request_content(&NewNode(root, flex1));

  let resolved = nodeTree.resolve();

  assert_debug_snapshot!(resolved);
}