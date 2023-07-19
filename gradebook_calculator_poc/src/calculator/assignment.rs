use crate::calculator::submission::Submission;

pub enum AssignmentWorkflowState {
    Published,
    Unpublished,
}

pub enum AssignmentGradingType {
    Points,
    Percent,
    LetterGrade,
    GpaScale,
    PassFail,
    NotGraded,
}

pub enum AssignmentLatePolicyStatus {
    Missing,
    Late,
    Extended,
}

pub enum AssignmentSimilarityScoreStatus {
    Error,
    Pending,
    Scored
}

pub struct AssignmentSimilarityScore {
  similarity_score: f32,
  status: AssignmentSimilarityScoreStatus
}

pub enum AssignmentSubmissionType {
    BasicLtiLaunch,
    DiscussionTopic,
    ExternalTool,
    MediaRecording,
    OnPaper,
    OnlineQuiz,
    OnlineTextEntry,
    OnlineUpload,
    OnlineUrl,
    WikiPage,
}

pub struct Assignment {
    assignment_id: String,
    points_possible: u32,
    submission_type: AssignmentSubmissionType,
    anonymize_students: bool,
}
