use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Name(usize);

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct NameStore {
    store: Vec<String>,
    last_id: usize
}

impl NameStore {
    pub fn new() -> Self {
        NameStore {
            store: Vec::new(),
            last_id: 0
        }
    }

    pub fn get_by_interned(&self, name: &str) -> Option<Name> {
        self.store.iter().enumerate().find(|(k, v)| v == &name).map(|(id, _)| Name(id))
    }

    pub fn get(&self, n: Name) -> Option<&String> {
        let id = n.0;
        self.store.get(id)
    }

    pub fn by_index(&self, idx: usize) -> Option<Name> {
        if idx >= 0 && idx < self.store.len() {
            Some(Name(idx))
        } else {
            None
        }
    }

    pub fn register(&mut self, name: &str) -> Name {
        match self.get_by_interned(name) {
            Some(n) => n,
            None => {
                let id = self.last_id + 1;
                self.last_id = id;
                self.store.push(String::from(name));
                Name(id)
            }
        }
    }
}