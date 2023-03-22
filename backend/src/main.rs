mod model;
mod handler;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use common::User;
use mongodb::{bson::doc, Client, IndexModel, options::IndexOptions};

const DB_NAME: &str = "myApp";
const COLL_NAME: &str = "users";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uri = std::env::var("mongodb://localhost:27017").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    create_username_index(&client).await;

    HttpServer::new(move || {
        let cors = Cors::default()
	.allowed_origin("https://www.shebolaheh.com")
	.allowed_methods(vec!["GET","POST"])
	.allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::ALLOW])
	.allowed_header(http::header::CONTENT_TYPE)
	.supports_credentials();
        App::new()
            .app_data(web::Data::new(client.clone()))
            .wrap(cors)
            .wrap(Logger::default())
	    .configure(handler::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}
