use crate::backend::serve;

mod config;
mod generator;
mod backend;

#[tokio::main]
async fn main() {
    let _ = serve().await;
}
