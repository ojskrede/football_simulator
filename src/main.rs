//! Predict season standings

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate csv;
extern crate failure;

use std::path::Path;
use failure::Error;

pub mod data_io;

pub fn main() -> Result<(), Error> {

    let match_overview_filename = Path::new("assets/matches.csv");

    let matches = data_io::import_matches(&match_overview_filename)?;

    for m in matches {
        println!("{}", m);
    }

    Ok(())
}
