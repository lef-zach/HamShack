use axum::{
    extract::State,
    response::{sse::Event, Sse},
    routing::get,
    Router,
    response::Html,
};
use std::{convert::Infallible, net::SocketAddr, time::Duration};
use tokio_stream::{StreamExt as _, wrappers::IntervalStream};
use tracing_subscriber;
use serde_json::json;
use tower_http::services::ServeDir;

mod config;
mod sdr;
mod websocket;

#[derive(Clone)]
struct AppState {
    sdr_manager: std::sync::Arc<tokio::sync::Mutex<sdr::SDRManager>>,
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("HamShack Rust Backend Starting...");
    
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let mut config = config::Config::load().expect("Failed to load configuration");
    
    // Override port if specified
    if args.len() > 1 && args[1].starts_with("--port") {
        if let Some(port_str) = args[1].split('=').nth(1) {
            if let Ok(port) = port_str.parse::<u16>() {
                config.port = port;
            }
        } else if args.len() > 2 {
            if let Ok(port) = args[2].parse::<u16>() {
                config.port = port;
            }
        }
    }
    
    // Force localhost for testing
    config.host = "127.0.0.1".to_string();
    
    // Initialize SDR manager
    let mut sdr_manager = sdr::SDRManager::new();
    sdr_manager.start().expect("Failed to start SDR");
    
    // Initialize application state
    let state = AppState {
        sdr_manager: std::sync::Arc::new(tokio::sync::Mutex::new(sdr_manager)),
    };
    
    // Build router with SSE endpoint and static file serving
    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/sse", get(sse_handler))
        .route("/api/sdr/status", get(sdr_status))
        .route("/api/sdr/start", get(sdr_start))
        .route("/api/sdr/stop", get(sdr_stop))
        .route("/api/sdr/frequency/{freq}", get(set_frequency))
        .nest_service("/assets", ServeDir::new("static/assets"))
        .route("/", get(index_handler))
        .fallback_service(ServeDir::new("static"))
        .with_state(state);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    
    println!("Server listening on http://{}", addr);
    println!("SSE endpoint: http://{}/api/sse", addr);
    println!("SDR endpoints available");
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "OK"
}

async fn index_handler() -> Html<String> {
    match std::fs::read_to_string("static/index.html") {
        Ok(content) => Html(content),
        Err(_) => Html("<h1>HamShack Dashboard</h1><p>Frontend files not found</p>".to_string())
    }
}



async fn sse_handler(
    State(state): State<AppState>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let sdr_manager = state.sdr_manager.clone();
    
    let stream = IntervalStream::new(tokio::time::interval(Duration::from_millis(100)))
        .map(move |_| {
            let sdr_manager = sdr_manager.clone();
            
            async move {
                let sdr_lock = sdr_manager.lock().await;
                let status = sdr_lock.get_status();
                
                let data = json!({
                    "type": "sdr_status",
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "data": {
                        "running": status.is_running,
                        "frequency": status.frequency,
                        "sample_rate": status.sample_rate,
                        "gain": status.gain
                    }
                });
                
                Ok(Event::default().data(data.to_string()))
            }
        })
        .then(|fut| fut);

    Sse::new(stream)
}

async fn sdr_status(
    State(state): State<AppState>,
) -> axum::Json<serde_json::Value> {
    let sdr_manager = state.sdr_manager.lock().await;
    let status = sdr_manager.get_status();
    
    axum::Json(json!({
        "running": status.is_running,
        "frequency": status.frequency,
        "sample_rate": status.sample_rate,
        "gain": status.gain
    }))
}

async fn sdr_start(
    State(state): State<AppState>,
) -> axum::Json<serde_json::Value> {
    let mut sdr_manager = state.sdr_manager.lock().await;
    let result = sdr_manager.start();
    
    axum::Json(json!({
        "success": result.is_ok(),
        "message": if result.is_ok() { "SDR started" } else { "Failed to start SDR" }
    }))
}

async fn sdr_stop(
    State(state): State<AppState>,
) -> axum::Json<serde_json::Value> {
    let mut sdr_manager = state.sdr_manager.lock().await;
    let result = sdr_manager.stop();
    
    axum::Json(json!({
        "success": result.is_ok(),
        "message": if result.is_ok() { "SDR stopped" } else { "Failed to stop SDR" }
    }))
}

async fn set_frequency(
    axum::extract::Path(freq): axum::extract::Path<u64>,
    State(state): State<AppState>,
) -> axum::Json<serde_json::Value> {
    let mut sdr_manager = state.sdr_manager.lock().await;
    let result = sdr_manager.set_frequency(freq);
    
    axum::Json(json!({
        "success": result.is_ok(),
        "message": if result.is_ok() { "Frequency set" } else { "Failed to set frequency" },
        "frequency": freq
    }))
}