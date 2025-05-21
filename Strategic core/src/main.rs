use ia_strategic_core::config;
use ia_strategic_core::engine;
use ia_strategic_core::logger;

#[tokio::main]
async fn main() {
    logger::init();
    let settings = config::load();
    engine::run(settings).await;
}
