use axum::{Router, routing::get};

async fn server() -> std::io::Result<()> {
    let app = Router::new().route("/health", get(|| async { "ok" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    match server().await {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}
