use clap::Parser;
use fb::config::Config;
use fb::resources::Resources;
use fb::routes;
use tokio::net::TcpListener;
use tokio::signal::unix::{signal, SignalKind};
use tracing_appender::rolling;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let config = Config::parse();

    let file_appender = rolling::RollingFileAppender::builder()
        .rotation(config.log_rotation.clone().into())
        .filename_prefix(&config.log_prefix)
        .filename_suffix(&config.log_suffix)
        .max_log_files(config.log_max_files)
        .build("logs")
        .unwrap();
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(EnvFilter::new(config.log_level.to_string()))
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_writer(non_blocking),
        )
        .init();

    let res = Resources::new(&config).await.expect("app init");
    let router = routes::build_router(res);

    let listener = TcpListener::bind((config.addr.as_str(), config.port))
        .await
        .unwrap();
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = tokio::signal::ctrl_c();
    let mut sigterm = signal(SignalKind::terminate()).expect("sigterm handler");

    tokio::select! {
        _ = ctrl_c => {}
        _ = sigterm.recv() => {}
    }
}
