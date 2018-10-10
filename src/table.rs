//! Table
//!

use std::collections::HashMap;

use structures;

#[derive(Clone, Debug)]
pub struct Table {
    standings: HashMap<String, structures::Team>,
    sorted_standings: Vec<(String, structures::Team)>,
    probability: f32,
}

impl Table {
    pub fn new_with(games: &[structures::Game]) -> Table {
        let mut table = HashMap::<String, structures::Team>::new();
        let mut probability = 1.0;

        for game in games {
            let home_team_name = game.home();
            let away_team_name = game.away();
            let home_goals = game.home_goals();
            let away_goals = game.away_goals();

            match (home_goals, away_goals) {
                (Some(hg), Some(ag)) => {
                    // Create or update home team
                    if table.contains_key(&home_team_name) {
                        let team = table.get_mut(&home_team_name).unwrap();
                        team.update_from_game(hg, ag, true);
                    } else {
                        let mut team = structures::Team::new(&home_team_name);
                        team.update_from_game(hg, ag, true);
                        table.insert(home_team_name, team);
                    }
                    // Create or update away team
                    if table.contains_key(&away_team_name) {
                        let team = table.get_mut(&away_team_name).unwrap();
                        team.update_from_game(ag, hg, false);
                    } else {
                        let mut team = structures::Team::new(&away_team_name);
                        team.update_from_game(ag, hg, false);
                        table.insert(away_team_name, team);
                    }
                    probability *= game.result_probability().unwrap();
                },
                _ => {},
            }
        }

        let mut table_vec: Vec<(String, structures::Team)> = table.iter().map(|(a, b)| (a.clone(), b.clone())).collect();
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

        Table {
            standings: table.clone(),
            sorted_standings: table_vec,
            probability: probability,
        }
    }

    pub fn get_updated(&self, games: &[structures::Game]) -> Table {
        let mut table = self.standings.clone();
        let mut probability = self.probability;

        for game in games {
            // We assume that all teams are present
            // Update home team
            {
                table.get_mut(&game.home()).unwrap()
                     .update_from_game(game.home_goals().unwrap(), game.away_goals().unwrap(), true);
            }
            // Update away team
            {
                table.get_mut(&game.away()).unwrap()
                     .update_from_game(game.away_goals().unwrap(), game.home_goals().unwrap(), false);
            }
            probability *= game.result_probability().unwrap();
        }

        let mut table_vec: Vec<(String, structures::Team)> = table.iter().map(|(a, b)| (a.clone(), b.clone())).collect();
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

        Table {
            standings: table.clone(),
            sorted_standings: table_vec,
            probability: probability,
        }
    }

    pub fn print_table(&self) {
        println!("Table with probability {}", self.probability);
        for (_, team) in self.sorted_standings.iter() {
            println!("{}", team.total_as_table_row());
        }
    }

    pub fn probability(&self) -> f32 {
        self.probability
    }
}
