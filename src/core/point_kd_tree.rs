use std::sync::Arc;
use std::fmt::{ Debug, Formatter, Result };
use std::collections::BinaryHeap;
use std::cmp::{ Ordering, PartialEq, Eq, PartialOrd, Ord };
use ordered_float::NotNaN;
use math::*;
use core::*;

trait Pointable: Debug + Send + Sync {
  fn get_point(&self) -> Point;
}

enum Node<T: Pointable> {
    Internal(Arc<T>, Point, Axis, Box<Node<T>>, Box<Node<T>>),
    Leaf(Arc<T>, Point),
    Empty,
}

pub struct PointKdTree<T: Pointable>(Node<T>);

impl <T: Pointable> PointKdTree<T> {
    pub fn from(items: Vec<T>) -> PointKdTree<T> {
        let arc_items: Vec<Arc<T>> = items
            .into_iter()
            .map(|i| Arc::new(i))
            .collect();

        PointKdTree(recursively_build_tree(&mut arc_items))
    }

    pub fn k_nearest(&self, point: Point, k: usize) -> Vec<Arc<T>> {
        let mut found_items = BinaryHeap::new();
        find_k_nearest(point, k, &self.0, &mut found_items);
        found_items.into_vec().into_iter().map(|(item, _)| item).collect()
    }
}

fn recursively_build_tree<T: Pointable>(items: &mut [Arc<T>]) -> Node<T> {
    if items.len() == 0 {
        Node::Empty
    } else if items.len() == 1 {
        let item = items.first().unwrap();
        Node::Leaf(Arc::clone(item), item.get_point())
    } else {
        let split_axis = items
            .iter()
            .fold(BoundingBox::empty(), |unioned_bounds, &item| BoundingBox::with_point(&unioned_bounds, &item.get_point()))
            .maximum_extent();

        // TODO: Partitioning can be done in O(n) rather than O(nlgn)
        items.sort_unstable_by(|ref p1, ref p2| {
            let value1 = NotNaN::new(p1.get_point()[split_axis]).unwrap();
            let value2 = NotNaN::new(p2.get_point()[split_axis]).unwrap();
            value1.cmp(&value2)
        });

        let middle_index = items.len() / 2;

        let left_items = items.get_mut(0..middle_index).unwrap();
        let right_items = items.get_mut((middle_index + 1)..items.len()).unwrap();

        Node::Internal(
            Arc::clone(&items[middle_index]),
            items[middle_index].get_point(),
            split_axis,
            Box::new(recursively_build_tree(left_items)),
            Box::new(recursively_build_tree(right_items)),
        )
    }
}

struct SearchNode<T: Pointable>(Arc<T>, NotNaN<f64>);

impl <T: Pointable + Sized> PartialEq for SearchNode<T> {
    fn eq(&self, other: &SearchNode<T>) -> bool {
        self.1 == other.1
    }
}

impl <T: Pointable + Sized> Eq for SearchNode<T> {}

impl <T: Pointable + Sized> PartialOrd for SearchNode<T> {
    fn partial_cmp(&self, other: &SearchNode<T>) -> Option<Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl <T: Pointable + Sized> Ord for SearchNode<T> {
    fn cmp(&self, other: &SearchNode<T>) -> Ordering {
        self.1.cmp(&other.1)
    }
}

// TODO: Implement with stack rather than recursion?
fn find_k_nearest<T: Pointable>(target_point: Point, k: usize, node: &Node<T>, found_items: &mut BinaryHeap<SearchNode<T>>) {
    let maybe_add_point = |item: Arc<T>, point: Point| {
        let squared_distance = NotNaN::new((point - target_point).magnitude2()).unwrap();
        if found_items.len() < k || found_items.peek().unwrap().1 > squared_distance {
            if found_items.len() == k {
                found_items.pop();
            }
            found_items.push(SearchNode(item, squared_distance));
        }
    };

    let maybe_recurse = |splitting_point: Point, axis: Axis, child_node: &Node<T>| {
        if found_items.peek().unwrap().1 >= NotNaN::new((splitting_point[axis] - target_point[axis]).powi(2)).unwrap() {
            find_k_nearest(target_point, k, child_node, found_items);
        }
    };

    match node {
        &Node::Internal(item, point, axis, left, right) => {
            if target_point[axis] < point[axis] {
                find_k_nearest(target_point, k, &left, found_items);
                maybe_add_point(item, point);
                maybe_recurse(point, axis, &right);
            } else if target_point[axis] > point[axis] {
                find_k_nearest(target_point, k, &right, found_items);
                maybe_add_point(item, point);
                maybe_recurse(point, axis, &left);
            } else {
                maybe_add_point(item, point);
                find_k_nearest(target_point, k, &left, found_items);
                find_k_nearest(target_point, k, &right, found_items);
            }
        },
        &Node::Leaf(item, point) => {
            maybe_add_point(item, point);
        },
        &Node::Empty => {},
    }
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
            &Node::Leaf(ref object, _) => {
                write!(f, "{}leaf at {:?}\n", " ".repeat(indent_level * 2), object.get_point())
            },
            &Node::Empty => {
                Result::Ok
            },
        }
    }
}

impl <T: Pointable> Debug for PointKdTree<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.0.fmt_indented(f, 0)
    }
}
