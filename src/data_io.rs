//! Handle input output
//!

use std::path::Path;

use failure::Error;
use csv::Reader;

use structures;
use table;

#[derive(Debug, Clone, Deserialize)]
pub struct Game {
    #[serde(rename = "Runde")]
    round: u8,
    #[serde(rename = "Dato")]
    date: String,
    #[serde(rename = "Dag")]
    day: String,
    #[serde(rename = "Tid")]
    time: String,
    #[serde(rename = "Hjemmelag")]
    home_team_name: String,
    #[serde(rename = "Resultat")]
    result: String,
    #[serde(rename = "Bortelag")]
    away_team_name: String,
    #[serde(rename = "Bane")]
    pitch: String,
    #[serde(rename = "Turnering")]
    tournament: String,
    #[serde(rename = "Kampnummer")]
    game_number: usize,
}

pub fn import_games(input_filename: &Path) -> Result<Vec<structures::Game>, Error> {
    let mut game_records = Vec::<GameRecord>::new();
    let mut reader = Reader::from_path(&input_filename)?;
    for result in reader.deserialize() {
        let mut record: GameRecord = result?;
        //record.set_result_from_current()?;
        game_records.push(record);
    }

    let mut games = Vec::<structures:Game>::new();
    let mut teams = HashMap::<String, structures::Team>::new();
    let team_name_order = table::lexicographical_team_order_from_games(&games);
    for game_record in game_records {
        let home_team_name = game.home();
        let away_team_name = game.away();
        let home_goals = game.home_goals();
        let away_goals = game.away_goals();

        match (home_goals, away_goals) {
            (Some(hg), Some(ag)) => {
                // Create or update home team
                if teams.contains_key(&home_team_name) {
                    let team = teams.get_mut(&home_team_name).unwrap();
                    let game = structures::Game::new()
                    team.update_from_game(hg, ag, true);
                } else {
                    let pos = team_name_order.iter().position(|ref r| **r == home_team_name).unwrap();
                    let mut team = structures::Team::new(&home_team_name, pos as u8);
                    team.update_from_game(hg, ag, true);
                    teams.insert(home_team_name, team);
                }
                // Create or update away team
                if teams.contains_key(&away_team_name) {
                    let team = teams.get_mut(&away_team_name).unwrap();
                    team.update_from_game(ag, hg, false);
                } else {
                    let pos = team_name_order.iter().position(|ref r| **r == away_team_name).unwrap();
                    let mut team = structures::Team::new(&away_team_name, pos as u8);
                    team.update_from_game(ag, hg, false);
                    teams.insert(away_team_name, team);
                }
                probability *= game.result_probability().unwrap();
            },
            _ => {},
        }
    }
    Ok(games)
}
