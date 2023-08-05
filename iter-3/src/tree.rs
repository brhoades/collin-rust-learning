use std::fmt::Debug;

/// TreeMap is a map whose iteration is ordered on the keys.
pub enum TreeMap<K, V> {
    Empty,
    NonEmpty(TreeNode<K, V>),
}

impl<K: Ord, V: PartialEq> TreeMap<K, V> {
    pub fn new() -> Self {
        Self::Empty
    }
    pub fn size(&self) -> usize {
        self.iter().count()
    }
    pub fn insert(&mut self, k: K, v: V) {
        let node = TreeNode::new(k, v);
        match self {
            TreeMap::Empty => *self = TreeMap::NonEmpty(node),
            TreeMap::NonEmpty(n) => n.insert(node),
        }
    }
    pub fn iter<'a>(&'a self) -> TreeIter<'a, K, V> {
        TreeIter::new(match self {
            TreeMap::Empty => None,
            TreeMap::NonEmpty(node) => Some(node),
        })
    }
}

impl<'a, K: Ord, V: PartialEq> IntoIterator for &'a TreeMap<K, V> {
    type Item = &'a Entry<K, V>;
    type IntoIter = TreeIter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug, PartialEq)]
pub struct Entry<K, V> {
    key: K,
    val: V,
}

impl<K, V> Entry<K, V> {
    fn new(key: K, val: V) -> Self {
        Self { key, val }
    }
}

#[derive(Debug, PartialEq)]
pub struct TreeNode<K, V> {
    entry: Entry<K, V>,
    left: Option<Box<TreeNode<K, V>>>,
    right: Option<Box<TreeNode<K, V>>>,
}

impl<K: Ord, V: PartialEq> TreeNode<K, V> {
    fn new(key: K, val: V) -> Self {
        Self {
            entry: Entry::new(key, val),
            left: None,
            right: None,
        }
    }
    /// walk is not really needed, was just an intermediary state
    /// to make sure the tree looked right
    fn walk<F>(&self, mut f: F)
    where
        F: FnMut(&K, &V),
    {
        fn walk_helper<K, V, F>(node: &TreeNode<K, V>, f: &mut F)
        where
            F: FnMut(&K, &V),
        {
            if let Some(node) = &node.left {
                walk_helper(node, f);
            }
            f(&node.entry.key, &node.entry.val);
            if let Some(node) = &node.right {
                walk_helper(node, f);
            }
        }
        walk_helper(self, &mut f);
    }
    fn insert(&mut self, node: TreeNode<K, V>) {
        if node.entry.key == self.entry.key {
            self.entry.val = node.entry.val;
            return;
        }
        let child = if node.entry.key < self.entry.key {
            &mut self.left
        } else {
            &mut self.right
        };
        if let Some(child) = child {
            child.insert(node);
        } else {
            *child = Some(Box::new(node))
        }
    }
}

pub struct TreeIter<'a, K, V> {
    stack: Vec<&'a TreeNode<K, V>>,
}

impl<'a, K, V> TreeIter<'a, K, V> {
    fn new(cur: Option<&'a TreeNode<K, V>>) -> Self {
        let mut iter = Self { stack: vec![] };
        if let Some(node) = cur {
            iter.push_left(node);
        }
        iter
    }
    fn push_left(&mut self, mut node: &'a TreeNode<K, V>) {
        self.stack.push(node);
        while let Some(n) = node.left.as_ref() {
            self.stack.push(n);
            node = n;
        }
    }
}

impl<'a, K, V> Iterator for TreeIter<'a, K, V> {
    type Item = &'a Entry<K, V>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some(node) => {
                if let Some(ref right) = node.right {
                    self.push_left(right);
                }
                Some(&node.entry)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_iter() {
        println!("here");
        let mut t = TreeMap::new();
        t.insert("foo", 32);
        t.insert("bar", 33);
        t.insert("zar", 34);
        t.insert("aar", 35);
        println!("here");
        let v = t.iter().count();
        assert_eq!(v, 4);
    }

    #[test]
    fn test_adds() {
        let mut t = TreeMap::new();
        assert_eq!(t.size(), 0);
        t.insert("foo", 32);
        t.insert("bar", 33);
        assert_eq!(t.size(), 2);
        t.insert("zar", 34);
        assert_eq!(t.size(), 3);
        t.insert("aar", 35);
        assert_eq!(t.size(), 4);
    }

    #[test]
    fn test_new_tree() {
        let mut t = TreeMap::new();

        println!("Here");
        assert_eq!(t.size(), 0);
        println!("Here2");
        t.insert("foo", 32);
        println!("Here3");
        assert_eq!(t.size(), 1);
        println!("Here4");
        t.insert("bar", 33);
        println!("Here5");
        assert_eq!(t.size(), 2);
        println!("Here6");
        t.insert("bar", 33);
        println!("Here7");
        assert_eq!(t.size(), 2);
    }

    #[test]
    fn test_tree_iter() {
        // empty
        let mut t: TreeMap<&'static str, i32> = TreeMap::new();
        let v = t.into_iter().collect::<Vec<_>>();
        assert!(v.is_empty());

        // has one entry
        t.insert("age", 48);
        let v = t.iter().collect::<Vec<_>>();
        assert_eq!(v, vec![&Entry::new("age", 48)]);

        // change the value of the entry
        t.insert("age", 49);
        let v = t.iter().collect::<Vec<_>>();
        assert_eq!(v, vec![&Entry::new("age", 49)]);

        // add a new entry with a greater key
        t.insert("age2", 50);
        let v = t.iter().collect::<Vec<_>>();
        assert_eq!(v, vec![&Entry::new("age", 49), &Entry::new("age2", 50)]);

        // add a new entry with a lesser key
        t.insert("age3", 45);
        assert_eq!(t.size(), 3);
        let v = t.iter().collect::<Vec<_>>();
        assert_eq!(
            v,
            vec![
                &Entry::new("age", 49),
                &Entry::new("age2", 50),
                &Entry::new("age3", 45),
            ]
        );
    }
}
