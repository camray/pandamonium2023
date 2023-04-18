#![allow(dead_code)]

struct Submission {
    id: u64,
    user_id: u64,
    score: f64,
    points_possible: f64,
    workflow_state: String,
}

#[derive(Debug)]
struct Grade {
    score: f64,
    possible: f64,
    percentage: f64,
}

// Takes a reference to a vector of submissions and returns a final grade
fn calculate_grade(submissions: &Vec<Submission>) -> Grade {
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

fn main() {
    let submissions = vec![
        Submission {
            id: 1,
            user_id: 1,
            score: 5.0,
            points_possible: 10.0,
            workflow_state: "graded".to_string(),
        },
        Submission {
            id: 2,
            user_id: 1,
            score: 10.0,
            points_possible: 10.0,
            workflow_state: "graded".to_string(),
        },
        Submission {
            id: 3,
            user_id: 1,
            score: 10.0,
            points_possible: 10.0,
            workflow_state: "graded".to_string(),
        },
    ];

    let grade = calculate_grade(&submissions);
    println!("Final grade: {} out of {} or {}%", grade.score, grade.possible, grade.percentage);
}