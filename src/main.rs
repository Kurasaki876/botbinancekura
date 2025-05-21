mod config;
mod logger;
mod engine;

#[tokio::main]
async fn main() {
    logger::init();
    let settings = config::load();
    engine::run(settings).await;
}
