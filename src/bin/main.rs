use lib_tvt::app::run;

fn main() {
    if let Err(e) = run() {
        tracing::error!("{e}");
    }
}
