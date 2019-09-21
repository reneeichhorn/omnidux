use std::collections::HashMap;
use std::sync::RwLock;

use omnidux_core::capsule::{Capsule, CapsuleContent};
use stretch::Stretch;
use stretch::node::{Node as SNode};
use stretch::style::{Style as SStyles};
use stretch::geometry::{Size as SSize};

use super::node::{Node, NodeId, Layout};
use super::shadow::ShadowDiffer;
use crate::style::Styles;

pub struct NodeTree {
  stretch: RwLock<Stretch>,
  shadow: RwLock<HashMap<NodeId, Node>>,
  root: NodeId,
}

#[derive(Debug)]
pub enum ResolvedNode {
  Node { styles: Styles, layout: Layout, children: Vec<ResolvedNode> },
  Leaf,
}

impl NodeTree {
  /// Creates a new instance of the NodeTree and creates the first root node.
  pub fn new() -> NodeTree {
    let mut stretch = Stretch::new();

    let root = NodeId::from(stretch.new_node(Default::default(), Vec::new()).unwrap());
    let mut shadow = HashMap::new();
    shadow.insert(root, Node::new(Default::default()));

    NodeTree {
      stretch: RwLock::new(stretch),
      shadow: RwLock::new(shadow),
      root: root,
    }
  }

  /// Recursively travels the root node and resolves the whole tree (Debug purpose only)
  pub fn resolve(&self) -> ResolvedNode {
    self.resolve_node(self.root)
  }

  /// Recursively travels a node (Debug purpose only)
  pub fn resolve_node(&self, node: NodeId) -> ResolvedNode {
    let stretch = self.stretch.read().unwrap();
    let shadow = self.shadow.read().unwrap();
    let shadow_node = shadow.get(&node).unwrap();

    ResolvedNode::Node {
      styles: shadow_node.styles,
      layout: shadow_node.layout,
      children: stretch.children(SNode::from(node))
        .unwrap()
        .iter()
        .map(|child| self.resolve_node(NodeId::from(*child)))
        .collect(),
    }
  }
}

#[derive(Copy, Clone)]
pub enum NodeTreeKey {
  /// Targets all pending changes.
  PollPendingChanges,
  /// Targets the automatically create root node.
  RootNode,
  /// Targets a new node that belongs to a parent and contains a certain styling.
  NewNode(NodeId, Styles),
  /// Targets any existing node somewhere in the tree.
  ExNode(NodeId),
}

#[derive(Copy, Clone)]
pub enum NodeTreeAction {
  /// The response for a newly create node.
  Id(NodeId),
  /// The action that forces a node to be removed from the tree.
  Remove,
  /// The action that updates the styling of a tree.
  SetStyles(Styles),
}

impl NodeTreeAction {
  pub fn id (&self) -> NodeId {
    match self {
      NodeTreeAction::Id(id) => *id,
      _ => panic!("Node tree response was not a node id.")
    }
  }
}

impl Capsule<NodeTreeKey, NodeTreeAction> for NodeTree {
  /// Request is used to request the creation of a new empty node
  /// that can be later modified after rendering the content.
  fn request_content(&self, key: &NodeTreeKey) -> CapsuleContent<NodeTreeAction> {
    use NodeTreeKey::{*};

    match key {
      RootNode => {
        CapsuleContent::Some(NodeTreeAction::Id(self.root))
      },
      NewNode(parent, styles) => {
        // Create an empty node.
        let mut stretch = self.stretch.write().unwrap();
        let node = stretch.new_node(SStyles::from(*styles), Vec::new()).unwrap();

        // Append new node to parent.
        stretch.add_child(SNode::from(parent.clone()), node).unwrap();

        // Trigger recalculation
        stretch.compute_layout(SNode::from(self.root), SSize::undefined()).unwrap();

        {
          // Append shadow node.
          let mut shadow_tree = self.shadow.write().unwrap();
          shadow_tree.insert(NodeId::from(node), Node::new(*styles));

          // Synch stretch tree with shadow tree and find layout diffs.
          ShadowDiffer::new(&mut shadow_tree, &stretch)
            .with_layout(self.root);
        }

        // Return node id.
        CapsuleContent::Some(NodeTreeAction::Id(NodeId::from(node)))
      },
      _ => panic!("Requesting from capsule is only allowed for new content."),
    }
  }

  /// Can be used to update an existing node.
  fn set_content(&self, key: NodeTreeKey, value: CapsuleContent<NodeTreeAction>) {
    use NodeTreeKey::{*};
    use NodeTreeAction::{*};

    let node = match key {
      ExNode(node) => node,
      _ => panic!("Invalid NodeTree target."),
    };

    match value {
      CapsuleContent::Some(action) => {
        match action {
          SetStyles(styles) => {
            // Apply style to content.
            {
              let mut stretch = self.stretch.write().unwrap();
              stretch.set_style(SNode::from(node), SStyles::from(styles));

              // Trigger recalculation
              stretch.compute_layout(SNode::from(self.root), SSize::undefined()).unwrap();
            }

            // Apply new style to shadow tree.
            let mut shadow_tree = self.shadow.write().unwrap();
            shadow_tree.get_mut(&node).unwrap().styles = styles;

            // Synch stretch tree with shadow tree and find layout diffs.
            let stretch = self.stretch.read().unwrap();
            ShadowDiffer::new(&mut shadow_tree, &stretch)
              .with_layout(self.root);
          },
          _ => panic!("Invalid NodeTree action."),
        }
      },
      _ => panic!("NodeTree only supports CapsuleContent::some(..)"),
    }
  }
}