#![feature(type_alias_impl_trait)]
// supress dead code warnings during development
#![allow(dead_code)]
#![allow(unused_imports)]

mod models;
mod prelude;
mod repos;
mod stores;
mod gui;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    /// CSV file to open
    csv_file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::prelude::*;
    let args = Args::parse();
    let store = InventoryCsv::from_path(&args.csv_file);
    let mut app = DataManagmentGui::new(store);
    app.run()?;

    Ok(())
}
