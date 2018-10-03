use std::sync::Arc;
use std::fmt::{ Debug, Formatter, Result };
use math::*;
use core::*;

trait Pointable: Debug + Send + Sync {
  fn get_point(&self) -> Point;
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

enum Node<T: Pointable> {
    Internal(Arc<T>, Point, Axis, Box<Node<T>>, Box<Node<T>>),
    Leaf(Arc<T>, Point),
}

pub struct VolumeKdTree<T: Pointable> {
    root: Node<T>,
    bound: BoundingBox,
}

impl <T: Pointable> Node<T> {
    fn size(&self) -> usize {
        match self {
            &Node::Internal(_, _, _, ref left, ref right) => {
                left.size() + right.size()
            },
            &Node::Leaf(_, _) => {
                1
            },
        }
    }

    fn fmt_indented(&self, f: &mut Formatter, indent_level: usize) -> Result {
        match self {
            &Node::Internal(ref pointable, ref point, ref axis, ref left, ref right) => {
                write!(f, "{}{} objects, split {:?} at {:?}\n",  " ".repeat(indent_level * 2), self.size(), *axis, point)?;
                left.fmt_indented(f, indent_level + 1)?;
                right.fmt_indented(f, indent_level + 1)
            },
            &Node::Leaf(_, ref point) => {
                write!(f, "{}leaf at {:?}\n", " ".repeat(indent_level * 2), point)
            },
        }
    }
}

impl <T: Pointable> Debug for VolumeKdTree<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.root.fmt_indented(f, 0)
    }
}
