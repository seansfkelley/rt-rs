use std::f64;
use std::sync::Arc;
use std::fmt::{ Debug, Formatter, Result };
use ordered_float::NotNaN;
use core::*;
use math::*;

use rayon;

#[derive(Debug, PartialEq, Eq)]
enum EdgeType {
    Start,
    End,
}

enum Node<T: Geometry> {
    Internal(Axis, f64, Box<Node<T>>, Box<Node<T>>),
    // TODO: Should this be Box, not Arc?
    Leaf(Vec<Arc<T>>),
}

impl <T: Geometry> Node<T> {
    fn size(&self) -> usize {
        match self {
            &Node::Internal(_, _, ref left, ref right) => {
                left.size() + right.size()
            },
            &Node::Leaf(ref items) => {
                items.len()
            },
        }
    }

    fn fmt_indented(&self, f: &mut Formatter, indent_level: usize) -> Result {
        match self {
            &Node::Internal(ref axis, ref distance, ref left, ref right) => {
                write!(f, "{}{} objects, split {:?} at {}\n",  " ".repeat(indent_level * 2), self.size(), *axis, distance)?;
                left.fmt_indented(f, indent_level + 1)?;
                right.fmt_indented(f, indent_level + 1)
            },
            &Node::Leaf(ref items) => {
                write!(f, "{}{} objects\n", " ".repeat(indent_level * 2), items.len())
            },
        }
    }
}

// pbrt pg. 240
fn intersect<T: Geometry>(tree: &VolumeKdTree<T>, ray: Ray) -> Option<Intersection> {
    let (t_min_init, t_max_init) = match tree.bound.intersect(&ray) {
        Some((t0, t1)) => (t0, t1),
        None => { return None; }
    };
    let mut node_stack = vec![(&tree.root, t_min_init, t_max_init)];
    let mut r = ray;
    let mut closest: Option<Intersection> = None;

    while node_stack.len() > 0 {
        let (node, t_min, t_max) = node_stack.pop().unwrap();
        if t_min < r.t_max {
            match node {
                &Node::Internal(axis, distance, ref left, ref right) => {
                    let origin_component = r.origin[axis];
                    let direction_component = r.direction[axis];
                    let (near, far) = if (origin_component < distance) || (origin_component == distance && direction_component <= 0f64) {
                        (left, right)
                    } else {
                        (right, left)
                    };

                    let t_plane = (distance - origin_component) / direction_component;
                    if t_plane > t_max || t_plane <= 0f64 {
                        // t_plane > t_max means we hit the plane outside the current node's bounds, so skip far.
                        // t_plane <= 0 is not because the starting point of the ray is significant, but because the
                        // sign tells us if we're pointing away from the plane and can skip far.
                        // Note that this automatically handles both infinities.
                        node_stack.push((near, t_min, t_max));
                    } else if t_plane < t_min {
                        // t_plane < t_min means we're poining towards the plane, but it's behind where we care about
                        // so skip near.
                        node_stack.push((far, t_min, t_max));
                    } else {
                        node_stack.push((far, t_plane, t_max));
                        node_stack.push((near, t_min, t_plane));
                    }
                },
                &Node::Leaf(ref items) => {
                    for item in items {
                        // TODO: Should we bother checking the bounding box before testing intersection?
                        match item.intersect(&r) {
                            Some(intersection) => {
                                r.t_max = intersection.distance;
                                closest = Some(intersection);
                            },
                            None => {},
                        }
                    }
                },
            }
        }
    }

    closest
}

pub struct VolumeKdTree<T: Geometry> {
    root: Node<T>,
    bound: BoundingBox,
}

fn surface_area(bound: &BoundingBox) -> f64 {
    let dimensions = bound.max - bound.min;
    2f64 * (dimensions.x * dimensions.y + dimensions.y * dimensions.z + dimensions.z * dimensions.x)
}

