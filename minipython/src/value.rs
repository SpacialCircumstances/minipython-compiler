use crate::name::{InternedName, NameStore};

pub struct Value {
    id: u64,
    name: InternedName
}

impl Value {
    pub fn new(id: u64, name: InternedName) -> Self {
        Value { id, name }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_name<'a>(&self, store: &'a NameStore) -> Option<&'a String> {
        store.get(self.name)
    }
}