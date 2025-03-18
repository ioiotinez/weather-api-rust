/* I want to create an API using actix  */

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/weather")]
async fn invoke_weather_api() -> impl Responder {
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let city = "Montevideo";
    let path = format!("https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/{}?unitGroup=metric&include=days&key={}&contentType=json", city, api_key);
    let response = reqwest::get(path).await.unwrap().text().await.unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load the .env file
    HttpServer::new(|| App::new().service(index).service(invoke_weather_api))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
