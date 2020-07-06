use crate::types::error::LibquizError;
use crate::types::id::{OptionId, QuestionId};
use juniper::{GraphQLInputObject, GraphQLObject};
use std::convert::{TryFrom, TryInto};

#[derive(Clone, Copy, Debug)]
pub enum QuizOptionType {
    Unknown = 0,
    MultiChoice = 1,
    SingleChoice = 2,
    ShortAnswer = 3,
}

impl From<QuizOptionType> for i32 {
    fn from(o: QuizOptionType) -> Self {
        match o {
            QuizOptionType::Unknown => 0,
            QuizOptionType::MultiChoice => 1,
            QuizOptionType::SingleChoice => 2,
            QuizOptionType::ShortAnswer => 3,
        }
    }
}

// TODO: Use &str instead of String.
impl From<QuizOptionType> for String {
    fn from(o: QuizOptionType) -> Self {
        match o {
            QuizOptionType::Unknown => "Unknown".to_string(),
            QuizOptionType::MultiChoice => "MultiChoice".to_string(),
            QuizOptionType::SingleChoice => "SingleChoice".to_string(),
            QuizOptionType::ShortAnswer => "ShortAnswer".to_string(),
        }
    }
}

impl TryFrom<i32> for QuizOptionType {
    type Error = LibquizError;
    fn try_from(i: i32) -> Result<Self, Self::Error> {
        match i {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::MultiChoice),
            2 => Ok(Self::SingleChoice),
            3 => Ok(Self::ShortAnswer),
            e => Err(LibquizError::QuizOption(format!(
                "QuizOption type not found: {}",
                e
            ))),
        }
    }
}

impl TryFrom<&str> for QuizOptionType {
    type Error = LibquizError;
    fn try_from(i: &str) -> Result<Self, Self::Error> {
        match i {
            "Unknown" => Ok(Self::Unknown),
            "MultiChoice" => Ok(Self::MultiChoice),
            "SingleChoice" => Ok(Self::SingleChoice),
            "ShortAnswer" => Ok(Self::ShortAnswer),
            e => Err(LibquizError::QuizOption(format!(
                "QuizOption type not found: {}",
                e
            ))),
        }
    }
}

#[juniper::graphql_scalar(name = "QuizOptionType", description = "QuizOptionType")]
impl<S> GraphQLScalar for QuizOptionType
where
    S: juniper::ScalarValue,
{
    fn resolve(&self) -> juniper::Value {
        juniper::Value::scalar(String::from(*self))
    }

    fn from_input_value(v: &InputValue) -> Option<QuizOptionType> {
        v.as_string_value().and_then(|s| s.try_into().ok())
    }

    fn from_str<'a>(value: juniper::parser::ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}

#[derive(GraphQLObject, Debug)]
#[graphql(description = "An option")]
pub struct QuizOption {
    pub quiz_option_id: OptionId,
    pub quiz_question_id: QuestionId,
    pub option_data: String,
    pub is_correct: bool,
    pub position: Option<i32>,
    pub option_type: QuizOptionType,
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "New Option")]
pub struct QuizOptionInput {
    pub quiz_question_id: QuestionId,
    pub option_data: String,
    pub is_correct: bool,
    pub position: Option<i32>,
    pub option_type: QuizOptionType,
}
