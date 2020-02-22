use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct InternedName(usize);

impl Display for InternedName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct NameStore {
    store: Vec<String>,
    next_id: usize
}

impl NameStore {
    pub fn new() -> Self {
        NameStore {
            store: Vec::new(),
            next_id: 0
        }
    }

    pub fn get_by_interned(&self, name: &str) -> Option<InternedName> {
        self.store.iter().position(|v| v == &name).map(|id| InternedName(id))
    }

    pub fn get(&self, n: InternedName) -> Option<&String> {
        let id = n.0;
        self.store.get(id)
    }

    pub fn by_index(&self, idx: usize) -> Option<InternedName> {
        if idx < self.store.len() {
            Some(InternedName(idx))
        } else {
            None
        }
    }

    pub fn register(&mut self, name: &str) -> InternedName {
        match self.get_by_interned(name) {
            Some(n) => n,
            None => {
                let id = self.next_id;
                self.next_id = id + 1;
                self.store.push(String::from(name));
                InternedName(id)
            }
        }
    }
}