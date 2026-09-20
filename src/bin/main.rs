use lib_tvt::app::run;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        tracing::error!("{e}");
    }
}
