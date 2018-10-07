use std::sync::Arc;
use std::fmt::{ Debug, Formatter, Result };
use std::collections::BinaryHeap;
use std::cmp::{ Ordering, PartialEq, Eq, PartialOrd, Ord };
use ordered_float::NotNaN;
use math::*;
use core::*;

pub trait Pointable: Debug + Send + Sync {
  fn get_point(&self) -> Point;
}

enum Node<T: Pointable> {
    // TODO: Should these be Box, not Arc?
    Internal(Arc<T>, Point, Axis, Box<Node<T>>, Box<Node<T>>),
    Leaf(Arc<T>, Point),
    Empty,
}

pub struct PointKdTree<T: Pointable>(Node<T>);

impl <T: Pointable> PointKdTree<T> {
    pub fn from(items: Vec<T>) -> PointKdTree<T> {
        let mut arc_items: Vec<Arc<T>> = items
            .into_iter()
            .map(|i| Arc::new(i))
            .collect();

        PointKdTree(recursively_build_tree(&mut arc_items))
    }

    pub fn k_nearest(&self, point: Point, k: usize) -> Vec<Arc<T>> {
        find_k_nearest(point, k, &self.0)
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
            .fold(BoundingBox::empty(), |unioned_bounds, ref item| BoundingBox::with_point(&unioned_bounds, &item.get_point()))
            .maximum_extent();

        // TODO: Partitioning can be done in O(n) rather than O(nlgn)
        items.sort_unstable_by(|ref p1, ref p2| {
            let value1 = NotNaN::new(p1.get_point()[split_axis]).unwrap();
            let value2 = NotNaN::new(p2.get_point()[split_axis]).unwrap();
            value1.cmp(&value2)
        });

        let middle_index = items.len() / 2;
        let item = Arc::clone(&items[middle_index]);
        let point = item.get_point();

        let (left_items, middle_and_right_items) = items.split_at_mut(middle_index);
        let right_items = if let Some((_, rest)) = middle_and_right_items.split_first_mut() {
            rest
        } else {
            &mut[]
        };

        Node::Internal(
            item,
            point,
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

fn find_k_nearest<T: Pointable>(target_point: Point, k: usize, root: &Node<T>) -> Vec<Arc<T>> {
    let mut found_items = BinaryHeap::<SearchNode<T>>::new();
    let mut search_stack = vec![root];

    let maybe_add_point = |found_items: &mut BinaryHeap<SearchNode<T>>, item: &Arc<T>, point: Point| {
        let squared_distance = NotNaN::new((point - target_point).magnitude2()).unwrap();
        if found_items.len() < k || found_items.peek().unwrap().1 > squared_distance {
            if found_items.len() == k {
                found_items.pop();
            }
            found_items.push(SearchNode(Arc::clone(item), squared_distance));
        }
    };

    let farthest_crosses_splitting_plane = |found_items: &BinaryHeap<SearchNode<T>>, splitting_point: Point, axis: Axis|
        found_items.peek().unwrap().1 >= NotNaN::new((splitting_point[axis] - target_point[axis]).powi(2)).unwrap();

    while let Some(node) = search_stack.pop() {
        match node {
            &Node::Internal(ref item, point, axis, ref left, ref right) => {
                if target_point[axis] < point[axis] {
                    search_stack.push(&left);
                    maybe_add_point(&mut found_items, item, point);
                    if farthest_crosses_splitting_plane(&found_items, point, axis) {
                        search_stack.push(&right);
                    }
                } else if target_point[axis] > point[axis] {
                    search_stack.push(&right);
                    maybe_add_point(&mut found_items, item, point);
                    if farthest_crosses_splitting_plane(&found_items, point, axis) {
                        search_stack.push(&left);
                    }
                } else {
                    maybe_add_point(&mut found_items, item, point);
                    search_stack.push(&left);
                    search_stack.push(&right);
                }
            },
            &Node::Leaf(ref item, point) => {
                maybe_add_point(&mut found_items, item, point);
            },
            &Node::Empty => {},
        };
    }

    found_items.into_iter().map(|SearchNode(item, _)| item).collect()
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
            &Node::Internal(_, ref point, ref axis, ref left, ref right) => {
                write!(f, "{}{} objects, split {:?} at {:?}\n",  " ".repeat(indent_level * 2), self.size(), *axis, point)?;
                left.fmt_indented(f, indent_level + 1)?;
                right.fmt_indented(f, indent_level + 1)
            },
            &Node::Leaf(ref object, _) => {
                write!(f, "{}leaf at {:?}\n", " ".repeat(indent_level * 2), object.get_point())
            },
            &Node::Empty => {
                write!(f, "{}empty\n", " ".repeat(indent_level * 2))
            },
        }
    }
}

impl <T: Pointable> Debug for PointKdTree<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.0.fmt_indented(f, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{ Rng, SeedableRng, StdRng };

    const TEST_RNG_SEED: [usize; 1] = [5];

    macro_rules! assert_xyz_eq {
        ($left:expr, $right:expr) => {
            assert_eq!($left.x, $right.x);
            assert_eq!($left.y, $right.y);
            assert_eq!($left.z, $right.z);
        };
    }

    impl Pointable for Point {
        fn get_point(&self) -> Point {
            *self
        }
    }

    impl Eq for Point {}

    impl PartialOrd for Point {
        fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
            if self.x != other.x {
                NotNaN::new(self.x).unwrap().partial_cmp(&NotNaN::new(other.x).unwrap())
            } else if self.y != other.y {
                NotNaN::new(self.y).unwrap().partial_cmp(&NotNaN::new(other.y).unwrap())
            } else {
                NotNaN::new(self.z).unwrap().partial_cmp(&NotNaN::new(other.z).unwrap())
            }
        }
    }

