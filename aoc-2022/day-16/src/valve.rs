use crate::prelude::*;
use std::{collections::HashSet, hash::Hash, rc::Rc};

#[derive(PartialEq, Eq, Debug)]
struct Valves {
    current: Rc<Valve>,
    open: HashSet<Rc<Valve>>,
    closed: HashSet<Rc<Valve>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Valve {
    pub name: String,
    pub rate: i32,
    pub tunnels: Vec<String>,
}

impl Hash for Valve {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.name.as_bytes())
    }
}

impl Valve {
    pub fn new<T>(name: T, rate: i32, tunnels: Vec<String>) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            rate,
            tunnels,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(PartialEq, Eq, Debug)]
    struct State {
        open: HashSet<Rc<Valve>>,
        clos: HashSet<Rc<Valve>>,
    }

    impl State {}

    #[test]
    fn test_saved_state_super_struct() {
        let v1 = Rc::new(Valve::new("AA", 5, vec!["CC".to_string()]));
        let v2 = Rc::new(Valve::new("BB", 5, vec![]));
        let v3 = Rc::new(Valve::new("CC", 5, vec![]));

        let open: HashSet<Rc<Valve>> = [&v1, &v2].into_iter().map(|s| s.clone()).collect();
        let closed: HashSet<Rc<Valve>> = [&v3].into_iter().map(|s| s.clone()).collect();
        let mut s1 = State { open, clos: closed };

        let open: HashSet<Rc<Valve>> = [&v1, &v2].into_iter().map(|s| s.clone()).collect();
        let closed: HashSet<Rc<Valve>> = [&v3].into_iter().map(|s| s.clone()).collect();
        let mut s2 = State { open, clos: closed };

        assert_eq!(s1, s2);

        s1.open.insert(v3.clone());
        assert_ne!(s1, s2);
        s2.open.insert(v3.clone());
        assert_eq!(s1, s2);
        assert_eq!(s1, s2);

        s1.clos.clear();
        s1.open.clear();
        s2.clos.clear();
        s2.open.clear();
        assert_eq!(s1, s2);

        s1.open.insert(v1.clone());
        s2.clos.insert(v1.clone());
        assert_ne!(s1, s2);
        s1.clos.insert(v1.clone());
        assert_ne!(s1, s2);
        s2.open.insert(v1.clone());
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_hashset_with_rc() {
        let mut hs: HashSet<Rc<Valve>> = HashSet::new();
        let v1 = Rc::new(Valve::new("AA", 5, vec!["CC".to_string()]));
        let v2 = Rc::new(Valve::new("BB", 5, vec![]));

        // insertions
        assert!(!hs.contains(&v1));
        hs.insert(v1.clone());
        assert!(hs.contains(&v1));
        assert!(!hs.contains(&v2));
        hs.insert(v1.clone());
        assert!(hs.contains(&v1));
        assert!(!hs.contains(&v2));
        assert_eq!(1, hs.len());
        hs.insert(v2.clone());
        assert!(hs.contains(&v1));
        assert!(hs.contains(&v2));
        assert_eq!(2, hs.len());

        // deletions
        hs.remove(&v2);
        assert!(hs.contains(&v1));
        assert!(!hs.contains(&v2));
        assert_eq!(1, hs.len());
        hs.remove(&v1);
        assert!(!hs.contains(&v1));
        assert!(!hs.contains(&v2));
        assert_eq!(0, hs.len());
    }
}
