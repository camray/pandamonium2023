use serde::{Deserialize, Serialize};
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct Submission {
    score: f64,
    points_possible: f64,
}

#[derive(Serialize)]
struct Grade {
    score: f64,
    possible: f64,
    percentage: f64,
}

fn deserialize_submissions(json: &str) -> Result<Vec<Submission>, serde_json::Error> {
    serde_json::from_str(json) as Result<Vec<Submission>, serde_json::Error>
}

#[wasm_bindgen]
pub fn calculate_grade(submissions_json: &str) -> JsValue {
    let submissions =
        deserialize_submissions(submissions_json).expect("Failed to deserialize submissions");

    let mut total_score = 0.0;
    let mut total_possible = 0.0;

    for submission in submissions {
        total_score += submission.score;
        total_possible += submission.points_possible;
    }

    let grade = Grade {
        score: total_score,
        possible: total_possible,
        percentage: total_score / total_possible,
    };

    serde_wasm_bindgen::to_value(&grade).unwrap()
}
