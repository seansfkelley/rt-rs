use std::f64;
use std::collections::VecDeque;
use super::ray::Ray;
use super::bounding_box::Bounded;

enum Axis {
    X,
    Y,
    Z,
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

impl <'a, T: Bounded> KdTree<T> {
    pub fn from(items: Vec<T>) -> KdTree<T> {
        KdTree {
            tree: Node::Leaf(items)
        }
    }

    pub fn intersects(&'a self, ray: &'a Ray) -> TreeIterator<'a, T> {
        TreeIterator::new(ray, &self.tree)
    }
}
