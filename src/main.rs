use axum::extract::Request;
use axum::{Router, routing::get};

async fn handle_health(req: Request) -> &'static str {
    let method = req.method();
    let uri = req.uri();
    let headers = req.headers();

    let msg = format!("method: {method}\nuri: {uri}\nheaders: {headers:#?}");
    println!("{msg}");

    "OK"
}

async fn server() -> std::io::Result<()> {
    let app = Router::new().route("/health", get(handle_health));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
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
