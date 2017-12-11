use std::f64;
use std::rc::Rc;
use std::collections::{ VecDeque, HashSet };
use std::fmt::{ Debug, Formatter, Result };
use ordered_float::NotNaN;
use lazysort::SortedBy;
use super::ray::Ray;
use super::bounding_box::{ Bounded, BoundingBox };

#[derive(Debug, Clone, Copy)]
enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

enum Node<T: Bounded> {
    Internal(Axis, f64, Box<Node<T>>, Box<Node<T>>),
    Leaf(Vec<Rc<T>>),
}

impl <T: Bounded> Node<T> {
    fn fmt_indented(&self, f: &mut Formatter, indent_level: usize) -> Result {
        match self {
            &Node::Internal(ref axis, ref distance, ref left, ref right) => {
                write!(f, "{}split {:?}@{}\n",  " ".repeat(indent_level * 2), *axis, distance)?;
                left.fmt_indented(f, indent_level + 1)?;
                right.fmt_indented(f, indent_level + 1)
            },
            &Node::Leaf(ref items) => {
                write!(f, "{}{} nodes\n", " ".repeat(indent_level * 2), items.len())
            },
        }
    }
}

pub struct TreeIterator<'a, T: Bounded + 'a> {
    ray: &'a Ray,
    node_queue: VecDeque<&'a Node<T>>,
    current_leaf_contents: Option<(&'a Vec<Rc<T>>, usize)>,
}

impl <'a, T: Bounded + 'a> TreeIterator<'a, T> {
    fn new(ray: &'a Ray, root: &'a Node<T>) -> TreeIterator<'a, T> {
        let mut node_queue = VecDeque::new();
        node_queue.push_back(root);
        TreeIterator {
            ray,
            node_queue,
            current_leaf_contents: None,
        }
    }
}

impl <'a, T: Bounded> Iterator for TreeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.current_leaf_contents {
            Some((items, mut index)) => {
                while index < items.len() {
                    let item = &items[index];
                    if item.bound().intersect(self.ray) {
                        self.current_leaf_contents = Some((items, index + 1));
                        return Some(&items[index]);
                    } else {
                        index += 1;
                    }
                }
            },
            None => {},
        }

        match self.node_queue.pop_front() {
            Some(node) => {
                match node {
                    &Node::Internal(axis, distance, ref left, ref right) => {
                        let axis_index = axis as usize;
                        let direction_component = self.ray.direction[axis_index];
                        let origin_component = self.ray.origin[axis_index];
                        if direction_component != 0f64 && (distance - origin_component) / direction_component > 0f64 {
                            self.node_queue.push_back(&*left);
                            self.node_queue.push_back(&*right);
                        } else {
                            if origin_component < distance {
                                self.node_queue.push_back(&*left);
                            } else {
                                self.node_queue.push_back(&*right);
                            }
                        }
                        self.next()
                    },
                    &Node::Leaf(ref items) => {
                        self.current_leaf_contents = Some((items, 0usize));
                        self.next()
                    }
                }
            },
            None => None,
        }
    }
}

// TODO: Consider newtype.
pub struct KdTree<T: Bounded> {
    tree: Node<T>,
}

const LEAF_THRESHOLD: usize = 5;
const TRAVERSAL_COST: f64 = 2.5;
const INTERSECTION_COST: f64 = 0.9;

fn surface_area(bound: &BoundingBox) -> f64 {
    let dimensions = bound.max - bound.min;
    2f64 * (dimensions.x * dimensions.y + dimensions.y * dimensions.z + dimensions.z * dimensions.x)
}

