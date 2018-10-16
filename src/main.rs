//! Predict season standings

extern crate clap;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate csv;
#[macro_use] extern crate failure;
extern crate itertools;
extern crate indicatif;
extern crate rayon;

use std::path::{Path};
use std::time::{Instant};
use std::env;
use failure::{Error, err_msg};
use clap::{Arg, App};

pub mod data_io;
pub mod table;
pub mod game;
pub mod team;
pub mod simulate;

pub fn main() -> Result<(), Error> {
    let matches = App::new("Season predictor")
                      .version("0.1.0")
                      .author("Ole-Johan Skrede")
                      .about("Simulate the rest of a football season")
                      .arg(Arg::with_name("season_fixtures")
                           .short("i")
                           .long("season_fixtures")
                           .value_name("PATH")
                           .help("A csv file with current season fixtures (played and unplayed).")
                           .required(true))
                      .arg(Arg::with_name("task")
                           .short("t")
                           .long("task")
                           .value_name("STR")
                           .help("Task to perform in {tables, standings}")
                           .default_value("tables"))
                      .arg(Arg::with_name("output_dir")
                           .short("o")
                           .long("output_dir")
                           .value_name("PATH")
                           .help("Output directory"))
                      .get_matches();

    let season_fixtures = Path::new(matches.value_of("season_fixtures")
                                           .ok_or(err_msg("Season fixture input error"))?);
    let output_directory = match matches.value_of("output_dir") {
        Some(val) => Path::new(val).to_path_buf(),
        None => env::current_dir()?,
    };

    let (games, teams) = data_io::import_games(&season_fixtures)?;
    let rounds = &[25];

    for game in games.iter() {
        println!("{}", game);
    }

    let teams: Vec<team::Team> = teams.values().map(|x| x.clone()).collect();
    let mut played_games = Vec::<game::Game>::new();
    for game in games.iter() {
        if game.played() {
            played_games.push(game.clone());
        }
    }
    match matches.value_of("task") {
        Some("tables") => {
            let current_table = table::Table::new_with(&teams, &played_games);
            current_table.print_table();

            let start_time0 = Instant::now();
            let mut simulated_tables = simulate::simulate_rounds_parallel(rounds, &games, &teams)?;
            let total_duration = start_time0.elapsed();
            println!("Simulation finished, elapsed time {}",
                     total_duration.as_secs() as f64 + total_duration.subsec_nanos() as f64 * 1e-9);

            simulated_tables.sort_by(|a, b| b.probability().partial_cmp(&a.probability()).unwrap());

            println!("Number of simulations: {}", simulated_tables.len());
            simulated_tables[0].print_table();
            simulated_tables[0].print_latest_round();
            simulated_tables[1].print_table();
            simulated_tables[2].print_table();
            simulated_tables[3].print_table();
        },
        Some("standings") => {
            let start_time0 = Instant::now();
            let distributions = simulate::compute_standing_distributions(rounds, &games, &teams)?;
            let total_duration = start_time0.elapsed();
            println!("Simulation finished, elapsed time {}",
                     total_duration.as_secs() as f64 + total_duration.subsec_nanos() as f64 * 1e-9);
            let current_table = table::Table::new_with(&teams, &played_games);
            let order_map = table::lexicographical_team_order_from_table(&current_table);

            let mut writer = csv::WriterBuilder::new()
                .delimiter(b',')
                .quote_style(csv::QuoteStyle::NonNumeric)
                .from_path(output_directory.join("standing_distributions.csv"))?;

            writer.write_record(&["Team", "Pos 1", "Pos 2", "Pos 3", "Pos 4", "Pos 5", "Pos 6", "Pos 7", "Pos 8", "Pos 9", "Pos 10", "Pos 11", "Pos 12", "Pos 13", "Pos 14"])?;
            for (team_pos, team_name) in order_map.iter().enumerate() {
                println!("{:<30} {:?}", team_name, distributions[team_pos]);
                let mut row = Vec::<String>::new();
                row.push(team_name.clone());
                let pos_distributions: Vec<String> = distributions[team_pos].iter().map(|x| format!("{}", x)).collect();
                row.extend_from_slice(pos_distributions.as_slice());
                writer.write_record(&row)?;
            }
        }
        _ => println!("Please provide a valid task"),
    }

    Ok(())
}
