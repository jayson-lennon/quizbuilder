use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! impl_from_uuid {
    ($ty:ident) => {
        impl From<Uuid> for $ty {
            fn from(u: Uuid) -> Self {
                $ty(u)
            }
        }

        impl From<$ty> for Uuid {
            fn from(id: $ty) -> Self {
                id.0
            }
        }
    };
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct UserId(pub Uuid);
impl_from_uuid!(UserId);

#[juniper::graphql_scalar(name = "UserId", description = "User ID")]
impl<S> GraphQLScalar for UserId
where
    S: juniper::ScalarValue,
{
    fn resolve(&self) -> juniper::Value {
        juniper::Value::scalar(format!("{}", self.0))
    }

    fn from_input_value(v: &InputValue) -> Option<UserId> {
        v.as_string_value()
            .and_then(|s| Uuid::parse_str(s).ok())
            .and_then(|id| Some(UserId(id)))
    }

    fn from_str<'a>(value: juniper::parser::ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct QuizId(pub Uuid);
impl_from_uuid!(QuizId);

#[juniper::graphql_scalar(name = "QuizId", description = "Quiz ID")]
impl<S> GraphQLScalar for QuizId
where
    S: juniper::ScalarValue,
{
    fn resolve(&self) -> juniper::Value {
        juniper::Value::scalar(format!("{}", self.0))
    }

    fn from_input_value(v: &InputValue) -> Option<QuizId> {
        v.as_string_value()
            .and_then(|s| Uuid::parse_str(s).ok())
            .and_then(|id| Some(QuizId(id)))
    }

    fn from_str<'a>(value: juniper::parser::ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct QuestionId(pub Uuid);
impl_from_uuid!(QuestionId);

#[juniper::graphql_scalar(name = "QuestionId", description = "Question ID")]
impl<S> GraphQLScalar for QuestionId
where
    S: juniper::ScalarValue,
{
    fn resolve(&self) -> juniper::Value {
        juniper::Value::scalar(format!("{}", self.0))
    }

    fn from_input_value(v: &InputValue) -> Option<QuestionId> {
        v.as_string_value()
            .and_then(|s| Uuid::parse_str(s).ok())
            .and_then(|id| Some(QuestionId(id)))
    }

    fn from_str<'a>(value: juniper::parser::ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct OptionId(pub Uuid);
impl_from_uuid!(OptionId);

#[juniper::graphql_scalar(name = "OptionID", description = "Option ID")]
impl<S> GraphQLScalar for OptionId
where
    S: juniper::ScalarValue,
{
    fn resolve(&self) -> juniper::Value {
        juniper::Value::scalar(format!("{}", self.0))
    }

    fn from_input_value(v: &InputValue) -> Option<OptionId> {
        v.as_string_value()
            .and_then(|s| Uuid::parse_str(s).ok())
            .and_then(|id| Some(OptionId(id)))
    }

    fn from_str<'a>(value: juniper::parser::ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SubmissionId(pub Uuid);
impl_from_uuid!(SubmissionId);

#[juniper::graphql_scalar(name = "SubmissionId", description = "Submission ID")]
impl<S> GraphQLScalar for SubmissionId
where
    S: juniper::ScalarValue,
{
    fn resolve(&self) -> juniper::Value {
        juniper::Value::scalar(format!("{}", self.0))
    }

    fn from_input_value(v: &InputValue) -> Option<SubmissionId> {
        v.as_string_value()
            .and_then(|s| Uuid::parse_str(s).ok())
            .and_then(|id| Some(SubmissionId(id)))
    }

    fn from_str<'a>(value: juniper::parser::ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}
