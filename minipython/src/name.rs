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
    store: HashMap<usize, String>,
    last_id: usize
}

impl NameStore {
    fn new() -> Self {
        NameStore {
            store: HashMap::new(),
            last_id: 0
        }
    }

    fn get(&self, n: Name) -> Option<&String> {
        let id = n.0;
        self.store.get(&id)
    }

    fn register(&mut self, name: &str) -> Name {
        let found = self.store.iter().find(|(key, value)| value.eq(&name));
        match found {
            None => {
                let id = self.last_id + 1;
                self.last_id = id;
                self.store.insert(id, String::from(name));
                Name(id)
            },
            Some((k, _)) => {
                Name(*k)
            }
        }
    }
}