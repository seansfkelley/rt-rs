use std::f64;
use std::collections::VecDeque;
use super::ray::Ray;
use super::bounding_box::Boundable;

enum Axis {
    X,
    Y,
    Z,
}

enum Node<T: Boundable> {
    Internal(Axis, f64, Box<Node<T>>, Box<Node<T>>),
    Leaf(Vec<T>),
}

struct TreeIterator<'a, T: Boundable + 'a> {
    node_queue: VecDeque<&'a Node<T>>,
    item_queue: VecDeque<&'a T>,
}

impl <'a, T: Boundable + 'a> TreeIterator<'a, T> {
    fn new(root: &'a Node<T>) -> TreeIterator<'a, T> {
        let node_queue = VecDeque::new();
        node_queue.push_back(root);
        TreeIterator {
            node_queue,
            item_queue: VecDeque::new(),
        }
    }
}

impl <'a, T: Boundable> Iterator for TreeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.item_queue.pop_front() {
            Some(item) => Some(item),
            None => {
                match self.node_queue.pop_front() {
                    Some(node) => {
                        match node {
                            &Node::Internal(_, _, left, right) => {
                                self.node_queue.push_back(&*left);
                                self.node_queue.push_back(&*right);
                                self.next()
                            },
                            &Node::Leaf(items) => {
                                for i in items {
                                    self.item_queue.push_back(&i);
                                }
                                self.next()
                            }
                        }
                    },
                    None => None
                }
            }
        }
    }
}

// TODO: Consider newtype.
pub struct KdTree<T: Boundable> {
    tree: Node<T>,
}

impl <'a, T: Boundable> KdTree<T> {
    pub fn from(items: Vec<T>) -> KdTree<T> {
        KdTree {
            tree: Node::Leaf(items)
        }
    }

    pub fn intersects(&self, ray: &Ray) -> TreeIterator<'a, T> {
        TreeIterator::new(&self.tree)
    }
}
