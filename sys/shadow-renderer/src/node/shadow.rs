use std::collections::HashMap;
use stretch::Stretch;
use stretch::node::{Node as SNode};
use crate::node::node::{NodeId, Node};

pub enum PropertyValue {
  String(String),
}

pub enum PendingChange {
  MountNode { node_type: String },
  UpdatePropertyList { id: u64, property_list: Vec<(String, PropertyValue)> },
}

pub struct ShadowDiffer<'a> {
  shadow_tree: &'a mut HashMap<NodeId, Node>,
  stretch: &'a Stretch,
}

impl<'a> ShadowDiffer<'a> {
  pub fn new(
    shadow_tree: &'a mut HashMap<NodeId, Node>,
    stretch: &'a Stretch,
  ) -> Self {
    ShadowDiffer {
      shadow_tree: shadow_tree,
      stretch: stretch,
    }
  }

  fn diff_stretch_layout(&mut self, node: SNode) {
    let layout = self.stretch.layout(node).unwrap();
    let shadow_node = self.shadow_tree.get_mut(&NodeId::from(node)).unwrap();

    // Sync layout.
    shadow_node.layout.x = layout.location.x;
    shadow_node.layout.y = layout.location.y;
    shadow_node.layout.width = layout.size.width;
    shadow_node.layout.height = layout.size.height;

    // Sync children.
    let children = self.stretch.children(node).unwrap();
    for child in children {
      self.diff_stretch_layout(child);
    }
  }
  
  pub fn with_layout(mut self, root: NodeId) -> Self {
    self.diff_stretch_layout(SNode::from(root));
    self
  }
}
