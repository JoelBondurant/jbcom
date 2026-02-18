mod hits;

use crate::hits::HitCounter;
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
	hit_counter: Arc<HitCounter>,
	photos: Vec<String>,
}

#[tokio::main]
async fn main() {
	println!("jbcom started.");
	let hit_counter = Arc::new(HitCounter::new("/var/lib/jbcom/hits.u64"));
	let mut photos = std::fs::read_dir("./photos")
		.expect("photo loading failure")
		.map(|x| x.unwrap().file_name().into_string().unwrap())
		.collect::<Vec<String>>();
	photos.sort();
	println!("Photos loaded: {}", photos.len());
	let app_state = Arc::new(AppState {
		hit_counter,
		photos,
	});
	let photo_service = ServeDir::new("./photos");
	let cache_layer = SetResponseHeaderLayer::overriding(
		HeaderName::from_static("cache-control"),
		HeaderValue::from_static("public, max-age=6000"),
	);
	let cached_photo_service = get_service(photo_service).layer(cache_layer);
	let app = Router::new()
		.route("/", get(index_handler))
		.route("/photos", get(photos_handler))
		.route("/resume", get(resume_handler))
		.nest_service("/photo", cached_photo_service)
		.nest_service("/static", ServeDir::new("static"))
		.with_state(app_state);
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
	hit_count: &'a str,
	ip_address: &'a str,
}

async fn index_handler(
	ConnectInfo(addr): ConnectInfo<SocketAddr>,
	State(app_state): State<Arc<AppState>>,
) -> Html<String> {
	let hit_count = app_state.hit_counter.increment();
	Html(
		IndexTemplate {
			hit_count: &hit_count.to_string(),
			ip_address: &addr.ip().to_string(),
		}
		.render()
		.unwrap(),
	)
}

#[derive(Template)]
#[template(path = "photos.html")]
struct PhotosTemplate<'a> {
	photos: &'a Vec<String>,
}

async fn photos_handler(State(app_state): State<Arc<AppState>>) -> Html<String> {
	Html(
		PhotosTemplate {
			photos: &app_state.photos,
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
