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


/// A planned, or played game (or match, but match is a reserved keyword in rust, so we use game in
/// stead). Optional fields are only Some when the game has been played.
#[derive(Debug, Clone)]
pub struct Game {
    round: u8,
    date: String,
    day: String,
    time: String,
    home: String,
    result: String,
    away: String,
    pitch: String,
    tournament: String,
    game_number: usize,
    home_goals: Option<u8>,
    away_goals: Option<u8>,
    game_result: Option<GameResult>,
    result_probability: Option<f32>,
    result_weight: Option<f32>,
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
    pub fn new_with(
        game_record: &data_io::GameRecord,
        home_team: &Team,
        away_team: &Team,
        ) -> Game {

    }

    pub fn get_result_from_record(
        game_record: &data_io::GameRecord,
        ) -> Result<(), Error> {
        let goals: Vec<&str> = game_record.result.split("-").collect();

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

    fn num_played(&self) -> u8 {
        self.num_played
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

    pub fn sum_games_played(&self) -> u8 {
        self.sum_result.num_played()
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

/// Compute a weight that should represent how surprising the result is. Higher weights to more
/// surprising results than lower weights.
///
/// Currently it tries to take into account the point difference between the teams before the
/// game is played, and if it is a home win, away win, or draw. If we define the relative point
/// difference (rpd) between the teams as (winner.points - loser.points) / max(1, max_points),
/// and max_points = 3 * max(winner.rounds_played, loser.rounds_played) (for draw we have the
/// same except (home.points - away.points). Then, this function gives weights in the following
/// order (higher to lower)
///
/// ```
///     rpd = -1 and AwayWin
///     rpd = -1 and HomeWin
///     rpd = -1 and Draw
///     rpd = 0 and AwayWin
///     rpd = 0 and Draw
///     rpd = 0 and HomeWin
///     rpd = 1 and Draw
///     rpd = 1 and AwayWin
///     rpd = 1 and HomeWin
/// ```
///
/// It could be argued that a draw when rdf=0 is more surprising than one of the teams winning,
/// but the above gives a simple solution.
///
fn compute_result_weight(
    home: &Team,
    away: &Team,
    result: &GameResult,
) -> f32 {
    let max_num_played = home.sum_games_played().max(away.sum_games_played()) as f32;
    match result {
        GameResult::HomeWin => {
            let point_difference = home.sum_points() as f32 - away.sum_points() as f32;
            let rpd = point_difference / max_num_played;
            (1.0 - rpd + 0.1 * (rpd - 1.0)) / 2.0
        },
        GameResult::AwayWin => {
            let point_difference = away.sum_points() as f32 - home.sum_points() as f32;
            let rpd = point_difference / max_num_played;
            (1.0 - rpd + 0.1 * (rpd + 1.0)) / 2.0
        },
        GameResult::Draw => {
            let point_difference = home.sum_points() as f32 - away.sum_points() as f32;
            let rpd = point_difference / max_num_played;
            (1.0 - 0.4 * rpd) / 2.0
        },
    }
}
