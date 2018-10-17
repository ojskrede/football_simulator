//! Misc useful structs
//!

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TeamRecord {
    club: String,
    played_home: u8,
    won_home: u8,
    draw_home: u8,
    lost_home: u8,
    points_home: u8,
    played_away: u8,
    won_away: u8,
    draw_away: u8,
    lost_away: u8,
    points_away: u8,
    played_total: u8,
    won_total: u8,
    draw_total: u8,
    lost_total: u8,
    points_total: u8,
}


#[derive(Debug, Clone)]
struct Aggregate {
    num_played: u8,
    num_won: u8,
    num_draw: u8,
    num_lost: u8,
    goals_scored: u8,
    goals_conceded: u8,
    goal_difference: i32,
    points: u8,
}

impl Default for Aggregate {
    fn default() -> Aggregate {
        Aggregate {
            num_played: 0,
            num_won: 0,
            num_draw: 0,
            num_lost: 0,
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
            self.num_won += 1;
        } else if goals_scored == goals_conceded {
            self.points += 1;
            self.num_draw += 1;
        } else {
            self.num_lost += 1;
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

    fn num_won(&self) -> u8 {
        self.num_won
    }

    fn num_draw(&self) -> u8 {
        self.num_draw
    }

    fn num_lost(&self) -> u8 {
        self.num_lost
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

    pub fn num_played_total(&self) -> u8 {
        self.sum_result.num_played()
    }

    pub fn num_won_total(&self) -> u8 {
        self.sum_result.num_won()
    }

    pub fn num_draw_total(&self) -> u8 {
        self.sum_result.num_draw()
    }

    pub fn num_lost_total(&self) -> u8 {
        self.sum_result.num_lost()
    }

    pub fn num_points_total(&self) -> u8 {
        self.sum_result.points()
    }

    pub fn num_played_home(&self) -> u8 {
        self.home_result.num_played()
    }

    pub fn num_won_home(&self) -> u8 {
        self.home_result.num_won()
    }

    pub fn num_draw_home(&self) -> u8 {
        self.home_result.num_draw()
    }

    pub fn num_lost_home(&self) -> u8 {
        self.home_result.num_lost()
    }

    pub fn num_points_home(&self) -> u8 {
        self.home_result.points()
    }

    pub fn num_played_away(&self) -> u8 {
        self.away_result.num_played()
    }

    pub fn num_won_away(&self) -> u8 {
        self.away_result.num_won()
    }

    pub fn num_draw_away(&self) -> u8 {
        self.away_result.num_draw()
    }

    pub fn num_lost_away(&self) -> u8 {
        self.away_result.num_lost()
    }

    pub fn num_points_away(&self) -> u8 {
        self.away_result.points()
    }

    pub fn get_record(&self) -> TeamRecord {
        TeamRecord {
            club: self.name(),
            played_home: self.num_played_home(),
            won_home: self.num_won_home(),
            draw_home: self.num_draw_home(),
            lost_home: self.num_lost_home(),
            points_home: self.num_points_home(),
            played_away: self.num_played_away(),
            won_away: self.num_won_away(),
            draw_away: self.num_draw_away(),
            lost_away: self.num_lost_away(),
            points_away: self.num_played_away(),
            played_total: self.num_played_total(),
            won_total: self.num_won_total(),
            draw_total: self.num_draw_total(),
            lost_total: self.num_lost_total(),
            points_total: self.num_points_total(),
        }
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
