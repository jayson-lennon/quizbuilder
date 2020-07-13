use crate::schema::{FullQuizQuestionInput, QuizQuestion};
use crate::types::id::{QuizId, UserId};
use crate::types::time::Duration;
use chrono::{DateTime, Utc};
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject, Debug)]
#[graphql(description = "A quiz")]
pub struct Quiz {
    pub quiz_id: QuizId,
    pub name: Option<String>,
    pub owner: UserId,
    pub date_created: DateTime<Utc>,
    pub open_date: DateTime<Utc>,
    pub close_date: Option<DateTime<Utc>>,
    pub duration: Option<Duration>,
    pub shortcode: String,
    pub questions: Vec<QuizQuestion>,
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "Quiz input")]
pub struct QuizInput {
    pub name: Option<String>,
    pub owner: UserId,
    pub open_date: DateTime<Utc>,
    pub close_date: Option<DateTime<Utc>>,
    pub duration: Option<Duration>,
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "Full quiz input")]
pub struct FullQuizInput {
    pub name: Option<String>,
    pub owner: UserId,
    pub open_date: DateTime<Utc>,
    pub close_date: Option<DateTime<Utc>>,
    pub duration: Option<Duration>,
    pub questions: Vec<FullQuizQuestionInput>,
}
