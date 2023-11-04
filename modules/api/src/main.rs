use lambda_web::actix_web::{self, web, App,  Error, HttpServer, HttpResponse};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};

use std::sync::Arc;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
    
mod graphql_schema;
mod dynamo_db;

use crate::graphql_schema::{create_schema, Schema};

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    // Initialize the graphql schema
    let schema = std::sync::Arc::new(create_schema());
    
    
    let factory = move || App::new()

        // clone the schema
        .app_data(lambda_web::actix_web::web::Data::new(schema.clone()))

        // service for executing query and mutation requests
        .service(web::resource("/graphql").route(web::post().to
        (graphql)))

        // service for providing an interface to send the requests
        .service(web::resource("/graphiql").route(web::get().to
        (graphiql))) 
        
        
        .service(web::redirect("/", "/graphiql"));



    if is_running_on_lambda() {
        // Run on AWS Lambda
        run_actix_on_lambda(factory).await?;
    } else {
        // Run local server
        println!("Running on 0.0.0.0:8080");
        HttpServer::new(factory)
            .bind("0.0.0.0:8080")?
            .run()
            .await?;
    }
    Ok(())
}


async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse,Error>{
    let res = data.execute(&st, &()).await;
    let result = Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?);

    result
        .map_err(Error::from)
        .and_then(|user| {
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(user))
        })
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}