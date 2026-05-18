pub mod index;

use actix_web::{
    App, HttpServer,
    middleware::{self, Compress, NormalizePath, TrailingSlash},
    web::{self},
};
use dotenv::dotenv;
use tracing::info;
use tracing_subscriber::{EnvFilter, util::SubscriberInitExt};



#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let dev_mode = std::env::var("DEV").is_ok();

    tracing_subscriber::fmt::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .finish()
        // .with(sentry_layer)
        .init();

    let host = "0.0.0.0:8080";

    if dev_mode {
        tracing::error!("RUNNING IN DEV MODE");
    }

    info!("starting HTTP server at {host}");


    // srv is server controller type, `dev::Server`
    HttpServer::new(move || {
        App::new()
            // enable logger
            .service(web::resource("/")
                .head(index::index_head)
                .get(index::index_get)
                .post(index::index_post)
            )
            .wrap(middleware::Logger::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
    })
        .bind(host)?
        .workers(1)
        .disable_signals()
        .run()
        .await?;
    Ok(())
}