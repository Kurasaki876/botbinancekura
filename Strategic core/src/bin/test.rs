use ia_strategic_core::test_runner;

#[tokio::main]
async fn main() {
    test_runner::run_all_tests().await;
}
