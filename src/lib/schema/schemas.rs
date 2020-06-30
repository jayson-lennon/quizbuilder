//use chrono::{DateTime, Utc};
//
//use crate::types::{
//    id::{OptionId, QuestionId, QuizId, SubmissionId, UserId},
//    time::Duration,
//};
//
//#[derive(juniper::GraphQLObject)]
//#[graphql(description = "A quiz")]
//struct Quiz {
//    quiz_id: QuizId,
//    owner: UserId,
//    name: Option<String>,
//    date_created: DateTime<Utc>,
//    open_date: DateTime<Utc>,
//    close_date: Option<DateTime<Utc>>,
//    duration: Option<Duration>,
//    shortcode: Option<String>,
//}
//
//#[derive(juniper::GraphQLObject)]
//#[graphql(description = "A question")]
//struct QuizQuestion {
//    quiz_question_id: QuestionId,
//    quiz_id: QuizId,
//    question_data: String,
//    position: Option<i32>,
//}
//
//#[derive(juniper::GraphQLObject)]
//#[graphql(description = "An option")]
//struct QuizOption {
//    quiz_option_id: OptionId,
//    quiz_question_id: QuestionId,
//    option_data: String,
//    is_correct: bool,
//    position: Option<i32>,
//}
//
//#[derive(juniper::GraphQLObject)]
//#[graphql(description = "A submission")]
//struct QuizSubmission {
//    quiz_submission_id: SubmissionId,
//    identity: String,
//    quiz_id: QuizId,
//    start_date: DateTime<Utc>,
//    finish_date: Option<DateTime<Utc>>,
//}
//
//#[derive(juniper::GraphQLObject)]
//#[graphql(description = "An answer")]
//struct QuizAnswer {
//    quiz_submission_id: SubmissionId,
//    quiz_question_id: QuestionId,
//    quiz_option_id: OptionId,
//}
//
