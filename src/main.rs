use std::{
    env,
    error::Error,
    fs::OpenOptions,
    io::{ErrorKind, Write},
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::Duration,
};

use config::db::{init_db_pool, run_migration, Pool};
use controller::api::{game::GameApi, level::LevelApi, score::ScoreApi, user::UserApi};
#[cfg(debug_assertions)]
use dotenvy::dotenv;
use log::{error, info};
use tokio::{net::TcpListener, spawn, time::interval};
use utoipa::OpenApi;

pub mod config;
pub mod controller;
pub mod middleware;
pub mod models;
pub mod response;
pub mod routes;
pub mod schema;
pub mod service;

pub const JWK_FILE_PATH: &str = "data/jwk.json";

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Bonk inc Backend",
        description = "My Api description"
    ),
    servers((url = "https://babs.bonk.group/api")),
    nest(
        (path = "/game", api = GameApi),
        (path = "/level", api = LevelApi),
        (path = "/score", api = ScoreApi),
        (path = "/user", api = UserApi)
    ),
    tags(
        (name = "Game", description = "Game management endpoints."),
        (name = "Level", description = "Level management endpoints."),
        (name = "Score", description = "Score management endpoints."),
        (name = "User", description = "User management endpoints.")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().expect(".env file not found");

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let app_host = env::var("APP_HOST").expect("APP_HOST must be set");
    let app_port = env::var("APP_PORT").expect("APP_PORT must be set");
    let app_url = format!("{}:{}", app_host, app_port);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = init_db_pool(&db_url);
    run_migration(&mut db_pool.get().unwrap());

    let state = SharedState::new(RwLock::new(AppState { db: db_pool }));

    let addr: SocketAddr = app_url.parse().expect("Cannot parse app url to socket");
    let listener = TcpListener::bind(addr).await.unwrap();
    let app = routes::create_app(state).await;

    spawn(refresh_jwk());
    axum::serve(listener, app).await.unwrap();
}

/// Fetches the JWKS data from the authorization server and saves the 
/// content to a json file.
/// 
/// # Errors
/// 
/// This function fails if:
/// - JWKS data could not be fetched
/// - JWKS content could not be saved to file
/// 
async fn fetch_and_save_jwks() -> Result<(), Box<dyn Error>> {
    let jwsk_url = env::var("OAUTH_JWKS_URL").expect("OAUTH_JWKS_URL must be set");
    let tokens = reqwest::get(&jwsk_url).await?.text().await;

    if tokens.is_err() {
        error!("Could not fetch token from {}", jwsk_url);
        return Err(Box::new(std::io::Error::new(
            ErrorKind::Other,
            "Could not fetch JWKS token",
        )));
    }

    info!("JWKS token fetched, saving to file");
    let file_options = OpenOptions::new()
        .write(true)
        .create(true)
        .open(JWK_FILE_PATH);

    if file_options.is_ok() {
        info!("Write new JWKS token to file");
        let _ = file_options.unwrap().write_all(tokens.unwrap().as_bytes());
    } else {
        error!(
            "Cannot write JWKS token to file, reason {}",
            file_options.err().unwrap()
        )
    }

    Ok(())
}

/// Calls the [`fetch_and_save_jwks`] function every month to make sure the 
/// JWKS info is valid.
async fn refresh_jwk() {
    let mut delay = interval(Duration::from_secs(604_800));

    loop {
        delay.tick().await;

        info!("Refreshing JWKS token");
        let _ = fetch_and_save_jwks().await;
    }
}

type SharedState = Arc<RwLock<AppState>>;

#[derive(Clone)]
pub struct AppState {
    db: Pool,
}
