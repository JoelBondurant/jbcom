use axum::{
    Router,
    extract::State,
    response::Html,
    routing::get,
    routing::get_service,
};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    images: Vec<String>,
}

#[tokio::main]
async fn main() {
    println!("jbcom started.");
    let mut images = std::fs::read_dir("./img")
        .expect("img failure")
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<String>>();
    images.sort();
    println!("Images loaded: {}", images.len());
    let app_state = Arc::new(AppState { images: images });
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/images", get(img_handler))
        .nest_service("/img", get_service(ServeDir::new("./img")))
        .with_state(app_state);
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

async fn index_handler() -> Html<&'static str> {
    Html(
        "<html><body style='background-color:black; color:white;'><h1>Rewriting in Rust...</h1></body></html>",
    )
}

async fn img_handler(State(app_state): State<Arc<AppState>>) -> Html<String> {
    let template = "<a href=/img/{}>{}</a>";
    let images = app_state
        .images
        .iter()
        .map(|im| template.replace("{}", im))
        .collect::<Vec<String>>()
        .join("</br>\n");
    Html(
        "<html><body style='background-color:black; color:white;'>{}</body></html>"
            .replace("{}", &images),
    )
}
