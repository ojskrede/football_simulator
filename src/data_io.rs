//! Handle input output
//!

use std::path::Path;

use failure::Error;
use csv::Reader;

use structures;

pub fn import_games(input_filename: &Path) -> Result<Vec<structures::Game>, Error> {
    let mut games = Vec::<structures::Game>::new();
    let mut reader = Reader::from_path(&input_filename)?;
    for result in reader.deserialize() {
        let mut record: structures::Game = result?;
        record.set_result_from_current()?;
        games.push(record);
    }
    Ok(games)
}
