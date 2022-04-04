use crate::prelude::*;
use csv::Reader;
use serde::Deserialize;
use std::marker::PhantomData;

pub type InventoryCsv = CsvStore<InventoryRow, Inventory>;

pub trait DataStore<M: Model>: IntoIterator<Item = M> + Sized + Clone {
    fn into_repo(self) -> Repo<M> {
        let mut repo = Repo::new();
        repo.load(self);
        repo
    }
}

#[derive(Debug, Clone)]
pub struct CsvStore<Row: Clone, Data: Clone> {
    path: String,
    row: PhantomData<Row>,
    data: PhantomData<Data>,
}

impl<Row: Clone, Data: Clone> CsvStore<Row, Data> {
    pub fn from_path(path: &str) -> Self {
        Self {
            path: path.to_owned(),
            row: PhantomData,
            data: PhantomData,
        }
    }
}

impl<Row: for<'de> Deserialize<'de> + Clone, Data: From<Row> + Clone> IntoIterator
    for CsvStore<Row, Data>
{
    type Item = Data;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Reader::from_path(self.path)
            .unwrap()
            .into_deserialize()
            .flat_map(|result| {
                let row: Row = result.ok()?;
                Some(row.into())
            })
    }
}

impl<R: for<'de> Deserialize<'de> + Clone, M: Model + From<R> + Clone> DataStore<M>
    for CsvStore<R, M>
{
}

// Plumbing for Inventory CSV Store

#[derive(Deserialize, Clone)]
pub struct InventoryRow {
    #[serde(rename = "Product ID")]
    id: String,
    #[serde(rename = "Product Name")]
    name: String,
    #[serde(rename = "Product Qty")]
    quantity: Option<usize>,
    #[serde(rename = "Product Note")]
    note: Option<String>,
}

impl From<InventoryRow> for Inventory {
    fn from(row: InventoryRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            quantity: row.quantity,
            note: row.note,
        }
    }
}
