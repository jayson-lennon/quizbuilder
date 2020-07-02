#[derive(Copy, Clone)]
pub struct Duration(pub chrono::Duration);

juniper::graphql_scalar!(Duration where Scalar = <S> {
    description: "Duration"

    resolve(&self) -> juniper::Value {
        juniper::Value::scalar(format!("{}", self.0.num_seconds()))
    }

    from_input_value(v: &InputValue) -> Option<Duration> {
        v.as_scalar_value::<String>()
        .and_then(|raw| i64::from_str_radix(raw, 10).ok())
        .map(|ms| Duration(chrono::Duration::seconds(ms)))
    }

    from_str<'a>(value: ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
});
