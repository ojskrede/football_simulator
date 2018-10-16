//! Table
//!

use std::collections::HashMap;

use game;
use team;

pub fn lexicographical_team_order_from_table(table: &Table) -> [String; 14] {
    let mut names_vec = Vec::<String>::new();
    for (team_name, _) in table.team_positions() {
        names_vec.push(team_name);
    }
    names_vec.sort_by(|a, b| a.cmp(&b));
    let mut names_array = [
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
    ];
    names_array.clone_from_slice(names_vec.as_slice());
    names_array
}

pub fn lexicographical_team_order_from_games(games: &[game::GameRecord]) -> [String; 14] {
    let mut names_vec = Vec::<String>::new();
    for game in games {
        if !names_vec.contains(&game.home_name()) {
            names_vec.push(game.home_name());
        }
        if !names_vec.contains(&game.away_name()) {
            names_vec.push(game.away_name());
        }
    }
    names_vec.sort_by(|a, b| a.cmp(&b));
    let mut names_array = [
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
    ];
    names_array.clone_from_slice(names_vec.as_slice());
    names_array
}

#[derive(Clone, Debug)]
pub struct Table {
    standings: HashMap<String, team::Team>,
    sorted_standings: Vec<(String, team::Team)>,
    team_positions: Vec<(String, u8)>,
    played_games: Vec<game::Game>,
    probability: f32,
}

impl Table {
    pub fn new_with(teams: &[team::Team], played_games: &[game::Game]) -> Table {
        let mut table = HashMap::<String, team::Team>::new();
        for team in teams.iter() {
            table.insert(team.name(), team.clone());
        }

        let mut table_vec: Vec<(String, team::Team)> = table.iter().map(|(a, b)| (a.clone(), b.clone())).collect();
        table_vec.sort_by(|a, b|
            if b.1.sum_points() == a.1.sum_points() {
                if b.1.sum_goal_difference() == a.1.sum_goal_difference() {
                    b.1.sum_goals_scored().cmp(&a.1.sum_goals_scored())
                } else {
                    b.1.sum_goal_difference().cmp(&a.1.sum_goal_difference())
                }
            } else {
                b.1.sum_points().cmp(&a.1.sum_points())
            }
        );

        let mut team_positions = Vec::<(String, u8)>::new();
        for (pos, (name, _)) in table_vec.iter().enumerate() {
            team_positions.push((name.clone(), pos as u8));
        }

        Table {
            standings: table.clone(),
            sorted_standings: table_vec,
            team_positions: team_positions,
            played_games: played_games.to_vec(),
            probability: 1.0,
        }
    }

    pub fn get_updated(&self, games: &[game::Game]) -> Table {
        let mut table = self.standings.clone();
        let mut probability = self.probability;

        for game in games {
            // We assume that all teams are present
            // Update home team
            {
                table.get_mut(&game.home_name())
                    .unwrap()
                    .update_from_game(
                        game.home_goals().unwrap(),
                        game.away_goals().unwrap(),
                        true,
                        );
            }
            // Update away team
            {
                table.get_mut(&game.away_name())
                    .unwrap()
                    .update_from_game(
                        game.away_goals().unwrap(),
                        game.home_goals().unwrap(),
                        false,
                        );
            }
            probability *= game.result_probability().unwrap();
        }

        let mut table_vec: Vec<(String, team::Team)> = table
            .iter()
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect();
        table_vec.sort_by(|a, b|
            if b.1.sum_points() == a.1.sum_points() {
                if b.1.sum_goal_difference() == a.1.sum_goal_difference() {
                    b.1.sum_goals_scored().cmp(&a.1.sum_goals_scored())
                } else {
                    b.1.sum_goal_difference().cmp(&a.1.sum_goal_difference())
                }
            } else {
                b.1.sum_points().cmp(&a.1.sum_points())
            }
        );

        let mut team_positions = Vec::<(String, u8)>::new();
        for (pos, (name, _)) in table_vec.iter().enumerate() {
            team_positions.push((name.clone(), pos as u8));
        }

        let mut extended_games = self.played_games.clone();
        extended_games.extend_from_slice(games);

        Table {
            standings: table.clone(),
            sorted_standings: table_vec,
            team_positions: team_positions,
            played_games: extended_games,
            probability: probability,
        }
    }

    pub fn print_table(&self) {
        println!("Table with probability {}", self.probability);
        for (_, team) in self.sorted_standings.iter() {
            println!("{}", team.total_as_table_row());
        }
    }

    pub fn print_latest_round(&self) {
        let num_played_games = self.played_games.len();
        let latest_round = &self.played_games[(num_played_games-7)..];
        println!("Round {}", latest_round[0].round());
        for game in latest_round.iter() {
            println!("{}", game);
        }
    }

    pub fn probability(&self) -> f32 {
        self.probability
    }

    pub fn team_positions(&self) -> Vec<(String, u8)> {
        self.team_positions.clone()
    }

    pub fn sorted_standings(&self) -> Vec<(String, team::Team)> {
        self.sorted_standings.clone()
    }
}
