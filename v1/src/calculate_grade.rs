
pub struct Submission {
    pub id: u64,
    pub user_id: u64,
    pub score: f64,
    pub points_possible: f64,
    pub workflow_state: String,
}

#[derive(Debug)]
pub struct Grade {
    pub score: f64,
    pub possible: f64,
    pub percentage: f64,
}

// Takes a reference to a vector of submissions and returns a final grade
pub fn calculate_grade(submissions: &Vec<Submission>) -> Grade {
    let mut total_score = 0.0;
    let mut total_possible = 0.0;

    for submission in submissions {
        total_score += submission.score;
        total_possible += submission.points_possible;
    }

    Grade {
        score: total_score,
        possible: total_possible,
        percentage: total_score / total_possible,
    }
}
