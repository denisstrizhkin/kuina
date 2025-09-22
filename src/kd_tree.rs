use std::{collections::VecDeque, marker::PhantomData};

pub trait Point<T, const K: usize>
where
    T: Ord,
{
    type Output;
    fn get(&self, index: usize) -> Self::Output;
}

pub struct KDTree<T, P, const K: usize, const N: usize>
where
    P: Point<T, K>,
    T: Ord,
{
    points: [P; N],
    root: Node,
    _m: PhantomData<T>,
}

impl<T, P, const K: usize, const N: usize> From<[P; N]> for KDTree<T, P, K, N>
where
    P: Point<T, K>,
    T: Ord,
{
    fn from(value: [P; N]) -> Self {
        let mut points = value;
        let root = Node {
            bound_right: N,
            axis: 0,
            ..Default::default()
        };
        let mut queue = VecDeque::new();
        queue.push_back(&root);
        // while let Some(node) = queue.pop_front() {
        //     points[node.bound_left..node.bound_right].sort_by(|a, b| a.get(0).cmp(&b.get(0)));
        //     node.index = (node.bound_right - node.bound_left);
        // }
        Self {
            points,
            root,
            _m: Default::default(),
        }
    }
}

impl<T, P, const K: usize, const N: usize> KDTree<T, P, K, N>
where
    P: Point<T, K>,
    T: Ord,
{
    fn insert(&mut self) {}
}

#[derive(Default)]
struct Node {
    index: usize,
    bound_left: usize,
    bound_right: usize,
    axis: usize,
    left: Option<usize>,
    right: Option<usize>,
}
