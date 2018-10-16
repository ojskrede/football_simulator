//! Misc useful structs
//!

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
