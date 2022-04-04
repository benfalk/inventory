use crate::prelude::*;
use std::cmp::Eq;
use std::hash::Hash;
use std::fmt::Debug;

pub trait Model: Sized + Debug {
    type ID: Eq + Hash + Clone + Debug;

    fn matches_text_search(&self, _text: &str) -> bool {
        false
    }

    fn id(&self) -> &Self::ID;

    fn merge_with_other(&mut self, _other: Self) {}

    fn normalize_search_term(term: &str) -> String;
}

#[derive(Debug, Clone)]
pub struct Inventory {
    pub id: String,
    pub name: String,
    pub quantity: Option<usize>,
    pub note: Option<String>,
}

impl GuiTableModel for Inventory {
    fn header() -> Vec<String> {
        vec![
            "ID".to_owned(),
            "Name".to_owned(),
            "Quantity".to_owned(),
            "Note".to_owned(),
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.name.clone(),
            self.quantity.clone().map(|q|format!("{q}")).unwrap_or_default(),
            self.note.clone().unwrap_or_default(),
        ]
    }
}

impl Model for Inventory {
    type ID = String;

    fn id(&self) -> &Self::ID {
        &self.id
    }

    fn normalize_search_term(term: &str) -> String {
        term.to_lowercase()
    }

    fn matches_text_search(&self, text: &str) -> bool {
        self.name.to_lowercase().contains(text)
    }

    fn merge_with_other(&mut self, other: Self) {
        if let Some(amount) = other.quantity {
            *self.quantity.get_or_insert(0) += amount;
        }

        if let Some(mut note) = other.note {
            let original = self.note.take().unwrap_or_default();
            note.push_str(&original);
            self.note = Some(note);
        }
    }
}
