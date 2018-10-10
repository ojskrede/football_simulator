//! Predict season standings

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate csv;
#[macro_use] extern crate failure;
extern crate itertools;
extern crate indicatif;
extern crate rayon;

use std::path::Path;
use std::time::{Instant};
use failure::Error;

pub mod data_io;
pub mod table;
pub mod structures;
pub mod simulate;

pub fn main() -> Result<(), Error> {

    let games_overview_filename = Path::new("assets/matches.csv");

    let games = data_io::import_games(&games_overview_filename)?;

    for game in games.iter() {
        println!("{}", game);
    }

    let current_table = table::Table::new_with(&games);
    current_table.print_table();

    let start_time0 = Instant::now();
    let mut simulated_tables = simulate::simulate_rounds_parallel(&[21, 22, 24, 25], &games)?;
    let total_duration = start_time0.elapsed();
    println!("Simulation finished, elapsed time {}",
             total_duration.as_secs() as f64 + total_duration.subsec_nanos() as f64 * 1e-9);

    simulated_tables.sort_by(|a, b| b.probability().partial_cmp(&a.probability()).unwrap());

    println!("Number of simulations: {}", simulated_tables.len());
    simulated_tables[0].print_table();
    simulated_tables[1].print_table();
    simulated_tables[2].print_table();
    simulated_tables[3].print_table();
    //simulated_tables.last().unwrap().print_table();

    Ok(())
}
