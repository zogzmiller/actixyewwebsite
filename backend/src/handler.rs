use crate::{model::{User, UserLogin}};


use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

const DB_NAME: &str = "myApp";
const COLL_NAME: &str = "users";

/// Adds a new user to the "users" collection in the database.
#[post("/add_user")]
async fn add_user(client: web::Data<Client>, form: web::Form<User>) -> impl Responder {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().json("Successfully added user"),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

/// Gets the user with the supplied username.
#[get("/get_user/{username}")]
async fn get_user(client: web::Data<Client>, username: web::Path<String>) -> impl Responder {
    let username = username.into_inner();
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    match collection
        .find_one(doc! { "username": &username }, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().json(format!("No user found with username {username}"))
        }
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[post("/verification")]
async fn verification(client: web::Data<Client>, form: web::Form<UserLogin>) -> impl Responder {
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    let logininfo = form.into_inner();
    match collection
    .find_one(doc! { "username": &logininfo.username }, None)
    .await
{
    Ok(Some(user)) if user.password.ne(&logininfo.password) => {
        HttpResponse::NotFound().json(format!("Invalid password for user: {}", user.username))
    }
    Ok(Some(user)) => HttpResponse::Ok().json(user),
    Ok(None) => {
        HttpResponse::NotFound().json(format!("No user found with username {}", logininfo.username))
    }

    Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
}
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_user)
        .service(add_user)
        .service(verification);

    conf.service(scope);
}
