use crate::calculator::user::User;

pub enum EnrollmentWorkflowState {
    Active,
    Deleted,
    Inactive,
    Completed,
    Invited,
}

pub struct Enrollment {
    pub id: u64,
    pub user: User,
    pub workflow_state: EnrollmentWorkflowState,
}
