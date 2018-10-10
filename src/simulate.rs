//! Simulate
//!

use std::slice;
use std::sync::{Arc, Mutex};

use structures::{Game, GameResult};
use table::Table;
use failure::{Error};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use itertools::Itertools;
use itertools::structs::MultiProduct;
use std::time::{Duration, Instant};

fn simulated_outcomes(num_games: usize) -> MultiProduct<slice::Iter<'static, GameResult>> {
    let outcomes = &[GameResult::HomeWin, GameResult::AwayWin, GameResult::Draw];

    let simulations = (0..num_games).map(|_| outcomes.iter()).multi_cartesian_product();
    simulations
}

fn simulated_outcomes_as_vec(num_games: usize) -> Vec<Vec<&'static GameResult>> {
    let outcomes = &[GameResult::HomeWin, GameResult::AwayWin, GameResult::Draw];

    let simulations = (0..num_games).map(|_| outcomes.iter()).multi_cartesian_product();
    let mut simulations_vec = Vec::<Vec<&GameResult>>::with_capacity(simulations.size_hint().1.unwrap());
    for simulation in simulations {
        simulations_vec.push(simulation);
    }
    simulations_vec
}

fn find_unplayed_games_in_round(round: u8, games: &[Game]) -> Vec<Game> {
    let mut unplayed_games = Vec::<Game>::new();
    for game in games.iter() {
        if game.round() == round && game.game_result().is_none() {
            unplayed_games.push(game.clone());
        }
    }
    unplayed_games
}

pub fn simulate_rounds(rounds: &[u8], games: &Vec<Game>) -> Result<Vec<Table>, Error> {
    let original_table = Table::new_with(&games);
    let mut simulated_tables = Vec::<Table>::new();
    let mut unplayed_games = Vec::<Game>::new();
    for round in rounds {
        unplayed_games.extend_from_slice(find_unplayed_games_in_round(*round, &games).as_slice());
    }
    let simulations = simulated_outcomes(unplayed_games.len());
    println!("Simulating {} results from {} games", simulations.size_hint().1.unwrap(), unplayed_games.len());

    // Progress bar takes about 10 % of the time
    let pbar = ProgressBar::new(simulations.size_hint().1.unwrap() as u64);
    pbar.set_style(ProgressStyle::default_bar().template(
        "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining] [rendering]",
    ));

    let mut iter_unplayed_duration = Duration::new(0, 0);
    let mut match_simulation_duration = Duration::new(0, 0);
    let mut result_probability_duration = Duration::new(0, 0);
    let mut set_simulated_duration = Duration::new(0, 0);
    let mut update_games_duration = Duration::new(0, 0);
    let mut simulate_tables_duration = Duration::new(0, 0);
    let start_time0 = Instant::now();
    for simulation in simulations {
        //assert_eq!(unplayed_games.clone().len(), simulation.clone().len());
        //let start_time1 = Instant::now();
        for (ind, game) in unplayed_games.iter_mut().enumerate() {
            //let start_time2 = Instant::now();
            let (home_goals, away_goals) = match &simulation[ind] {
                &GameResult::HomeWin => (1, 0),
                &GameResult::Draw => (0, 0),
                &GameResult::AwayWin => (0, 1),
            };
            //match_simulation_duration += start_time2.elapsed();

            //let start_time3 = Instant::now();
            let probability = result_probability(
                &simulation[ind]
                );
            //result_probability_duration += start_time3.elapsed();

            //let start_time4 = Instant::now();
            game.set_result_from_simulated(
                &simulation[ind],
                probability,
                home_goals,
                away_goals,
                ).unwrap();
            //set_simulated_duration += start_time4.elapsed();
        }
        //iter_unplayed_duration += start_time1.elapsed();

        //let start_time6 = Instant::now();
        let table = original_table.get_updated(unplayed_games.as_slice());
        simulated_tables.push(table);
        //simulate_tables_duration += start_time6.elapsed();
        //pbar.inc(1);
    }
    //pbar.finish();

    let total_duration = start_time0.elapsed();
    println!("Simulation finished, elapsed time {}",
             total_duration.as_secs() as f64 + total_duration.subsec_nanos() as f64 * 1e-9);
    /*
    println!("Match simulation:                 {}",
             match_simulation_duration.as_secs() as f64 + match_simulation_duration.subsec_nanos() as f64 * 1e-9);
    println!("Result probability:               {}",
             result_probability_duration.as_secs() as f64 + result_probability_duration.subsec_nanos() as f64 * 1e-9);
    println!("Set simulated:                    {}",
             set_simulated_duration.as_secs() as f64 + set_simulated_duration.subsec_nanos() as f64 * 1e-9);
    println!("Iterate unplayed:                 {}",
             iter_unplayed_duration.as_secs() as f64 + iter_unplayed_duration.subsec_nanos() as f64 * 1e-9);
    println!("Simulate tables:                  {}",
             simulate_tables_duration.as_secs() as f64 + simulate_tables_duration.subsec_nanos() as f64 * 1e-9);
    */

    Ok(simulated_tables)
}

pub fn simulate_rounds_parallel(rounds: &[u8], games: &Vec<Game>) -> Result<Vec<Table>, Error> {
    let original_table = Table::new_with(&games);
    let mut simulated_tables = Arc::new(Mutex::new(Vec::<Table>::new()));
    let mut unplayed_games = Vec::<Game>::new();
    for round in rounds {
        unplayed_games.extend_from_slice(find_unplayed_games_in_round(*round, &games).as_slice());
    }
    let simulations = simulated_outcomes_as_vec(unplayed_games.len());
    println!("Simulating {} results from {} games", simulations.len(), unplayed_games.len());

    // Progress bar takes about 10 % of the time
    let pbar = ProgressBar::new(simulations.len() as u64);
    pbar.set_style(ProgressStyle::default_bar().template(
        "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining] [rendering]",
    ));

    let _ = simulations.par_iter().map(|simulation| {
        //assert_eq!(unplayed_games.clone().len(), simulation.clone().len());
        println!("Num games in simulation: {}", simulation.len());
        let mut unplayed_games = unplayed_games.clone();
        for (ind, game) in unplayed_games.iter_mut().enumerate() {
            //let game_result = simulation[ind].clone();
            let (home_goals, away_goals) = match &simulation[ind] {
                &GameResult::HomeWin => (1, 0),
                &GameResult::Draw => (0, 0),
                &GameResult::AwayWin => (0, 1),
            };
            let probability = result_probability(
                &simulation[ind]
                );

            game.set_result_from_simulated(
                &simulation[ind],
                probability,
                home_goals,
                away_goals,
                ).unwrap();
        }
        // Should be possible to do updated_games[ind] = game ???
        let table = original_table.get_updated(unplayed_games.as_slice());
        simulated_tables.lock().unwrap().push(table);
        //pbar.inc(1);
    });
    //pbar.finish();

    match Arc::try_unwrap(simulated_tables) {
        Ok(mutex_val) => {
            match mutex_val.into_inner() {
                Ok(tables) => Ok(tables),
                Err(msg) => Err(format_err!("{}", msg)),
            }
        },
        Err(msg) => Err(format_err!("{:?}", msg)),
    }
}

pub fn result_probability(
    //home_team: &Team,
    //away_team: &Team,
    //games: &[Game],
    result: &GameResult,
) -> f32 {
    match result {
        GameResult::HomeWin => 0.4,
        GameResult::AwayWin => 0.35,
        GameResult::Draw => 0.25,
    }
}
