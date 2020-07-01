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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct UserId(pub Uuid);
scalar_id!(UserId, "User ID");

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct QuizId(pub Uuid);
scalar_id!(QuizId, "Quiz ID");

impl From<Uuid> for QuizId {
    fn from(u: Uuid) -> Self {
        QuizId(u)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct QuestionId(pub Uuid);
scalar_id!(QuestionId, "Question ID");

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct OptionId(pub Uuid);
scalar_id!(OptionId, "Option ID");

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SubmissionId(pub Uuid);
scalar_id!(SubmissionId, "Submittion ID");
