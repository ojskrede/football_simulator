//! Handle input output
//!

use std::collections::HashMap;
use std::path::Path;

use failure::Error;
use csv::Reader;

use team;
use game;
use table;

pub fn import_games(
    input_filename: &Path,
    ) -> Result<(Vec<game::Game>, HashMap<String, team::Team>), Error> {
    let mut game_records = Vec::<game::GameRecord>::new();
    let mut reader = Reader::from_path(&input_filename)?;
    for result in reader.deserialize() {
        let mut record: game::GameRecord = result?;
        //record.set_result_from_current()?;
        game_records.push(record);
    }

    let mut games = Vec::<game::Game>::new();
    let mut teams = HashMap::<String, team::Team>::new();
    let team_name_order = table::lexicographical_team_order_from_games(&game_records);
    for game_record in game_records {
        let home_team_name = game_record.home_name();
        let away_team_name = game_record.away_name();
        //let home_goals = game_record.home_goals();
        //let away_goals = game_record.away_goals();

        let mut home_team = if teams.contains_key(&home_team_name) {
            teams.get(&home_team_name).unwrap().clone()
        } else {
            let pos = team_name_order.iter().position(|ref r| **r == home_team_name).unwrap();
            let team = team::Team::new(&home_team_name, pos as u8);
            teams.insert(home_team_name, team.clone());
            team
        };

        let mut away_team = if teams.contains_key(&away_team_name) {
            teams.get(&away_team_name).unwrap().clone()
        } else {
            let pos = team_name_order.iter().position(|ref r| **r == away_team_name).unwrap();
            let team = team::Team::new(&away_team_name, pos as u8);
            teams.insert(away_team_name, team.clone());
            team
        };

        let game = game::Game::new_with(&home_team, &away_team, &game_record);
        if game.played() {
            home_team.update_from_game(game.home_goals().unwrap(), game.away_goals().unwrap(), true);
            away_team.update_from_game(game.away_goals().unwrap(), game.home_goals().unwrap(), false);
        }
        games.push(game.clone());
    }
    Ok((games, teams))
}
