use juniper::{EmptyMutation, EmptySubscription};
use rocket::{response::content, State};

#[macro_use]
extern crate rocket;

mod database;
mod graphql_resolvers;
mod models;
mod schema;
mod services;

#[get("/")]
fn graphiql() -> content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: &State<graphql_resolvers::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<graphql_resolvers::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &*context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: &State<graphql_resolvers::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<graphql_resolvers::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &*context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(graphql_resolvers::Context::new())
        .manage(graphql_resolvers::Schema::new(
            graphql_resolvers::Query,
            EmptyMutation::<graphql_resolvers::Context>::new(),
            EmptySubscription::<graphql_resolvers::Context>::new(),
        ))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
}
