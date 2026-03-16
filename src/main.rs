use std::{env, net::SocketAddr, sync::Arc};

use axum::{
    Json, Router,
    extract::{Request, State},
    http::{
        HeaderMap, HeaderValue, StatusCode,
        header::{AUTHORIZATION, WWW_AUTHENTICATE},
    },
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};

#[derive(Clone)]
struct AppState {
    username: Arc<str>,
    password: Arc<str>,
}

#[tokio::main]
async fn main() {
    let username = env::var("BASIC_AUTH_USER").unwrap_or_else(|_| "admin".to_string());
    let password = env::var("BASIC_AUTH_PASS").unwrap_or_else(|_| "password".to_string());
    let addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let socket_addr: SocketAddr = addr.parse().expect("BIND_ADDR is invalid");

    let state = AppState {
        username: username.into(),
        password: password.into(),
    };

    let protected = Router::new()
        .route("/private", get(private_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            basic_auth_middleware,
        ));

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .merge(protected)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .expect("failed to bind TCP listener");

    println!("listening on http://{socket_addr}");
    axum::serve(listener, app).await.expect("server error");
}

async fn root_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "message": "hello",
        "protected_endpoint": "/private"
    }))
}

async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn private_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "message": "basic auth passed"
    }))
}

async fn basic_auth_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    let authorized = extract_basic_auth(req.headers())
        .map(|(u, p)| u == state.username.as_ref() && p == state.password.as_ref())
        .unwrap_or(false);

    if !authorized {
        return unauthorized_response();
    }

    next.run(req).await
}

fn extract_basic_auth(headers: &HeaderMap) -> Option<(String, String)> {
    let auth = headers.get(AUTHORIZATION)?.to_str().ok()?;
    let encoded = auth.strip_prefix("Basic ")?;
    let decoded = BASE64_STANDARD.decode(encoded).ok()?;
    let decoded = String::from_utf8(decoded).ok()?;
    let (user, pass) = decoded.split_once(':')?;
    Some((user.to_string(), pass.to_string()))
}

fn unauthorized_response() -> Response {
    let mut headers = HeaderMap::new();
    headers.insert(
        WWW_AUTHENTICATE,
        HeaderValue::from_static(r#"Basic realm="restricted", charset="UTF-8""#),
    );
    (StatusCode::UNAUTHORIZED, headers, "Unauthorized").into_response()
}
