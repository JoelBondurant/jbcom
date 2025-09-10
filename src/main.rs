use axum::{
    routing::get,
    routing::get_service,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use std::path::PathBuf;
use axum::response::Html;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(handler))
        .nest_service("/img", get_service(ServeDir::new("./img")));
    /*
    setcap 'cap_net_bind_service=+ep' jbcom
    */
    let addr = SocketAddr::from(([0, 0, 0, 0], 443)); 
    let pem_path = "/etc/letsencrypt/live/joelbondurant.com";
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(format!("{pem_path}/fullchain.pem")),
        PathBuf::from(format!("{pem_path}/privkey.pem")),
    )
    .await
    .expect("Failed to load TLS certificates");

    println!("jbcom listening on {}", addr);
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("Rewriting in Rust....")
}

