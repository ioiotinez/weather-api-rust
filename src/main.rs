/* I want to create an API using actix  */

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref CACHE: Mutex<HashMap<String, (String, u64)>> = Mutex::new(HashMap::new());
}

const TTL: u64 = 60; // Tiempo de vida en segundos


fn get_from_cache(key: &str) -> Option<String> {
    let cache = CACHE.lock().unwrap();
    if let Some((value, timestamp)) = cache.get(key) {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if current_time - timestamp > TTL {
            None
        } else {
            Some(value.clone())
        }
    } else {
        None
    }
}

fn insert_into_cache(key: String, value: String) {
    let mut cache = CACHE.lock().unwrap();
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    cache.insert(key, (value, current_time));
}


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/weather")]
async fn invoke_weather_api() -> impl Responder {
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let city = "Montevideo";
    let path = format!("https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/{}?unitGroup=metric&include=days&key={}&contentType=json", city, api_key);

    if let Some(response) = get_from_cache(&path) {
        return HttpResponse::Ok()
            .content_type("application/json")
            .body(response);
    }

    let response = reqwest::get(path.clone()).await.unwrap().text().await.unwrap();

    insert_into_cache(path, response.clone());

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
