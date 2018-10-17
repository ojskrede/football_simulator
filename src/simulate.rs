//! Simulate
//!

use std::slice;
use std::sync::{Arc, Mutex};

use game;
use team;
use table;
use failure::{Error};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use itertools::Itertools;
use itertools::structs::MultiProduct;

fn simulated_outcomes(num_games: usize) -> MultiProduct<slice::Iter<'static, game::GameResult>> {
    let outcomes = &[game::GameResult::HomeWin, game::GameResult::AwayWin, game::GameResult::Draw];

    let simulations = (0..num_games).map(|_| outcomes.iter()).multi_cartesian_product();
    simulations
}

#[allow(dead_code)]
fn simulated_outcomes_as_vec(num_games: usize) -> Vec<Vec<&'static game::GameResult>> {
    let outcomes = &[game::GameResult::HomeWin, game::GameResult::AwayWin, game::GameResult::Draw];

    let simulations = (0..num_games).map(|_| outcomes.iter()).multi_cartesian_product();
    let mut simulations_vec = Vec::<Vec<&game::GameResult>>::with_capacity(simulations.size_hint().1.unwrap());
    for simulation in simulations {
        simulations_vec.push(simulation);
    }
    simulations_vec
}

fn find_unplayed_games_in_round(round: u8, games: &[game::Game]) -> Vec<game::Game> {
    let mut unplayed_games = Vec::<game::Game>::new();
    for game in games.iter() {
        if game.round() == round && game.game_result().is_none() {
            unplayed_games.push(game.clone());
        }
    }
    unplayed_games
}

pub fn simulate_rounds(
    rounds: &[u8],
    games: &Vec<game::Game>,
    teams: &Vec<team::Team>,
    ) -> Result<Vec<table::Table>, Error> {
    let mut played_games = Vec::<game::Game>::new();
    for game in games.iter() {
        if game.played() {
            played_games.push(game.clone());
        }
    }
    let original_table = table::Table::new_with(teams, &played_games);
    let mut simulated_tables = Vec::<table::Table>::new();
    let mut unplayed_games = Vec::<game::Game>::new();
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

    for simulation in simulations {
        //assert_eq!(unplayed_games.clone().len(), simulation.clone().len());
        for (ind, game) in unplayed_games.iter_mut().enumerate() {
            let (home_goals, away_goals) = match &simulation[ind] {
                &game::GameResult::HomeWin => (1, 0),
                &game::GameResult::Draw => (0, 0),
                &game::GameResult::AwayWin => (0, 1),
            };

            let probability = game::compute_result_probability(
                &game.home_team(),
                &game.away_team(),
                &simulation[ind],
                game::ResultProbabilityMethod::GameResultAndPointDifference,
                );

            game.set_result_from_simulated(
                &simulation[ind],
                probability,
                home_goals,
                away_goals,
                ).unwrap();
        }

        let table = original_table.get_updated(unplayed_games.as_slice());
        simulated_tables.push(table);
        //pbar.inc(1);
    }
    //pbar.finish();

    Ok(simulated_tables)
}

pub enum StoreTable {
    MostProbable(usize),
    LeastProbable(usize),
}

pub fn simulate_rounds_parallel(
    rounds: &[u8],
    games: &[game::Game],
    teams: &[team::Team],
    store_table: &StoreTable,
    ) -> Result<Vec<table::Table>, Error> {
    let mut played_games = Vec::<game::Game>::new();
    for game in games.iter() {
        if game.played() {
            played_games.push(game.clone());
        }
    }
    let original_table = table::Table::new_with(teams, &played_games);
    let simulated_tables = Arc::new(Mutex::new(Vec::<table::Table>::new()));
    let mut unplayed_games = Vec::<game::Game>::new();
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

    let _ = simulations.par_bridge().map(|simulation| {
        //assert_eq!(unplayed_games.clone().len(), simulation.clone().len());
        let mut unplayed_games = unplayed_games.clone();
        for (ind, game) in unplayed_games.iter_mut().enumerate() {
            //let game_result = simulation[ind].clone();
            let (home_goals, away_goals) = match &simulation[ind] {
                &game::GameResult::HomeWin => (1, 0),
                &game::GameResult::Draw => (0, 0),
                &game::GameResult::AwayWin => (0, 1),
            };

            let probability = game::compute_result_probability(
                &game.home_team(),
                &game.away_team(),
                &simulation[ind],
                game::ResultProbabilityMethod::GameResultAndPointDifference,
                );

            game.set_result_from_simulated(
                &simulation[ind],
                probability,
                home_goals,
                away_goals,
                ).unwrap();
        }
        let table = original_table.get_updated(unplayed_games.as_slice());

        match store_table {
            StoreTable::MostProbable(n) => {
                simulated_tables.lock().unwrap().push(table);
                simulated_tables
                    .lock()
                    .unwrap()
                    .sort_by(|a, b| b.probability().partial_cmp(&a.probability()).unwrap());
                if simulated_tables.lock().unwrap().len() > *n {
                    let _ = simulated_tables.lock().unwrap().pop();
                }
            },
            StoreTable::LeastProbable(n) => {
                simulated_tables.lock().unwrap().push(table);
                simulated_tables
                    .lock()
                    .unwrap()
                    .sort_by(|a, b| a.probability().partial_cmp(&b.probability()).unwrap());
                if simulated_tables.lock().unwrap().len() > *n {
                    let _ = simulated_tables.lock().unwrap().pop();
                }
            },
        }

        //pbar.inc(1);
    }).count();
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

pub fn compute_standing_distributions(
    rounds: &[u8],
    games: &Vec<game::Game>,
    teams: &Vec<team::Team>,
) -> Result<[[u64; 14]; 14], Error> {
    let mut played_games = Vec::<game::Game>::new();
    for game in games.iter() {
        if game.played() {
            played_games.push(game.clone());
        }
    }
    let original_table = table::Table::new_with(teams, &played_games);
    let standing_distributions = Arc::new(Mutex::new([[0_u64; 14]; 14]));

    let mut unplayed_games = Vec::<game::Game>::new();
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

    let _ = simulations.par_bridge().map(|simulation| {
        //assert_eq!(unplayed_games.clone().len(), simulation.clone().len());
        let mut unplayed_games = unplayed_games.clone();
        for (ind, game) in unplayed_games.iter_mut().enumerate() {
            //let game_result = simulation[ind].clone();
            let (home_goals, away_goals) = match &simulation[ind] {
                &game::GameResult::HomeWin => (1, 0),
                &game::GameResult::Draw => (0, 0),
                &game::GameResult::AwayWin => (0, 1),
            };

            let probability = game::compute_result_probability(
                &game.home_team(),
                &game.away_team(),
                &simulation[ind],
                game::ResultProbabilityMethod::GameResultAndPointDifference,
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
        for (position, (_, team)) in table.sorted_standings().iter().enumerate() {
            standing_distributions.lock().unwrap()[team.lexicographical_position() as usize][position] += 1;
        }
        //pbar.inc(1);
    }).count();
    //pbar.finish();

    match Arc::try_unwrap(standing_distributions) {
        Ok(mutex_val) => {
            match mutex_val.into_inner() {
                Ok(distributions) => Ok(distributions),
                Err(msg) => Err(format_err!("{}", msg)),
            }
        },
        Err(msg) => Err(format_err!("{:?}", msg)),
    }
}
