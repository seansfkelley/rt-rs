use std::sync::Arc;
use std::fmt::{ Debug, Formatter, Result };
use ordered_float::NotNaN;
use math::*;
use core::*;

trait Pointable: Debug + Send + Sync {
  fn get_point(&self) -> Point;
}

enum Node<T: Pointable> {
    Internal(Arc<T>, Point, Axis, Box<Node<T>>, Box<Node<T>>),
    Leaf(Arc<T>),
    Empty,
}

pub struct PointKdTree<T: Pointable> {
    root: Node<T>,
}

impl <T: Pointable> PointKdTree<T> {
    pub fn from(items: Vec<T>) -> PointKdTree<T> {
        let arc_items: Vec<Arc<T>> = items
            .into_iter()
            .map(|i| Arc::new(i))
            .collect();

        let tree_bound = items
            .iter()
            .fold(BoundingBox::empty(), |unioned_bounds, &item| BoundingBox::with_point(&unioned_bounds, &item.get_point()));

        PointKdTree {
            root: recursively_build_tree(&mut arc_items, tree_bound),
        }
    }
}

fn recursively_build_tree<T: Pointable>(items: &mut [Arc<T>], node_bounds: BoundingBox) -> Node<T> {
    if items.len() == 0 {
        Node::Empty
    } else if items.len() == 1 {
        Node::Leaf(Arc::clone(items.first().unwrap()))
    } else {
        let split_axis = node_bounds.maximum_extent();

        // TODO: Partitioning can be done in O(n) rather than O(nlgn)
        items.sort_unstable_by(|ref p1, ref p2| {
            let value1 = NotNaN::new(p1.get_point()[split_axis]).unwrap();
            let value2 = NotNaN::new(p2.get_point()[split_axis]).unwrap();
            value1.cmp(&value2)
        });

        let middle_index = items.len() / 2;

        let left_items = items.get_mut(0..middle_index).unwrap();
        let left_bounds = {
            let mut bounds = BoundingBox::empty();
            for item in left_items {
                bounds = bounds.with_point(&item.get_point());
            }
            bounds
        };

        let right_items = items.get_mut((middle_index + 1)..items.len()).unwrap();
        let right_bounds = {
            let mut bounds = BoundingBox::empty();
            for item in right_items {
                bounds = bounds.with_point(&item.get_point());
            }
            bounds
        };

        Node::Internal(
            Arc::clone(&items[middle_index]),
            items[middle_index].get_point(),
            split_axis,
            Box::new(recursively_build_tree(left_items, left_bounds)),
            Box::new(recursively_build_tree(right_items, node_bounds)),
        )
    }
}

impl <T: Pointable> Node<T> {
    fn size(&self) -> usize {
        match self {
            &Node::Internal(_, _, _, ref left, ref right) => {
                left.size() + right.size()
            },
            &Node::Leaf(_) => {
                1
            },
            &Node::Empty => {
                0
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
            &Node::Leaf(ref object) => {
                write!(f, "{}leaf at {:?}\n", " ".repeat(indent_level * 2), object.get_point())
            },
            &Node::Empty => {},
        }
    }
}

impl <T: Pointable> Debug for PointKdTree<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.root.fmt_indented(f, 0)
    }
}
