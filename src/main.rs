#![allow(non_snake_case)]

pub mod app;

#[tokio::main]
async fn main() {
    app::app().await;
}