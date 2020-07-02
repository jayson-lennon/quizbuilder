use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! scalar_id {
    ($ty:ident, $description:expr) => {
        juniper::graphql_scalar!($ty where Scalar = <S> {
            description: $description

            resolve(&self) -> juniper::Value {
                juniper::Value::scalar(format!("{}", self.0))
            }

            from_input_value(v: &InputValue) -> Option<$ty> {
                v.as_scalar_value::<String>()
                .and_then(|s| Uuid::parse_str(s).ok())
                .and_then(|id| Some($ty(id)))
            }

            from_str<'a>(value: ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
                <String as juniper::ParseScalarValue<S>>::from_str(value)
            }
        });

    };
}

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
scalar_id!(UserId, "User ID");
impl_from_uuid!(UserId);

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct QuizId(pub Uuid);
scalar_id!(QuizId, "Quiz ID");
impl_from_uuid!(QuizId);

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct QuestionId(pub Uuid);
scalar_id!(QuestionId, "Question ID");
impl_from_uuid!(QuestionId);

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct OptionId(pub Uuid);
scalar_id!(OptionId, "Option ID");
impl_from_uuid!(OptionId);

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SubmissionId(pub Uuid);
scalar_id!(SubmissionId, "Submittion ID");
impl_from_uuid!(SubmissionId);
