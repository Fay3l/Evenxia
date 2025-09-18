use axum::{
    extract::{Request, State}, http::{Response, StatusCode}, middleware::{from_fn, Next}, response::Html, routing::{delete, get, post, put}, Json, Router
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::{json, Value};

use crate::{
    jwt::{create_jwt, Claims},
    models::{AppState, CreateEvent, CreateUser, LoginUser, UpdateEvent, UpdateUser},
};

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/api", get(handler).layer(from_fn(validate_token)))
        .route("/api/register", post(handler_create_user))
        .route("/api/login", post(handler_login))
        .route("/api/users", put(handler_update_user).layer(from_fn(validate_token)))
        .route("/api/users", delete(handler_delete_user).layer(from_fn(validate_token)))
        .route("/api/events", post(handler_create_event).layer(from_fn(validate_token)))
        .route("/api/events", put(handler_update_event).layer(from_fn(validate_token)))
        .route("/api/events", delete(handler_delete_event).layer(from_fn(validate_token)))
        .route("/api/events", get(handler_get_events))
}

async fn validate_token(
    req: Request,
    next: Next,
) -> Result<Response<axum::body::Body>, Json<Value>> {
    let auth_header = req.headers().get("Authorization");

    if let Some(auth_header) = auth_header {
        let token = auth_header.to_str().unwrap().replace("Bearer ", "");
        let validation = Validation::default();
        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &validation,
        ) {
            Ok(_) => return Ok(next.run(req).await),
            Err(_) => {
                return Err(Json(json!({
                    "status": StatusCode::UNAUTHORIZED.as_u16(),
                    "error": "Invalid token"
                })))
            }
        }
    }
    Err(Json(json!({
        "status": StatusCode::UNAUTHORIZED.as_u16(),
        "error": "Missing token"
    })))
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Evenxia api</h1>")
}

async fn handler_login(
    State(state): State<AppState>,
    Json(payload): Json<LoginUser>,
) -> axum::Json<Value> {
    let res = state
        .db
        .verify_user(&payload.username, &payload.email, &payload.password)
        .await
        .unwrap();
    if res != uuid::Uuid::nil() {
        let token = create_jwt(res);
        return Json(json!({"token": token, "user_id": res}));
    } else {
        return Json(
            json!({"code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(), "message": "Invalid credentials"}),
        );
    }
}

async fn handler_create_user(State(state): State<AppState>, Json(payload): Json<CreateUser>) {
    state.db.create_user(&payload).await.unwrap();
}

async fn handler_update_user(State(state): State<AppState>, Json(payload): Json<UpdateUser>) {
    state.db.update_user(payload).await.unwrap();
}

async fn handler_delete_user(State(state): State<AppState>, Json(payload): Json<uuid::Uuid>) {
    state.db.delete_user(payload).await.unwrap();
}

async fn handler_get_events(State(state): State<AppState>) -> axum::Json<Value> {
    let events = state.db.get_events().await.unwrap();
    Json(json!({"events": events}))
}

async fn handler_create_event(State(state): State<AppState>, Json(payload): Json<CreateEvent>) {
    state.db.create_event(payload).await.unwrap();
}

async fn handler_update_event(State(state): State<AppState>, Json(payload): Json<UpdateEvent>) {
    state.db.update_event(payload).await.unwrap();
}
async fn handler_delete_event(State(state): State<AppState>, Json(payload): Json<uuid::Uuid>) {
    state.db.delete_event(payload).await.unwrap();
}
