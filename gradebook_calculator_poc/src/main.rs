use calculator::{
    assignment::Assignment, assignment_group::AssignmentGroup, enrollment::Enrollment, enrollment::EnrollmentWorkflowState,
    submission::Submission, course::Course, user::User, assignment::AssignmentWorkflowState
};
mod calculator;

fn main() {
    // let course = Course {
    //     id: 1,
    //     enrollments: vec![Enrollment {
    //         id: 1,
    //         user: User {
    //             id: 1,
    //             name: "Jim".into()
    //         },
    //         workflow_state: EnrollmentWorkflowState::Active
    //     }],
    //     assignment_groups: vec![
    //         AssignmentGroup {
    //             id: 1,
    //             weight: 1.0,
    //             assignments: vec![
    //                 Assignment {
    //                     id: 1,
    //                     due_at: 0,
    //                     points_possible: 100,
    //                     name: "New Assignment".into(),
    //                     workflow_state: AssignmentWorkflowState::Published,
    //                     submissions: vec![
    //                         Submission {
    //                             id: 1,
    //                             user_id: 1,
    //                             score: 100.0,
    //                             submitted_at: 0
    //                         }
    //                     ]
    //                 }
    //             ]
    //         }
    //     ]
    // };
}