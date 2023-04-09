use warp::Filter;
use std::sync::Arc;
use juniper::{http::graphiql::graphiql_source, SchemaType};

#[tokio::main]
async fn main() {
    let schema = Arc::new(SchemaType::new(query_info, mutation_info, sub_info));
}
