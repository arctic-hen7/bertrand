use anyhow::{bail, Context};
use axum::{
    extract::{ws::Message, State},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use clap::Parser;
use std::path::PathBuf;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

static CONNECTOR_SCRIPT: &str = include_str!("connector.js");

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Opts::parse();

    // We could have either a file or directory
    let file_path = if args.path.is_file() {
        args.path.clone()
    } else if args.path.is_dir() {
        let index_html = args.path.join("index.html");
        if !index_html.is_file() {
            bail!("directory did not contain index.html");
        }
        index_html
    } else {
        bail!("no such file or directory");
    };

    // Read the HTML file and interpolate our WebSocket script
    let html_contents = tokio::fs::read_to_string(file_path)
        .await
        .with_context(|| "failed to read html file")?;
    let connector_script = format!("<script>\n{CONNECTOR_SCRIPT}\n</script>");
    let connector_script = connector_script.replace(
        "const STATE_DURATION = 1000",
        &format!("const STATE_DURATION = {}", args.duration * 1000.0),
    );
    let html_contents = html_contents.replace("</body>", &format!("{connector_script}\n</body>"));

    let app = Router::new()
        .route("/", get(serve_html))
        .route("/api/send", post(new_message))
        .route("/api/ws", get(ws_handler))
        .with_state(AppState {
            tx: broadcast::channel(100).0,
            html: html_contents,
        });
    let app = if args.path.is_dir() {
        app.fallback_service(ServeDir::new(args.path))
    } else {
        app
    };

    let listener = tokio::net::TcpListener::bind((args.host.clone(), args.port))
        .await
        .with_context(|| "failed to bind to address")?;

    println!("Listening on http://{}:{}", args.host, args.port);

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

/// Bertrand is a dead-simple demo system for statically hosting an HTML page that can change its
/// state based on states sent to the server over HTTP, to easily demonstrate prototypes of backend
/// applications.
#[derive(Parser)]
struct Opts {
    /// The path to the HTML file or directory (containing index.html) to serve
    path: PathBuf,
    /// The port to serve on
    #[arg(short, long, default_value = "8080")]
    port: u16,
    /// The host to serve on
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
    /// The minimum duration (seconds) for which each state will be shown if many states are sent
    /// consecutively
    #[arg(short, long, default_value = "1")]
    duration: f32,
}

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<String>,
    html: String,
}

/// Simple static file server for the HTML file.
async fn serve_html(State(state): State<AppState>) -> impl IntoResponse {
    Html(state.html)
}

/// Handler for the `POST /send` route that just broadcasts the message to all clients.
async fn new_message(State(state): State<AppState>, payload: String) {
    let _ = state.tx.send(payload.clone());
}

/// Handler for the `GET /ws` route that instantly upgrades to a WebSocket connection.
async fn ws_handler(
    ws: axum::extract::ws::WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Actual WebSocket connection handler that broadcasts messages received over HTTP to all clients.
async fn handle_socket(mut socket: axum::extract::ws::WebSocket, state: AppState) {
    // Subscribe to the broadcast channel, we won't worry about previous messages
    let mut rx = state.tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        if socket.send(Message::Text(msg)).await.is_err() {
            break;
        }
    }
}
