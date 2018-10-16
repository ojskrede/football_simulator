//! Game related structs

use std::fmt;
use std::f32::consts::PI;
use failure::Error;

use team;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum GameResult {
    HomeWin,
    AwayWin,
    Draw,
}

enum ResultProbabilityMethod {
    Uniform,
    GameResult,
    GameResultAndPointDifference,
    GameResultAndWeightedPointDifference,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameRecord {
    #[serde(rename = "Runde")]
    round: u8,
    #[serde(rename = "Dato")]
    date: String,
    #[serde(rename = "Dag")]
    day: String,
    #[serde(rename = "Tid")]
    time: String,
    #[serde(rename = "Hjemmelag")]
    home_name: String,
    #[serde(rename = "Resultat")]
    result: String,
    #[serde(rename = "Bortelag")]
    away_name: String,
    #[serde(rename = "Bane")]
    pitch: String,
    #[serde(rename = "Turnering")]
    tournament: String,
    #[serde(rename = "Kampnummer")]
    game_number: usize,
}

impl GameRecord {
    pub fn round(&self) -> u8 {
        self.round
    }

    pub fn date(&self) -> String {
        self.date.clone()
    }

    pub fn day(&self) -> String {
        self.day.clone()
    }

    pub fn time(&self) -> String {
        self.time.clone()
    }

    pub fn home_name(&self) -> String {
        self.home_name.clone()
    }

    pub fn away_name(&self) -> String {
        self.away_name.clone()
    }

    pub fn pitch(&self) -> String {
        self.pitch.clone()
    }

    pub fn tournament(&self) -> String {
        self.tournament.clone()
    }

    pub fn game_number(&self) -> usize {
        self.game_number
    }

    pub fn home_goals(&self) -> Option<u8> {
        let goals: Vec<&str> = self.result.split("-").collect();
        match goals[0].trim_left().trim_right().parse::<u8>() {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }

    pub fn away_goals(&self) -> Option<u8> {
        let goals: Vec<&str> = self.result.split("-").collect();
        match goals[1].trim_left().trim_right().parse::<u8>() {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }
}

/// A planned, or played game (or match, but match is a reserved keyword in rust, so we use game in
/// stead). Optional fields are only Some when the game has been played.
#[derive(Debug, Clone)]
pub struct Game {
    round: u8,
    date: String,
    day: String,
    time: String,
    home_team: team::Team,
    away_team: team::Team,
    pitch: String,
    tournament: String,
    game_number: usize,
    played: bool,
    home_goals: Option<u8>,
    away_goals: Option<u8>,
    game_result: Option<GameResult>,
    result_probability: Option<f32>, // The probability that this outcome would occur
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.played {
            write!(f, "{:>3}{:>15}:   {:<16} vs   {:<16}: {:>5} - {:>5}, result: {:?} (prob = {})",
               self.round,
               self.date,
               self.home_team.name(),
               self.away_team.name(),
               self.home_goals.unwrap(),
               self.away_goals.unwrap(),
               self.game_result.clone().unwrap(),
               self.result_probability.unwrap(),
               )
        } else {
            write!(f, "{:>3}{:>15}:   {:<16} vs   {:<16}",
               self.round,
               self.date,
               self.home_team.name(),
               self.away_team.name(),
               )
        }
    }
}

impl Game {
    pub fn new_with(
        home_team: &team::Team,
        away_team: &team::Team,
        game_record: &GameRecord,
        ) -> Game {

        match (game_record.home_goals(), game_record.away_goals()) {
            (Some(hg), Some(ag)) => {
                let game_result = if hg > ag {
                    GameResult::HomeWin
                } else if hg < ag {
                    GameResult::AwayWin
                } else {
                    GameResult::Draw
                };

                let result_probability = compute_result_probability(
                    home_team,
                    away_team,
                    &game_result,
                    ResultProbabilityMethod::GameResultAndPointDifference,
                    );

                Game {
                    round: game_record.round(),
                    date: game_record.date(),
                    day: game_record.day(),
                    time: game_record.time(),
                    home_team: home_team.clone(),
                    away_team: away_team.clone(),
                    pitch: game_record.pitch(),
                    tournament: game_record.tournament(),
                    game_number: game_record.game_number(),
                    played: true,
                    home_goals: Some(hg),
                    away_goals: Some(ag),
                    game_result: Some(game_result),
                    result_probability: Some(result_probability),
                }
            },
            (Some(_), None) => {
                panic!("Home goals set, but away goals is not, for game {}",
                       game_record.game_number());
            },
            (None, Some(_)) => {
                panic!("Away goals set, but home goals is not, for game {}",
                       game_record.game_number())
            },
            (None, None) => {
                Game {
                    round: game_record.round(),
                    date: game_record.date(),
                    day: game_record.day(),
                    time: game_record.time(),
                    home_team: home_team.clone(),
                    away_team: away_team.clone(),
                    pitch: game_record.pitch(),
                    tournament: game_record.tournament(),
                    game_number: game_record.game_number(),
                    played: false,
                    home_goals: None,
                    away_goals: None,
                    game_result: None,
                    result_probability: None,
                }
            },
        }

    }