    impl Ord for Point {
        fn cmp(&self, other: &Point) -> Ordering {
            self.partial_cmp(&other).unwrap()
        }
    }

    fn random_point(rng: &mut Rng) -> Point {
        Point::new(rng.next_f64(), rng.next_f64(), rng.next_f64())
    }

    #[test]
    fn it_should_find_nearest_when_in_the_same_plane() {
        let tree = PointKdTree::from(vec![
            Point::new(0f64, 0f64, 0f64),
            Point::new(0f64, 0f64, 1f64),
            Point::new(0f64, 0f64, 2f64),
            Point::new(0f64, 0f64, 3f64),
            Point::new(0f64, 0f64, 4f64),
            Point::new(0f64, 0f64, 5f64),
            Point::new(0f64, 0f64, 6f64),
            Point::new(0f64, 0f64, 7f64),
        ]);

        let mut nearest = tree.k_nearest(Point::new(0f64, 0f64, 3.5), 2);

        assert_eq!(nearest.len(), 2);

        nearest.sort_unstable();

        assert_xyz_eq!(nearest[0], Point::new(0f64, 0f64, 3f64));
        assert_xyz_eq!(nearest[1], Point::new(0f64, 0f64, 4f64));
    }

    #[test]
    fn it_should_find_nearest_of_random_data() {
        let mut rng = StdRng::from_seed(&TEST_RNG_SEED);
        let k = 25;

        let points: Vec<Point> = (0..1000)
            .into_iter()
            .map(|_| random_point(&mut rng))
            .collect();

        let tree = PointKdTree::from(points.clone());

        let target_point = random_point(&mut rng);

        let mut nearest_actual = tree.k_nearest(target_point, k);
        nearest_actual.sort_unstable();

        let mut nearest_expected = points.clone();
        nearest_expected.sort_unstable_by_key(|&p| NotNaN::new((p - target_point).magnitude2()).unwrap());
        let mut nearest_expected = nearest_expected.split_at(k).0.to_vec();
        nearest_expected.sort_unstable();

        assert_eq!(nearest_actual.len(), nearest_expected.len());

        for i in 0..k {
            assert_xyz_eq!(nearest_actual[i], nearest_expected[i]);
        }
    }
}
