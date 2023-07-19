use crate::calculator::assignment::Assignment;


pub struct AssignmentGroup {
    pub id: String,
    pub group_weight: f32,
    pub rules: AssignmentGroupRules,
    pub assignments: Vec<Assignment>,
}

pub struct AssignmentGroupRules {
    drop_highest: u32,
    drop_lowest: u32,
    never_drop: Vec<String>
}