fn recursively_build_tree<T: Geometry>(items: Vec<(Arc<T>, BoundingBox)>, node_bounds: BoundingBox) -> Node<T> {
    const LEAF_THRESHOLD: usize = 5;
    const TRAVERSAL_COST: f64 = 1f64;
    const INTERSECTION_COST: f64 = 20f64;
    const EMPTY_BONUS: f64 = 0.5;

    if items.len() < LEAF_THRESHOLD {
        Node::Leaf(items.into_iter().map(|(i, _)| Arc::clone(&i)).collect())
    } else {
        let node_surface_area = surface_area(&node_bounds);

        let best_partition: Option<(Axis, f64)> = [Axis::X, Axis::Y, Axis::Z]
            .into_iter()
            .map(|axis| {
                let (node_min, node_max) = (node_bounds.min[*axis], node_bounds.max[*axis]);

                let mut partition_candidates: Vec<(NotNaN<f64>, EdgeType)> = items
                    .iter()
                    .flat_map(|&(_, ref bound)| {
                        vec![
                            (NotNaN::new(bound.min[*axis]).unwrap(), EdgeType::Start),
                            (NotNaN::new(bound.max[*axis]).unwrap(), EdgeType::End),
                        ].into_iter()
                    })
                    .collect();

                assert_eq!(partition_candidates.len(), items.len() * 2);

                partition_candidates.sort_unstable_by(|ref p1, ref p2| p1.0.cmp(&p2.0));

                let mut left_count = 0;
                let mut right_count = items.len();

                let best_candidate = partition_candidates
                    .into_iter()
                    .map(|p| (p.0.into_inner(), p.1))
                    .fold(vec![], |mut coalesced: Vec<(f64, usize, usize)>, candidate| {
                        let (start_increment, end_increment): (usize, usize) = if candidate.1 == EdgeType::Start { (1, 0) } else { (0, 1) };
                        if coalesced.len() == 0 {
                            coalesced.push((candidate.0, start_increment, end_increment));
                        } else {
                            let last_index = coalesced.len() - 1;
                            let last = coalesced[last_index];
                            if last.0 == candidate.0 {
                                coalesced[last_index] = (last.0, last.1 + start_increment, last.2 + end_increment);
                            } else {
                                coalesced.push((candidate.0, start_increment, end_increment));
                            }
                        }
                        coalesced
                    })
                    .into_iter()
                    .filter_map(|(distance, start_count, end_count)| {
                        right_count -= end_count;
                        // Note that these are strict comparisons, since the case where distance == bounds is
                        // degenerate and useless (one partition will be zero-width which doesn't actually save work).
                        let candidate = if node_min < distance && distance < node_max {
                            let mut left_bounds = node_bounds.clone();
                            left_bounds.max[*axis] = distance;
                            let mut right_bounds = node_bounds.clone();
                            right_bounds.min[*axis] = distance;
                            let bonus_multiplier = 1f64 - (if left_count == 0 || right_count == 0 { EMPTY_BONUS } else { 0f64 });
                            let cost = TRAVERSAL_COST + INTERSECTION_COST * bonus_multiplier * (
                                surface_area(&left_bounds) * left_count as f64 / node_surface_area +
                                surface_area(&right_bounds) * right_count  as f64 / node_surface_area
                            );
                            Some((distance, NotNaN::new(cost).unwrap()))
                        } else {
                            None
                        };
                        left_count += start_count;
                        candidate
                    })
                    .min_by_key(|&(_, cost)| cost)
                    .map(|(distance, cost)| (axis, distance, cost));

                assert_eq!(left_count, items.len());
                assert_eq!(right_count, 0);

                best_candidate
            })
            .filter_map(|o| o)
            .min_by_key(|&(_, _, cost)| cost)
            .map(|(axis, distance, _)| (*axis, distance));

        match best_partition {
            Some((axis, distance)) => {
                let mut left_items: Vec<(Arc<T>, BoundingBox)> = vec![];
                let mut right_items: Vec<(Arc<T>, BoundingBox)> = vec![];
                for &(ref item, ref bound) in &items {
                    let in_plane = bound.min[axis] == distance && distance == bound.max[axis];
                    let mut did_add = false;
                    if bound.min[axis] < distance || in_plane {
                        left_items.push((Arc::clone(item), bound.clone()));
                        did_add = true;
                    }
                    if bound.max[axis] > distance || in_plane {
                        right_items.push((Arc::clone(item), bound.clone()));
                        did_add = true;
                    }
                    assert!(did_add);
                }
                let (left, right, total) = (left_items.len(), right_items.len(), items.len());
                // TODO: This is the kind of thing that should never be generated by the heuristic, perhaps?
                if (left == total && right > 0) || (right == total && left > 0) || (left + right) as f64 / total as f64 > 1.8 {
                    Node::Leaf(items.into_iter().map(|(i, _)| Arc::clone(&i)).collect())
                } else {
                    let mut left_bounds = node_bounds.clone();
                    left_bounds.max[axis] = distance;
                    let mut right_bounds = node_bounds.clone();
                    right_bounds.min[axis] = distance;
                    let (left_node, right_node) = rayon::join(
                        move || recursively_build_tree(left_items, left_bounds),
                        move || recursively_build_tree(right_items, right_bounds),
                    );
                    Node::Internal(
                        axis,
                        distance,
                        Box::new(left_node),
                        Box::new(right_node),
                    )
                }
            },
            None => {
                Node::Leaf(items.into_iter().map(|(i, _)| Arc::clone(&i)).collect())
            },
        }
    }
}

impl <T: Geometry> VolumeKdTree<T> {
    pub fn from(items: Vec<T>) -> VolumeKdTree<T> {
        let pairs: Vec<(Arc<T>, BoundingBox)> = items
            .into_iter()
            .map(|i| {
                let bound = i.bound();
                (Arc::new(i), bound)
            })
            .collect();

        let tree_bound = pairs
            .iter()
            .fold(BoundingBox::empty(), |unioned_bounds, &(_, ref bound)| BoundingBox::union(&unioned_bounds, bound));

        VolumeKdTree {
            root: recursively_build_tree(pairs, tree_bound.clone()),
            bound: tree_bound,
        }
    }
}

impl <T: Geometry> Geometry for VolumeKdTree<T> {
    fn bound(&self) -> BoundingBox {
        self.bound.clone()
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        intersect(&self, ray.clone())
    }

    fn does_intersect(&self, ray: &Ray) -> bool {
        // TODO: Actually implement a separate method for this.
        self.intersect(ray).is_some()
    }
}

impl <T: Geometry> Debug for VolumeKdTree<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.root.fmt_indented(f, 0)
    }
}
