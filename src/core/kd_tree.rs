use std::f64;
use std::collections::{ VecDeque, HashSet };
use ordered_float::NotNaN;
use lazysort::SortedBy;
use super::ray::Ray;
use super::bounding_box::{ Bounded, BoundingBox };

enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

enum Node<T: Bounded> {
    Internal(Axis, f64, Box<Node<T>>, Box<Node<T>>),
    Leaf(Vec<T>),
}

pub struct TreeIterator<'a, T: Bounded + 'a> {
    ray: &'a Ray,
    node_queue: VecDeque<&'a Node<T>>,
    current_leaf_contents: Option<(&'a Vec<T>, usize)>,
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
                    &Node::Internal(_, _, ref left, ref right) => {
                        self.node_queue.push_back(&*left);
                        self.node_queue.push_back(&*right);
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

const LEAF_THRESHOLD: usize = 10;
const TRAVERSAL_COST: f64 = 2.5;
const INTERSECTION_COST: f64 = 0.9;

fn surface_area(bound: &BoundingBox) -> f64 {
    let dimensions = bound.max - bound.min;
    2f64 * (dimensions.x * dimensions.y + dimensions.y * dimensions.z + dimensions.z * dimensions.x)
}

fn recursively_build_tree<T: Bounded>(items: Vec<(T, BoundingBox)>) -> Node<T> {
    if items.len() < LEAF_THRESHOLD {
        Node::Leaf(items.into_iter().map(|(i, _)| i).collect::<Vec<T>>())
    } else {
        let node_bounds = items
            .iter()
            .fold(BoundingBox::empty(), |unioned_bounds, &(_, bound)| BoundingBox::union(&unioned_bounds, &bound));
        let node_surface_area = surface_area(&node_bounds);

        // TODO: This algorithm is n^2! There are papers on this topic to read.
        const best_partition: Option<(Axis, NotNaN<f64>, NotNaN<f64>)> = [Axis::X, Axis::Y, Axis::Z]
            .into_iter()
            .map(|axis| {
                items;
                // let axis_index = *axis as usize;


                // let partition_candidates = items
                //     .iter()
                //     .flat_map(|&(_, ref bound)| vec![
                //         NotNaN::new(bound.min[axis_index]).unwrap(),
                //         NotNaN::new(bound.max[axis_index]).unwrap(),
                //     ].into_iter())
                //     .filter(|d| d >= node_bounds.min[axis_index] && d <= node_bounds.max[axis_index])
                //     .collect::<HashSet<NotNaN<f64>>>();
                //
                // if partition_candidates.len() > 0 {
                //     partition_candidates
                //         .into_iter()
                //         .map(|d| {
                //             let left_count = items.iter().filter(|&(_, bound)| bound.min[axis_index] <= d).count();
                //             let right_count = items.iter().filter(|&(_, bound)| bound.max[axis_index] >= d).count();
                //             let mut left_bounds = node_bounds.clone();
                //             left_bounds.max[axis_index] = d;
                //             let mut right_bounds = node_bounds.clone();
                //             right_bounds.min[axis_index] = d;
                //             let cost = TRAVERSAL_COST + INTERSECTION_COST * (
                //                 surface_area(left_bounds) * left_count / node_surface_area +
                //                 surface_area(right_bounds) * right_count / node_surface_area
                //             );
                //             (d.into_inner(), NotNaN::new(cost))
                //         })
                //         .sorted_by(|(_, a), (_, b)| a.cmp(b))
                //         .nth(0)
                //         .map(|(distance, cost)| (axis, distance, cost))
                // } else {
                //     None
                // }
                None
            })
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .sorted_by(|&(_, _, a), &(_, _, b)| a.cmp(b))
            .nth(0)
            .map(|(axis, distance, cost)| (axis, distance, cost.into_inner()));

        Node::Leaf(items.into_iter().map(|(i, _)| i).collect::<Vec<T>>())
    }
}

impl <'a, T: Bounded> KdTree<T> {
    pub fn from(items: Vec<T>) -> KdTree<T> {
        let pairs: Vec<(T, BoundingBox)> = items.into_iter().map(|i| {
            let bound = i.bound();
            (i, bound)
        }).collect();
        KdTree {
            tree: recursively_build_tree(pairs)
        }
    }

    pub fn intersects(&'a self, ray: &'a Ray) -> TreeIterator<'a, T> {
        TreeIterator::new(ray, &self.tree)
    }
}
