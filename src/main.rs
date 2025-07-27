use axum::{response::IntoResponse, Router};
use axum::routing::{delete, get, post, put};
use tokio::net::TcpListener;


pub mod database;
use database::*;


pub mod show;
use show::*;

pub mod playlist;

use crate::playlist::api::{add_a_playlist, fetch_all_playlists, fetch_single_playlist};

#[tokio::main]
async fn main() {
    let db = database_connection().await.expect("Failed to load database");

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
                                .with_state(db);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Server running on localhost:8080");

    axum::serve(listener, routes.into_make_service()).await.unwrap();

}

pub async fn handle_home_request() -> impl IntoResponse {
    "Server up and running!\n"
}

