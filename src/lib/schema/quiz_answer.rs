use crate::types::id::{OptionId, QuestionId, SubmissionId};
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "An answer")]
pub struct QuizAnswer {
    pub quiz_submission_id: SubmissionId,
    pub quiz_question_id: QuestionId,
    pub quiz_option_id: OptionId,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A new answer")]
pub struct QuizAnswerInput {
    pub quiz_submission_id: SubmissionId,
    pub quiz_question_id: QuestionId,
    pub quiz_option_id: OptionId,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "An updated answer")]
pub struct QuizAnswerUpdate {
    pub quiz_submission_id: SubmissionId,
    pub quiz_question_id: QuestionId,
    pub old_quiz_option_id: OptionId,
    pub new_quiz_option_id: OptionId,
}
