#[derive(Copy, Clone)]
pub struct Duration(pub chrono::Duration);

#[juniper::graphql_scalar(name = "Duration", description = "Duration")]
impl<S> GraphQLScalar for Duration
where
    S: juniper::ScalarValue,
{
    fn resolve(&self) -> juniper::Value {
        juniper::Value::scalar(format!("{}", self.0.num_seconds()))
    }

    fn from_input_value(v: &InputValue) -> Option<Duration> {
        v.as_string_value()
            .and_then(|raw| i64::from_str_radix(raw, 10).ok())
            .map(|ms| Duration(chrono::Duration::seconds(ms)))
    }

    fn from_str<'a>(value: juniper::parser::ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}
