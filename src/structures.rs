//! Misc useful structs
//!

use std::fmt;
use failure::Error;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum GameResult {
    HomeWin,
    AwayWin,
    Draw,
}

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
    home: String,
    #[serde(rename = "Resultat")]
    result: String,
    #[serde(rename = "Bortelag")]
    away: String,
    #[serde(rename = "Bane")]
    field: String,
    #[serde(rename = "Turnering")]
    tournament: String,
    #[serde(rename = "Kampnummer")]
    game_number: usize,
    home_goals: Option<u8>,
    away_goals: Option<u8>,
    game_result: Option<GameResult>,
    result_probability: Option<f32>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.result_probability.is_some() {
            write!(f, "{:>3}{:>15}:   {:<16} vs   {:<16}: {:>10}, result: {:?} (prob = {})",
               self.round,
               self.date,
               self.home,
               self.away,
               self.result,
               self.game_result.clone().unwrap(),
               self.result_probability.unwrap(),
               )
        } else {
            write!(f, "{:>3}{:>15}:   {:<16} vs   {:<16}: {:>10}",
               self.round,
               self.date,
               self.home,
               self.away,
               self.result,
               )
        }
    }
}

impl Game {
    pub fn set_result_from_current(&mut self) -> Result<(), Error> {
        let goals: Vec<&str> = self.result.split("-").collect();

        let home_goals = match goals[0].trim_left().trim_right().parse::<u8>() {
            Ok(val) => Some(val),
            Err(_) => None,
        };
        self.home_goals = home_goals;

        let away_goals = match goals[1].trim_left().trim_right().parse::<u8>() {
            Ok(val) => Some(val),
            Err(_) => None,
        };
        self.away_goals = away_goals;

        if self.home_goals != None && self.away_goals != None {
            if self.home_goals.unwrap() > self.away_goals.unwrap() {
                self.game_result = Some(GameResult::HomeWin);
            } else if self.home_goals.unwrap() < self.away_goals.unwrap() {
                self.game_result = Some(GameResult::AwayWin);
            } else {
                assert_eq!(self.home_goals.unwrap(), self.away_goals.unwrap(), "Something wrong");
                self.game_result = Some(GameResult::Draw);
            }
            self.result_probability = Some(1.0);
        }

        Ok(())
    }

    pub fn set_result_from_simulated(
        &mut self,
        result: &GameResult,
        probability: f32,
        home_goals: u8,
        away_goals: u8,
    ) -> Result<(), Error> {
        self.home_goals = Some(home_goals);
        self.away_goals = Some(away_goals);
        self.game_result = Some(result.clone());
        self.result_probability = Some(probability);

        if self.home_goals.unwrap() > self.away_goals.unwrap() {
            assert_eq!(self.game_result, Some(GameResult::HomeWin));
        } else if self.home_goals.unwrap() < self.away_goals.unwrap() {
            assert_eq!(self.game_result, Some(GameResult::AwayWin));
        } else {
            assert_eq!(self.home_goals.unwrap(), self.away_goals.unwrap(), "Something wrong");
            assert_eq!(self.game_result, Some(GameResult::Draw));
        }

        Ok(())
    }

    pub fn round(&self) -> u8 {
        self.round
    }

    pub fn home(&self) -> String {
        self.home.clone()
    }

    pub fn away(&self) -> String {
        self.away.clone()
    }

    pub fn home_goals(&self) -> Option<u8> {
        self.home_goals.clone()
    }

    pub fn away_goals(&self) -> Option<u8> {
        self.away_goals.clone()
    }

    pub fn game_result(&self) -> Option<GameResult> {
        self.game_result.clone()
    }

    pub fn result_probability(&self) -> Option<f32> {
        self.result_probability
    }
}


#[derive(Debug, Clone)]
struct Aggregate {
    num_played: u8,
    goals_scored: u8,
    goals_conceded: u8,
    goal_difference: i32,
    points: u8,
}

impl Default for Aggregate {
    fn default() -> Aggregate {
        Aggregate {
            num_played: 0,
            goals_scored: 0,
            goals_conceded: 0,
            goal_difference: 0,
            points: 0,
        }
    }
}

impl Aggregate {
    fn update_from_game(&mut self, goals_scored: u8, goals_conceded: u8) {
        self.num_played += 1;
        self.goals_scored += goals_scored;
        self.goals_conceded += goals_conceded;
        self.goal_difference += goals_scored as i32 - goals_conceded as i32;
        if goals_scored > goals_conceded {
            self.points += 3;
        } else if goals_scored == goals_conceded {
            self.points += 1;
        }
    }

    fn as_table_row(&self) -> String {
        format!("{:>8}: {:>4} - {:>4} = {:>4}, :: {:>4}",
                self.num_played, self.goals_scored, self.goals_conceded, self.goal_difference, self.points)
    }

    fn points(&self) -> u8 {
        self.points
    }

    fn goal_difference(&self) -> i32 {
        self.goal_difference
    }

    fn goals_scored(&self) -> u8 {
        self.goals_scored
    }
}

#[derive(Debug, Clone)]
pub struct Team {
    name: String,
    home_result: Aggregate,
    away_result: Aggregate,
    sum_result: Aggregate,
    sum_points: u8,
    sum_goal_difference: i32,
    sum_goals_scored: u8,
    lexicographical_position: u8,
}

impl Team {
    pub fn new(name: &str, position: u8) -> Team {
        Team {
            name: String::from(name),
            home_result: Aggregate::default(),
            away_result: Aggregate::default(),
            sum_result: Aggregate::default(),
            sum_points: 0,
            sum_goal_difference: 0,
            sum_goals_scored: 0,
            lexicographical_position: position,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn lexicographical_position(&self) -> u8 {
        self.lexicographical_position
    }

    pub fn sum_points(&self) -> u8 {
        self.sum_points
    }

    pub fn sum_goal_difference(&self) -> i32 {
        self.sum_goal_difference
    }

    pub fn sum_goals_scored(&self) -> u8 {
        self.sum_goals_scored
    }

    pub fn update_from_game(&mut self, goals_scored: u8, goals_conceded: u8, home: bool) {
        if home {
            self.home_result.update_from_game(goals_scored, goals_conceded);
        } else {
            self.away_result.update_from_game(goals_scored, goals_conceded);
        }
        self.sum_result.update_from_game(goals_scored, goals_conceded);
        self.sum_points = self.sum_result.points();
        self.sum_goal_difference = self.sum_result.goal_difference();
        self.sum_goals_scored = self.sum_result.goals_scored();
    }

    pub fn total_as_table_row(&self) -> String {
        format!("{:<30} | {} | {} || {}",
                self.name,
                self.home_result.as_table_row(),
                self.away_result.as_table_row(),
                self.sum_result.as_table_row())
    }
}
