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

    pub fn get(&self, n: Name) -> Option<&String> {
        let id = n.0;
        self.store.get(id)
    }

    pub fn register(&mut self, name: &str) -> Name {
        match self.store.iter().enumerate().find(|(k, v)| v == &name) {
            Some((k, _)) => Name(k),
            None => {
                let id = self.last_id + 1;
                self.last_id = id;
                self.store.push(String::from(name));
                Name(id)
            }
        }
    }
}