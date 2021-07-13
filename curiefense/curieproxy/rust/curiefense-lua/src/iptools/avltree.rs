use core::mem::swap;
use std::cmp::max;
use std::cmp::Ordering;
use std::fmt::Debug;

type Tree<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
struct Node<K: Ord + Debug, V> {
    key: K,
    value: V,
    height: isize,
    left: Tree<K, V>,
    right: Tree<K, V>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct AVLTreeMap<K: Ord + Debug, V> {
    root: Tree<K, V>,
    size: usize,
}

#[derive(PartialEq)]
enum InsertSide {
    Left,
    Right,
    Middle,
}

impl std::fmt::Display for InsertSide {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InsertSide::Left => write!(f, "Left"),
            InsertSide::Right => write!(f, "Right"),
            InsertSide::Middle => write!(f, "Middle"),
        }
    }
}

impl<K, V> AVLTreeMap<K, V>
where
    K: Ord + Debug,
{
    pub fn new() -> AVLTreeMap<K, V> {
        AVLTreeMap { root: None, size: 0 }
    }

    pub fn dump(&self) {
        match &self.root {
            None => println!("Empty tree"),
            Some(node) => node.dump(0),
        }
    }
    pub fn insert(&mut self, key: K, value: V) -> bool {
        fn ins<K: Ord + Debug, V>(p_tree: &mut Tree<K, V>, key: K, value: V) -> (Option<InsertSide>, isize) {
            match p_tree {
                None => {
                    *p_tree = Some(Box::new(Node::new(key, value)));
                    (Some(InsertSide::Middle), 1)
                }
                Some(p_node) => match p_node.key.cmp(&key) {
                    Ordering::Less => {
                        let (side, h) = ins(&mut p_node.right, key, value);
                        if side.is_none() {
                            return (None, p_node.height);
                        }
                        p_node.height = max(p_node.height, h + 1);
                        p_node.rebalance(side.unwrap());
                        (Some(InsertSide::Right), p_node.height)
                    }
                    Ordering::Greater => {
                        let (side, h) = ins(&mut p_node.left, key, value);
                        if side.is_none() {
                            return (None, p_node.height);
                        }
                        p_node.height = max(p_node.height, h + 1);
                        p_node.rebalance(side.unwrap());
                        (Some(InsertSide::Left), p_node.height)
                    }
                    Ordering::Equal => (None, p_node.height),
                },
            }
        }
        let (inserted, _) = ins(&mut self.root, key, value);
        if inserted.is_some() {
            self.size += 1
        };
        inserted.is_some()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut p_tree = &self.root;
        while let Some(p_node) = p_tree {
            match p_node.key.cmp(&key) {
                Ordering::Less => p_tree = &p_node.right,
                Ordering::Greater => p_tree = &p_node.left,
                Ordering::Equal => {
                    return Some(&p_node.value);
                }
            }
        }
        None
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn get_custom(&self, key: &K, compare: fn(&K, &K) -> Ordering) -> Option<&V> {
        let mut p_tree = &self.root;
        while let Some(p_node) = p_tree {
            match compare(&p_node.key, &key) {
                Ordering::Less => p_tree = &p_node.right,
                Ordering::Greater => p_tree = &p_node.left,
                Ordering::Equal => {
                    return Some(&p_node.value);
                }
            }
        }
        None
    }

    pub fn contains_custom(&self, key: &K, compare: fn(&K, &K) -> Ordering) -> bool {
        self.get_custom(key, compare).is_some()
    }

    pub fn height(&self) -> usize {
        match &self.root {
            None => 0,
            Some(node) => node.height as usize,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

impl<K, V> Node<K, V>
where
    K: Ord + Debug,
{
    pub fn new(key: K, value: V) -> Node<K, V> {
        Node {
            key,
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn height_left(&self) -> isize {
        match &self.left {
            None => 0,
            Some(node) => node.height,
        }
    }

    fn height_right(&self) -> isize {
        match &self.right {
            None => 0,
            Some(node) => node.height,
        }
    }

    fn update_height(&mut self) {
        self.height = max(self.height_left(), self.height_right()) + 1
    }

    fn balance_factor(&self) -> isize {
        self.height_right() - self.height_left()
    }

    fn rebalance(&mut self, side: InsertSide) -> bool {
        match self.balance_factor() {
            -1 | 0 | 1 => false,
            -2 => {
                if side == InsertSide::Right {
                    self.left.as_mut().unwrap().rotate_left();
                }
                self.rotate_right()
            }
            2 => {
                if side == InsertSide::Left {
                    self.right.as_mut().unwrap().rotate_right();
                }
                self.rotate_left()
            }
            bf => {
                self.dump(0);
                panic!("Internal error: AVLTree balance factor={}. Should be in [-2;2]", bf);
            }
        }
    }

    //      D          B
    //     / \        / \
    //    B   E  =>  A   D
    //   / \            / \
    //  A   C          C   E
    fn rotate_right(&mut self) -> bool {
        if self.left.is_none() {
            return false;
        }
        let mut btree = self.left.take();
        let mut bnode = btree.as_mut().unwrap();
        let atree = bnode.left.take();
        let ctree = bnode.right.take();
        let etree = self.right.take();

        // D becomes B and B becomes D
        swap(&mut self.key, &mut bnode.key);
        swap(&mut self.value, &mut bnode.value);

        bnode.left = ctree;
        bnode.right = etree;
        bnode.update_height();

        self.left = atree;
        self.right = btree;

        self.update_height();
        true
    }

    //     B            D
    //    / \          / \
    //   A   D   =>   B   E
    //      / \      / \
    //     C   E    A   C
    fn rotate_left(&mut self) -> bool {
        if self.right.is_none() {
            return false;
        }
        let mut dtree = self.right.take();
        let mut dnode = dtree.as_mut().unwrap();
        let ctree = dnode.left.take();
        let etree = dnode.right.take();
        let atree = self.left.take();

        // D becomes B and B becomes D
        swap(&mut self.key, &mut dnode.key);
        swap(&mut self.value, &mut dnode.value);

        dnode.right = ctree;
        dnode.left = atree;
        dnode.update_height();

        self.right = etree;
        self.left = dtree;
        self.update_height();
        true
    }
    fn dump(&self, n: usize) {
        println!("{: <1$}({:0})[{:?}]", "", n, self.key);
        if self.left.is_some() {
            self.left.as_ref().unwrap().dump(n + 2)
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().dump(n + 2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_insert() {
        let mut t = AVLTreeMap::new();
        assert!(!t.contains(&2));
        assert!(t.insert(2, "hello"));
        assert!(t.contains(&2));
        assert!(!t.insert(2, "hello"));
        assert!(!t.contains(&3));
        assert!(t.insert(3, "hello"));
        assert!(t.contains(&3));
        assert!(!t.contains(&4));
    }
    #[test]
    fn test_len() {
        let mut t = AVLTreeMap::new();
        assert!(t.len() == 0);
        assert!(t.insert(&1, "hello 1"));
        assert!(t.len() == 1);
        assert!(t.insert(&2, "hello 2"));
        assert!(t.len() == 2);
        assert!(!t.insert(&1, "hello 3"));
        assert!(t.len() == 2);
    }
    #[test]
    fn test_balance() {
        let mut t = AVLTreeMap::new();
        println!("start: height={} tree={:?}", t.height(), t);
        assert!(t.height() == 0);
        assert!(t.insert(&1, "hello"));
        println!("add 1: height={} tree={:?}", t.height(), t);
        assert!(t.height() == 1);
        assert!(t.insert(&2, "hello"));
        println!("add 2: height={} tree={:?}", t.height(), t);
        assert!(t.height() == 2);
        assert!(t.insert(&3, "hello"));
        println!("add 3: height={} tree={:?}", t.height(), t);
        assert!(t.height() == 2);
        assert!(t.insert(&4, "hello"));
        println!("add 4: height={} tree={:?}", t.height(), t);
        assert!(t.height() == 3);
        assert!(t.insert(&5, "hello"));
        println!("add 5: height={} tree={:?}", t.height(), t);
        assert!(t.height() == 3);
        assert!(t.insert(&6, "hello"));
        println!("add 6: height={} tree={:?}", t.height(), t);
        assert!(t.height() == 3);
        assert!(t.insert(&7, "hello"));
        println!("add 7: height={} tree={:?}", t.height(), t);
        assert!(t.height() == 3);
        assert!(t.insert(&8, "hello"));
        println!("add 8: height={} tree={:?}", t.height(), t);
        assert!(t.height() == 4);
    }

    #[test]
    fn test_balance2() {
        let mut t = AVLTreeMap::new();
        for i in 0..1000000 {
            assert!(t.insert(i, "hello"));
            assert!(t.len() == i + 1);
            let h: usize = (64 - (i + 1).leading_zeros()) as usize;
            assert!(t.height() == h);
        }
    }
    #[test]
    fn test_balance3() {
        let mut t = AVLTreeMap::new();
        for i in 0..1000000 {
            assert!(t.insert(10000000 - i, "hello"));
            assert!(t.len() == i + 1);
            let h: usize = (64 - (i + 1).leading_zeros()) as usize;
            assert!(t.height() == h);
        }
    }
    #[test]
    fn test_balance4() {
        let mut rng = rand::thread_rng();
        let mut t = AVLTreeMap::new();
        let mut cnt = 0;
        for _i in 0..1000000 {
            let val = rng.gen_range(0..10000);
            println!("###### Inserting {}", val);
            if !t.contains(&val) {
                cnt += 1
            };
            t.insert(val, "hello");
            assert!(cnt == t.len());
        }
    }
}
