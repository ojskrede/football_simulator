//! Simulate
//!

use std::slice;

use structures::{Game, GameResult};
use table::Table;
use failure::{Error};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use itertools::Itertools;
use itertools::structs::MultiProduct;

fn simulated_outcomes(num_games: usize) -> MultiProduct<slice::Iter<'static, GameResult>> {
    let outcomes = &[GameResult::HomeWin, GameResult::AwayWin, GameResult::Draw];
    let num_results = outcomes.len().pow(num_games as u32);

    let simulations = (0..num_games).map(|_| outcomes.iter()).multi_cartesian_product();
    simulations
}

fn find_unplayed_games_in_round(round: u8, games: &[Game]) -> Vec<(usize, Game)> {
    let mut unplayed_games = Vec::<(usize, Game)>::new();
    for (ind, game) in games.iter().enumerate() {
        if game.round() == round && game.game_result().is_none() {
            unplayed_games.push((ind, game.clone()));
        }
    }
    unplayed_games
}

pub fn simulate_rounds(rounds: &[u8], games: &Vec<Game>) -> Result<Vec<Table>, Error> {
    let mut simulated_tables = Vec::<Table>::new();
    let mut unplayed_games = Vec::<(usize, Game)>::new();
    for round in rounds {
        unplayed_games.extend_from_slice(find_unplayed_games_in_round(*round, &games).as_slice());
    }
    let simulations = simulated_outcomes(unplayed_games.len());
    println!("Simulating {} results from {} games", simulations.size_hint().1.unwrap(), unplayed_games.len());
    let pbar = ProgressBar::new(simulations.size_hint().1.unwrap() as u64);
    pbar.set_style(ProgressStyle::default_bar().template(
        "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining] [rendering]",
    ));
    for simulation in simulations {
        //assert_eq!(unplayed_games.clone().len(), simulation.clone().len());
        for (ind, (_, game)) in unplayed_games.iter_mut().enumerate() {
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
        let mut updated_games = games.clone();
        for (ind, game) in unplayed_games.iter() {
            updated_games[*ind] = game.clone();
        }
        // Should be possible to do updated_games[ind] = game ???
        simulated_tables.push(Table::new_with(&updated_games));
        pbar.inc(1);
    }
    pbar.finish();

    Ok(simulated_tables)
}

/*
pub fn simulate_rounds_par(rounds: &[u8], games: &Vec<Game>) -> Result<Vec<Table>, Error> {
    let mut simulated_tables = Arc::new(Mutex::new(Vec::<Table>::new()));
    let mut unplayed_games = Vec::<(usize, Game)>::new();
    for round in rounds {
        unplayed_games.extend_from_slice(find_unplayed_games_in_round(*round, &games).as_slice());
    }
    let results = match unplayed_games.len() {
        1 => result_simulator::run_01_games(),
        2 => result_simulator::run_02_games(),
        7 => result_simulator::run_07_games(),
        8 => result_simulator::run_08_games(),
        /*
        9 => result_simulator::run_09_games(),
        14 => result_simulator::run_14_games(),
        15 => result_simulator::run_15_games(),
        16 => result_simulator::run_16_games(),
        */
        _ => unreachable!(),
    };
    println!("Simulating {} results from {} games", results.len(), unplayed_games.len());
    let pbar = ProgressBar::new(results.len() as u64);
    pbar.set_style(ProgressStyle::default_bar().template(
        "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining] [rendering]",
    ));
    println!("HELLO 1");
    let _ = results.par_iter().map(|simulation| {
        println!("HELLO 2");
        //assert_eq!(unplayed_games.clone().len(), simulation.clone().len());
        println!("Num games in simulation: {}", simulation.len());
        let mut unplayed_games = unplayed_games.clone();
        for (ind, (_, game)) in unplayed_games.iter_mut().enumerate() {
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
        let mut updated_games = games.clone();
        for (ind, game) in unplayed_games.iter() {
            updated_games[*ind] = game.clone();
        }
        // Should be possible to do updated_games[ind] = game ???
        simulated_tables.lock().unwrap().push(Table::new_with(&updated_games));
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
*/

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
