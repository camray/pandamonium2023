mod calculate_grade;
use calculate_grade::{calculate_grade, Submission};

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
    println!("Grade: {:#?}", grade);
}