#![allow(dead_code)]

use rustc_hash::FxHashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(usize);

#[derive(Debug)]
pub struct Tree<T>
where
    T: Debug,
{
    next_id: usize,
    nodes: FxHashMap<NodeId, TreeNode<T>>,
}

impl<T> Tree<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        Self {
            next_id: 0,
            nodes: FxHashMap::default(),
        }
    }
    pub fn node_at(&self, node_id: Option<NodeId>) -> Option<&TreeNode<T>> {
        if let Some(node) = node_id {
            let n = self.nodes.get(&node);
            // if n.is_none() {
            //     panic!("missing {:?}", node_id);
            // };
            n
        } else {
            None
        }
    }

    pub fn at(&self, node: NodeId) -> &T {
        &self.nodes.get(&node).unwrap().data
    }
    pub fn at_mut(&mut self, node: NodeId) -> &mut T {
        &mut self.nodes.get_mut(&node).unwrap().data
    }

    pub fn create_root(&mut self, data: T) -> NodeId {
        self.add_node(data, None)
    }

    pub fn set_left(&mut self, node: NodeId, data: T) -> NodeId {
        let new_node_id = self.add_node(data, Some(node));
        self.update_node(node, |n| n.left = Some(new_node_id));
        new_node_id
    }

    pub fn set_right(&mut self, node: NodeId, data: T) -> NodeId {
        let new_node_id = self.add_node(data, Some(node));
        self.update_node(node, |n| n.right = Some(new_node_id));
        new_node_id
    }

    pub fn set_straight(&mut self, node: NodeId, data: T) -> NodeId {
        let new_node_id = self.add_node(data, Some(node));
        self.update_node(node, |n| n.straight = Some(new_node_id));
        new_node_id
    }

    fn update_node<F>(&mut self, node: NodeId, f: F)
    where
        F: Fn(&mut TreeNode<T>),
    {
        self.nodes.entry(node).and_modify(f);
    }
    pub fn update_value<F>(&mut self, node: NodeId, f: F)
    where
        F: Fn(&mut T),
    {
        self.update_node(node, |n| f(&mut n.data));
    }

    pub fn cull_nodes<Predicate>(&mut self, pred: Predicate) -> Vec<NodeId>
    where
        Predicate: Fn(&NodeId, &T) -> bool,
    {
        let mut culled = Vec::new();

        self.nodes = self
            .nodes
            .drain()
            .into_iter()
            .filter(|p| {
                let cull = pred(&p.0, &p.1.data);
                if cull {
                    culled.push(p.0);
                }
                !cull
            })
            .collect::<FxHashMap<NodeId, TreeNode<T>>>();
        culled
    }

    pub fn walk_parents<F>(&self, node: NodeId, f: &mut F)
    where
        F: FnMut(&T),
    {
        let mut current_id = self.node_at(Some(node)).unwrap().parent;
        while let Some(current) = self.node_at(current_id) {
            f(&current.data);
            current_id = current.parent;
        }
    }

    pub fn walk<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        if let Some(node) = self.node_at(Some(NodeId(0))) {
            node.walk(self, f);
        }
    }

    fn walk_node<F>(&self, node: &Option<NodeId>, f: &mut F)
    where
        F: FnMut(&T),
    {
        if let Some(node) = self.node_at(*node) {
            node.walk(self, f)
        }
    }

    fn add_node(&mut self, data: T, parent: Option<NodeId>) -> NodeId {
        let id = NodeId(self.next_id);
        self.next_id += 1;

        self.nodes.insert(id, TreeNode::<T>::new(id, data, parent));

        id
    }
}

#[derive(Debug)]
pub struct TreeNode<T> {
    id: NodeId,
    parent: Option<NodeId>,
    left: Option<NodeId>,
    straight: Option<NodeId>,
    right: Option<NodeId>,
    data: T,
}

impl<T> TreeNode<T>
where
    T: Debug,
{
    fn new(id: NodeId, data: T, parent: Option<NodeId>) -> Self {
        Self {
            id,
            parent,
            left: None,
            straight: None,
            right: None,
            data,
        }
    }

    fn walk<F>(&self, tree: &Tree<T>, f: &mut F)
    where
        F: FnMut(&T),
    {
        f(&self.data);
        tree.walk_node(&self.straight, f);
        tree.walk_node(&self.left, f);
        tree.walk_node(&self.right, f);
    }
}
