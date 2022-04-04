use crate::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Repo<M: Model> {
    storage: HashMap<M::ID, M>,
}

impl<M: Model> Repo<M> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            storage: HashMap::with_capacity(capacity),
        }
    }

    pub fn get(&self, id: &M::ID) -> Option<&M> {
        self.storage.get(id)
    }

    pub fn get_mut(&mut self, id: &M::ID) -> Option<&mut M> {
        self.storage.get_mut(id)
    }

    pub fn text_search(&self, term: &str) -> Vec<&M> {
        let term = M::normalize_search_term(term);

        self.storage
            .values()
            .filter(|model| model.matches_text_search(&term))
            .collect()
    }

    pub fn items(&self) -> impl Iterator<Item = &M> {
        self.storage.values()
    }

    pub fn add_model(&mut self, model: M) {
        match self.storage.get_mut(model.id()) {
            None => {
                self.storage.insert(model.id().clone(), model);
            },
            Some(existing) => {
                existing.merge_with_other(model);
            }
        }
    }

    pub fn load<S: DataStore<M>>(&mut self, store: S) {
        for model in store.into_iter() {
            self.add_model(model);
        }
    }
}