    pub fn round(&self) -> u8 {
        self.round
    }

    pub fn home_name(&self) -> String {
        self.home_team.name()
    }

    pub fn away_name(&self) -> String {
        self.away_team.name()
    }

    pub fn played(&self) -> bool {
        self.played
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
fn old_compute_result_weight(
    home: &team::Team,
    away: &team::Team,
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

/// Compute result probability. You have several alternatives on how to compute this (see
/// ResultProbabilityMethod).
///
/// ResultProbabilityMethod::Uniform
///     Assign 1/3 to home win, away win, and draw.
/// ResultProbabilityMethod::GameResult
///     Some fixed probability of home win, away win, and draw. E.g. (0.5, 0.3, 0.2).
/// ResultProbabilityMethod::GameResultAndPointDifference
///     Let R=GameResult, and D=relative point difference between home team and away team, this then
///     computes P(R, D) = P_r(R)P_d(D | R), for some prior P_r (as in GameResult option above),
///     and P_d which is assumed to be
///         R = Draw: normal distribution around d=0, truncated at (-1, 1)
///         R = HomeWin: 1 / (1 + exp(-4d)), truncated at (-1, 1)
///         R = AwayWin: 1 / (1 + exp(+4d)), truncated at (-1, 1)
///
fn compute_result_probability(
    home: &team::Team,
    away: &team::Team,
    result: &GameResult,
    method: ResultProbabilityMethod,
) -> f32 {
    match method {
        ResultProbabilityMethod::Uniform => {
            1.0 / 3.0
        },
        ResultProbabilityMethod::GameResult => {
            match result {
                GameResult::HomeWin => 0.5,
                GameResult::AwayWin => 0.3,
                GameResult::Draw => 0.2,
            }
        },
        ResultProbabilityMethod::GameResultAndPointDifference => {
            let max_num_played = home.sum_games_played().max(away.sum_games_played()) as f32;
            let point_difference = home.sum_points() as f32 - away.sum_points() as f32;
            let rpd = point_difference / max_num_played;
            let win_factor = 4.0;
            let draw_mean = 0.0;
            let draw_var = 0.2;
            match result {
                GameResult::HomeWin => 1.0 / (1.0 + (win_factor * rpd).exp()),
                GameResult::AwayWin => 1.0 / (1.0 + (win_factor * rpd).exp()),
                GameResult::Draw => 1.0 / (2.0 * PI * draw_var).sqrt() * (-(rpd - draw_mean).powi(2) / (2.0 * draw_var)).exp(), // TODO: Change to truncated normal
            }
        },
        ResultProbabilityMethod::GameResultAndWeightedPointDifference => {
            // TODO: Implement this
            let max_num_played = home.sum_games_played().max(away.sum_games_played()) as f32;
            let point_difference = home.sum_points() as f32 - away.sum_points() as f32;
            let rpd = point_difference / max_num_played;
            let win_factor = 4.0;
            let draw_mean = 0.0;
            let draw_var = 0.2;
            match result {
                GameResult::HomeWin => 1.0 / (1.0 + (win_factor * rpd).exp()),
                GameResult::AwayWin => 1.0 / (1.0 + (win_factor * rpd).exp()),
                GameResult::Draw => 1.0 / (2.0 * PI * draw_var).sqrt() * (-(rpd - draw_mean).powi(2) / (2.0 * draw_var)).exp(), // TODO: Change to truncated normal
            }
        },
    }
}
