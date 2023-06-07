use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use std::{convert::Infallible, sync::Arc};
use uuid::Uuid;
use warp::Filter;

use juniper::RootNode;
use tokio_postgres::{Client, NoTls};

struct QueryRoot;

struct MutationRoot;

struct SubRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn customer(ctx: &Context, id: String) -> juniper::FieldResult<Customer> {
        let uuid = Uuid::parse_str(&id)?;
        let row = ctx
            .0
            .query_one(
                "SELECT name, age, email, address FROM customers WHERE id = $1",
                &[&uuid.to_string()],
            )
            .await?;
        let customer = Customer {
            id,
            name: row.try_get(0)?,
            age: row.try_get(1)?,
            email: row.try_get(2)?,
            address: row.try_get(3)?,
        };
        Ok(customer)
    }
}

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    async fn register_customer(
        ctx: &Context,
        name: String,
        age: i32,
        email: String,
        address: String,
    ) -> juniper::FieldResult<Customer> {
        let id = uuid::Uuid::new_v4();
        let email = email.to_lowercase();
        ctx.0
            .execute(
                "INSERT INTO customers (id, name, age, email, address) VALUES ($1, $2, $3, $4, $5)",
                &[&id.to_string(), &name, &age, &email, &address],
            )
            .await?;
        Ok(Customer {
            id: id.to_string(),
            name,
            age,
            email,
            address,
        })
    }
}

#[juniper::graphql_object(Context = Context)]
impl SubRoot {
    fn root() -> Option<&'static str> {
        None
    }
}

type Schema = RootNode<'static, QueryRoot, MutationRoot, SubRoot>;

struct Context(Client);

impl juniper::Context for Context {}

#[tokio::main]
async fn main() {
    let host = "localhost";
    let user = "postgres";
    let password = "password";
    println!("Running...");
    let (client, connection) = tokio_postgres::connect(
        &format!("host={host} user={user} password={password}"),
        NoTls,
    )
    .await
    .unwrap();

    // connection object
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let schema = Arc::new(RootNode::new(QueryRoot, MutationRoot, SubRoot));

    let schema = warp::any().map(move || Arc::clone(&schema));

    let ctx = Arc::new(Context(client));
    let ctx = warp::any().map(move || Arc::clone(&ctx));

    let graphql_route = warp::post()
        .and(warp::path("graphiql"))
        .and(schema.clone())
        .and(ctx.clone())
        .and(warp::body::json())
        .and_then(graphql);

    let graphiql_route = warp::get()
        .and(warp::path("graphiql"))
        .map(|| warp::reply::html(graphiql_source("graphiql", None)));

    let routes = graphql_route.or(graphiql_route);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

async fn graphql(
    schema: Arc<Schema>,
    ctx: Arc<Context>,
    req: GraphQLRequest,
) -> Result<impl warp::Reply, Infallible> {
    let res = req.execute(&schema, &ctx).await;
    let json = serde_json::to_string(&res).expect("Invalid JSON response");
    Ok(json)
}

#[derive(juniper::GraphQLObject)]
struct Customer {
    id: String,
    name: String,
    age: i32,
    email: String,
    address: String,
}
