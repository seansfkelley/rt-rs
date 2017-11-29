use std::f64;
use super::bounding_box::Boundable;

pub enum Axis {
    X,
    Y,
    Z,
}

pub enum KdTreeNode {
    Internal(Axis, f64, Box<KdTreeNode>, Box<KdTreeNode>),
    Leaf(Vec<Box<Boundable>>),
}

pub struct KdTree {
    tree: KdTreeNode,
}

impl KdTree {
    pub fn from(items: Vec<Box<Boundable>>) -> KdTree {
        KdTree {
            tree: KdTreeNode::Leaf(items)
        }
    }
}
