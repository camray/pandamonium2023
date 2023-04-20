struct Submission {
    score: f64,
    points_possible: f64,
}

struct FinalGrade {
    score: f64,
    possible: f64,
    percentage: f64,
}

// Takes a reference to a vector of submissions and returns a final grade
fn calculate_grade(submissions: &Vec<Submission>) -> FinalGrade {
    let mut total_score = 0.0;
    let mut total_possible = 0.0;

    for submission in submissions {
        total_score += submission.score;
        total_possible += submission.points_possible;
    }

    FinalGrade {
        score: total_score,
        possible: total_possible,
        percentage: total_score / total_possible,
    }
}

fn main() {
    let submissions = vec![
        Submission {
            score: 8.5,
            points_possible: 10.0,
        },
        Submission {
            score: 10.0,
            points_possible: 10.0,
        },
        Submission {
            score: 10.0,
            points_possible: 10.0,
        },
    ];

    let grade = calculate_grade(&submissions);
    println!("Final grade: {} out of {} or {}%", grade.score, grade.possible, grade.percentage);
}
