//! Handle input output
//!

use std::path::Path;
use std::fmt;

use failure::Error;
use csv::Reader;

#[derive(Debug, Deserialize)]
enum Winner {
    Home,
    Away,
    Draw,
}

#[derive(Debug, Deserialize)]
pub struct Match {
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
    match_number: usize,
    home_goals: Option<u8>,
    away_goals: Option<u8>,
    winner: Option<Winner>,
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>3}{:>15}:   {:<16} vs   {:<16}: {:>10}: Winner: {:?}",
               self.round, self.date, self.home, self.away, self.result, self.winner)
    }
}

impl Match {
    fn set_result(&mut self) -> Result<(), Error> {
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
                self.winner = Some(Winner::Home);
            } else if self.home_goals.unwrap() < self.away_goals.unwrap() {
                self.winner = Some(Winner::Away);
            } else {
                assert!(self.home_goals.unwrap() == self.away_goals.unwrap(), "Something wrong");
                self.winner = Some(Winner::Draw);
            }
        }

        Ok(())
    }
}

pub fn import_matches(input_filename: &Path) -> Result<Vec<Match>, Error> {
    let mut matches = Vec::<Match>::new();
    let mut reader = Reader::from_path(&input_filename)?;
    for result in reader.deserialize() {
        let mut record: Match = result?;
        record.set_result()?;
        matches.push(record);
    }
    Ok(matches)
}
