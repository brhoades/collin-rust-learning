use std::hash::Hash;

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
