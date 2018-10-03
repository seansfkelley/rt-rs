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
    Leaf(Arc<T>),
}

pub struct VolumeKdTree<T: Pointable> {
    root: Node<T>,
    bound: BoundingBox,
}

fn recursively_build_tree<T: Pointable>(items: Vec<Arc<T>>) -> Node<T> {
    if items.len() == 0 {
        Node::Leaf(Arc::clone(items.first().unwrap()))
    } else {
        let best_partition: (Axis, f64) = [Axis::X, Axis::Y, Axis::Z]
            .into_iter()
            .map(|axis| {
                let axis_index = *axis as usize;

                let mut partition_candidates: Vec<f64> = items
                    .iter()
                    .map(|ref item| item.get_point[axis_index])
                    .collect();

                partition_candidates.sort_unstable();

                let (distance, count) = partition_candidates
                    .into_iter()
                    .fold(vec![], |mut coalesced: Vec<(f64, usize)>, candidate| {
                        if coalesced.len() == 0 {
                            coalesced.push((candidate, 1));
                        } else {
                            let last_index = coalesced.len() - 1;
                            let last = coalesced[last_index];
                            if last.0 == candidate {
                                coalesced[last_index] = (last.0, last.1 + 1);
                            } else {
                                coalesced.push((candidate, last.1 + 1));
                            }
                        }
                        coalesced
                    })
                    .into_iter()
                    .find(|(_, count)| count >= items.len() / 2)
                    .unwrap();

                (axis, distance, items.len() - count)
            })
            .min_by_key(|&(_, _, cost)| cost)
            .map(|(axis, distance, _)| (*axis, distance));

        // TODO: Split into two using the proper comparison operator.
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
        }
    }
}

impl <T: Pointable> Debug for VolumeKdTree<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.root.fmt_indented(f, 0)
    }
}
