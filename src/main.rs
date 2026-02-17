use askama::Template;
use axum::{
	extract::{ConnectInfo, State},
	response::Html,
	routing::get,
	routing::get_service,
	Router,
};
use axum_server::tls_rustls::RustlsConfig;
use http::header::{HeaderName, HeaderValue};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;

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
	let app_state = Arc::new(AppState { images });
	let img_service = ServeDir::new("./img");
	let cache_layer = SetResponseHeaderLayer::overriding(
		HeaderName::from_static("cache-control"),
		HeaderValue::from_static("public, max-age=6000"),
	);
	let cached_img_service = get_service(img_service).layer(cache_layer);
	let app = Router::new()
		.route("/", get(index_handler))
		.route("/images", get(img_handler))
		.route("/resume", get(resume_handler))
		.nest_service("/img", cached_img_service)
		.with_state(app_state);
	/*
	setcap 'cap_net_bind_service=+ep' jbcom
	*/
	let addr = SocketAddr::from(([0, 0, 0, 0], 443));
	let config =
		RustlsConfig::from_pem_file(PathBuf::from("fullchain.pem"), PathBuf::from("privkey.pem"))
			.await
			.expect("Failed to load TLS certificates");

	println!("jbcom listening on {}", addr);
	axum_server::bind_rustls(addr, config)
		.serve(app.into_make_service_with_connect_info::<SocketAddr>())
		.await
		.unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
	ip_address: &'a str,
}

async fn index_handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Html<String> {
	Html(
		IndexTemplate {
			ip_address: &addr.ip().to_string(),
		}
		.render()
		.unwrap(),
	)
}

#[derive(Template)]
#[template(path = "images.html")]
struct ImagesTemplate<'a> {
	images: &'a Vec<String>,
}

async fn img_handler(State(app_state): State<Arc<AppState>>) -> Html<String> {
	Html(
		ImagesTemplate {
			images: &app_state.images,
		}
		.render()
		.unwrap(),
	)
}

#[derive(Template)]
#[template(path = "resume.html")]
struct ResumeTemplate;

async fn resume_handler() -> Html<String> {
	let rt = ResumeTemplate;
	Html(rt.render().unwrap())
}
