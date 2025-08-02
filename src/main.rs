use axum::{response::IntoResponse, Router};
use axum::routing::{delete, get, post, put};
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};

pub mod database;
use database::*;

pub mod now_playing;

pub mod show;
use show::*;

pub mod playlist;
use playlist::*;

pub mod schedule;
use schedule::*;

pub mod error;

#[tokio::main]
async fn main() {
    let db = database_connection().await.expect("Failed to load database");

    let cors = CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any);

    let routes = Router::new()
                                .route("/", get(handle_home_request))
                                .route("/create", post(create_a_show))
                                .route("/shows/fetch", get(fetch_all_shows))
                                .route("/shows/fetch/{id}", get(fetch_single_show))
                                .route("/shows/update/{id}", put(update_a_show))
                                .route("/shows/delete/{id}", delete(delete_a_show))
                                .route("/playlists/add", post(add_a_playlist))
                                .route("/playlists/fetch", get(fetch_all_playlists))
                                .route("/playlists/fetch/{id}", get(fetch_single_playlist))
                                .route("/schedule/create", post(create_schedule_entry))
                                // .route("/ws/now-playing", get(now_playing_ws))
                                .layer(cors)
                                .with_state(db);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Server running on localhost:8080");

    axum::serve(listener, routes.into_make_service()).await.unwrap();

}

pub async fn handle_home_request() -> impl IntoResponse {
    "Server up and running!\n"
}