fn recursively_build_tree<T: Bounded>(items: Vec<(Rc<T>, BoundingBox)>) -> Node<T> {
    if items.len() < LEAF_THRESHOLD {
        Node::Leaf(items.into_iter().map(|(i, _)| Rc::clone(&i)).collect())
    } else {
        let node_bounds = items
            .iter()
            .fold(BoundingBox::empty(), |unioned_bounds, &(_, ref bound)| BoundingBox::union(&unioned_bounds, bound));
        let node_surface_area = surface_area(&node_bounds);

        // TODO: This algorithm is n^2! There are papers on this topic to read.
        let best_partition: Option<(Axis, f64)> = [Axis::X, Axis::Y, Axis::Z]
            .into_iter()
            .map(|axis| {
                let axis_index = *axis as usize;

                let mut partition_candidates: HashSet<NotNaN<f64>> = HashSet::new();
                for &(_, ref bound) in &items {
                    let (candidate0, candidate1) = (bound.min[axis_index], bound.max[axis_index]);
                    let (min, max) = (node_bounds.min[axis_index], node_bounds.max[axis_index]);
                    if min <= candidate0 && candidate0 <= max {
                        partition_candidates.insert(NotNaN::new(candidate0).unwrap());
                    }
                    if min <= candidate1 && candidate1 <= max {
                        partition_candidates.insert(NotNaN::new(candidate1).unwrap());
                    }
                }

                if partition_candidates.len() > 0 {
                    partition_candidates
                        .into_iter()
                        .map(|d| d.into_inner())
                        .map(|d| {
                            let left_count = items.iter().filter(|&&(_, ref bound)| bound.min[axis_index] <= d).count();
                            let right_count = items.iter().filter(|&&(_, ref bound)| bound.max[axis_index] >= d).count();
                            let mut left_bounds = node_bounds.clone();
                            left_bounds.max[axis_index] = d;
                            let mut right_bounds = node_bounds.clone();
                            right_bounds.min[axis_index] = d;
                            let cost = TRAVERSAL_COST + INTERSECTION_COST * (
                                surface_area(&left_bounds) * left_count as f64 / node_surface_area +
                                surface_area(&right_bounds) * right_count  as f64 / node_surface_area
                            );
                            (d, NotNaN::new(cost).unwrap())
                        })
                        .sorted_by(|&(_, a), &(_, b)| a.cmp(&b))
                        .nth(0)
                        .map(|(distance, cost)| (axis, distance, cost))
                } else {
                    None
                }
            })
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .sorted_by(|&(_, _, a), &(_, _, b)| a.cmp(&b))
            .nth(0)
            .map(|(axis, distance, _)| (*axis, distance));

        match best_partition {
            Some((axis, distance)) => {
                let axis_index = axis as usize;
                let mut left_items: Vec<(Rc<T>, BoundingBox)> = vec![];
                let mut right_items: Vec<(Rc<T>, BoundingBox)> = vec![];
                // TODO: Putting a reference in each side means that we might try to intersect the same object twice sometimes!
                for &(ref item, ref bound) in &items {
                    if bound.min[axis_index] <= distance {
                        left_items.push((Rc::clone(item), bound.clone()));
                    }
                    if bound.max[axis_index] >= distance {
                        right_items.push((Rc::clone(item), bound.clone()));
                    }
                }
                let (left, right, total) = (left_items.len(), right_items.len(), items.len());
                // TODO: This is the kind of thing that should never be generated by the heuristic, perhaps?
                if left == total || right == total || (left + right) as f64 / total as f64 > 1.8 {
                    Node::Leaf(items.into_iter().map(|(i, _)| Rc::clone(&i)).collect())
                } else {
                    Node::Internal(axis, distance, Box::new(recursively_build_tree(left_items)), Box::new(recursively_build_tree(right_items)))
                }
            },
            None => {
                Node::Leaf(items.into_iter().map(|(i, _)| Rc::clone(&i)).collect())
            },
        }
    }
}

impl <'a, T: Bounded> KdTree<T> {
    pub fn from(items: Vec<T>) -> KdTree<T> {
        let pairs: Vec<(Rc<T>, BoundingBox)> = items.into_iter().map(|i| {
            let bound = i.bound();
            (Rc::new(i), bound)
        }).collect();
        KdTree {
            tree: recursively_build_tree(pairs)
        }
    }

    pub fn intersects(&'a self, ray: &'a Ray) -> TreeIterator<'a, T> {
        TreeIterator::new(ray, &self.tree)
    }
}

impl <T: Bounded> Debug for KdTree<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.tree.fmt_indented(f, 0)
    }
}
