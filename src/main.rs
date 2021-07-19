#[macro_use]
extern crate rocket;
extern crate dotenv;
#[macro_use]
extern crate diesel;

use std::vec::Vec;

use dotenv::dotenv;
use rocket::http::Method;
use rocket::serde::json::Json;
use rocket::tokio;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_sync_db_pools::database;

pub mod crud;
pub mod github;
pub mod models;
pub mod schema;

#[database("db")]
struct DbConn(diesel::SqliteConnection);

#[get("/config")]
async fn get_config(conn: DbConn) -> Json<models::Config> {
    Json(conn.run(|c| crud::get_config(c)).await.expect("Get config"))
}
#[patch("/config", data = "<config>")]
async fn update_config(conn: DbConn, config: Json<models::Config>) -> Json<()> {
    Json(
        conn.run(|c| crud::update_config(c, config.into_inner()))
            .await
            .expect("Update config"),
    )
}
#[get("/targets")]
async fn get_targets(conn: DbConn) -> Json<Vec<models::Target>> {
    Json(
        conn.run(|c| crud::get_targets(c))
            .await
            .expect("Get targets"),
    )
}
#[post("/targets", data = "<target>")]
async fn create_target(conn: DbConn, target: Json<models::NewTarget>) -> Json<()> {
    Json(
        conn.run(|c| crud::create_target(c, target.into_inner()))
            .await
            .expect("Create target"),
    )
}
#[get("/mappings")]
async fn get_mappings(conn: DbConn) -> Json<Vec<models::TargetVersionMapping>> {
    Json(
        conn.run(|c| crud::get_mappings(c))
            .await
            .expect("Get Mappings"),
    )
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    tokio::spawn(async {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));
        loop {
            if let Err(s) = github::discover_new_releases().await {
                eprintln!("Discover new releases_error: {}", s);
                std::process::exit(1);
            }
            interval.tick().await;
        }
    });

    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions::default()
        .allowed_origins(allowed_origins)
        .allowed_methods(
            vec![Method::Get, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allowed_headers(AllowedHeaders::some(&["Authorization", "Accept"]))
        .allow_credentials(true)
        .to_cors()
        .expect("Cors options should compile");

    rocket::build()
        .mount(
            "/",
            routes![
                create_target,
                get_targets,
                get_config,
                update_config,
                get_mappings,
            ],
        )
        .attach(cors)
        .attach(DbConn::fairing())
}
