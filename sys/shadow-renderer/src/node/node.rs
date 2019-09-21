use stretch::node::{Node as SNode};
use crate::style::Styles;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct NodeId (SNode);

impl From<SNode> for NodeId {
  fn from(node: SNode) -> Self {
    NodeId(node)
  }
}

impl From<NodeId> for SNode {
  fn from(node: NodeId) -> Self {
    node.0
  }
}

#[derive(Copy, Clone, Debug)]
pub struct Layout {
  pub(crate) width: f32,
  pub(crate) height: f32,
  pub(crate) x: f32,
  pub(crate) y: f32,
}

pub struct Node {
  pub(crate) layout: Layout,
  pub(crate) styles: crate::style::Styles,
}

impl Node {
  pub fn new(styles: Styles) -> Node {
    Node {
      layout: Layout {
        width: 0.0,
        height: 0.0,
        x: 0.0,
        y: 0.0,
      },
      styles: styles,
    }
  }
}