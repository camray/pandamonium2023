use crate::calculator::{assignment_group::AssignmentGroup, enrollment::Enrollment};

pub struct Course {
    pub id: u64,
    pub enrollments: Vec<Enrollment>,
    pub assignment_groups: Vec<AssignmentGroup>,
}
