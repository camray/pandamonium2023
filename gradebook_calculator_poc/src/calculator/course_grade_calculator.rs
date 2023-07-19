use std::collections::HashMap;

use super::{assignment_group::AssignmentGroup, submission::Submission};

pub enum WeightingScheme {
    Percent,
    Points,
}

pub struct GradingPeriod {
    pub id: String,
    pub title: String,
    pub close_date: u32,
    pub end_date: u32,
    pub is_closed: bool,
    pub is_last: bool,
    pub start_date: u32,
    pub weight: f32,
}

pub struct GradingPeriodSet {
    pub id: String,
    pub created_at: u32,
    pub display_totals_for_all_grading_periods: bool,
    pub enrollment_term_ids: Option<Vec<String>>,
    pub grading_periods: Vec<GradingPeriod>,
    pub is_closed: Option<bool>,
    pub title: String,
    pub weighted: bool,
}

pub struct DueDate {
    pub due_at: Option<String>,
    pub grading_period_id: Option<String>,
    pub in_closed_grading_period: bool,
}

struct CalculationOptions {
    weight_grading_periods: bool,
    weight_assignment_groups: bool,
    ignore_unposted_anonymous: bool,
}

pub fn calculate(
    submissions: Vec<Submission>,
    assignment_groups: HashMap<u32, AssignmentGroup>,
    weighting_scheme: Option<String>,
    ignore_unposted_anonymous: bool,
    grading_period_set: Option<GradingPeriodSet>,
    effective_due_dates: Option<HashMap<String, DueDate>>,
) {
    let mut weight_assignment_groups = false;
    let mut weight_grading_periods = false;

    if weighting_scheme.is_some() && weighting_scheme.unwrap() == "percent" {
        weight_assignment_groups = true;
    }

    if grading_period_set.is_some() && grading_period_set.as_ref().unwrap().weighted {
        weight_grading_periods = true;
    }

    let calculation_options = CalculationOptions {
        weight_grading_periods,
        weight_assignment_groups,
        ignore_unposted_anonymous,
    };

    if grading_period_set.is_some() && effective_due_dates.is_some() {
        // Calculate with grading periods
    }

    calculate_without_grading_periods(&submissions, assignment_groups, calculation_options)
}

fn calculate_without_grading_periods(
    submissions: &Vec<Submission>,
    assignment_groups: HashMap<u32, AssignmentGroup>,
    options: CalculationOptions,
) {
    let assignment_group_grades = assignment_groups.iter().map(|assignment_group| {
        calculate_group(submissions, assignment_group.1, options.ignore_unposted_anonymous)
    });
}

fn calculate_group(submissions: &Vec<Submission>, assignment_group: &AssignmentGroup, ignore_unposted_anonymous: bool) {
    
}