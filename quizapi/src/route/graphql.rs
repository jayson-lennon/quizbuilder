use juniper_rocket::{GraphQLRequest, GraphQLResponse};
use libquiz::schema;
use rocket::{response::content::Html, State};

#[rocket::get("/graphiql")]
pub fn graphiql() -> Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
pub fn get(
    context: State<schema::Context>,
    request: GraphQLRequest,
    schema: State<schema::Schema>,
) -> GraphQLResponse {
    let schema = schema.inner();
    request.execute_sync(schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
pub fn post(
    context: State<schema::Context>,
    request: GraphQLRequest,
    schema: State<schema::Schema>,
) -> GraphQLResponse {
    let schema = schema.inner();
    request.execute_sync(schema, &context)
}
