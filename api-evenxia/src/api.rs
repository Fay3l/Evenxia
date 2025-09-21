use axum::{
    extract::{Request, State},
    http::{Response, StatusCode},
    middleware::{from_fn, Next},
    response::Html,
    routing::{delete, get, post, put},
    Json, Router,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::{json, Value};

use crate::{
    error::AppError,
    jwt::{create_jwt, Claims},
    models::{AppState, CreateEvent, CreateUser, LoginUser, UpdateEvent, UpdateUser},
};

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/api", get(handler).layer(from_fn(validate_token)))
        .route("/api/register", post(handler_create_user))
        .route("/api/login", post(handler_login))
        .route(
            "/api/users",
            put(handler_update_user).layer(from_fn(validate_token)),
        )
        .route(
            "/api/users",
            delete(handler_delete_user).layer(from_fn(validate_token)),
        )
        .route(
            "/api/events",
            post(handler_create_event).layer(from_fn(validate_token)),
        )
        .route(
            "/api/events",
            put(handler_update_event).layer(from_fn(validate_token)),
        )
        .route(
            "/api/events",
            delete(handler_delete_event).layer(from_fn(validate_token)),
        )
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
)-> Result<(StatusCode,Json<Value>), AppError> {
    let res = state
        .db
        .verify_user(&payload.username, &payload.email, &payload.password)
        .await
        .unwrap();
    if res != uuid::Uuid::nil() {
        let token = create_jwt(res);
        return Ok((StatusCode::OK, Json(json!({"token": token, "user_id": res}))));
    } else {
        return Ok((
            StatusCode::UNAUTHORIZED,
            Json(json!({"code": StatusCode::UNAUTHORIZED.as_u16(), "message": "Invalid credentials"})),
        ));
    }
}

async fn handler_create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode,Json<Value>), AppError> {
    state.db.create_user(&payload).await?;
    Ok((
        StatusCode::CREATED,
        Json(json!({"message": "User created successfully"})),
    ))
}

async fn handler_update_user(State(state): State<AppState>, Json(payload): Json<UpdateUser>)-> Result<(StatusCode,Json<Value>), AppError> {
    state.db.update_user(payload).await?;
    Ok((
        StatusCode::OK,
        Json(json!({"message": "User updated successfully"})),
    ))
}

async fn handler_delete_user(State(state): State<AppState>, Json(payload): Json<uuid::Uuid>) -> Result<(StatusCode,Json<Value>), AppError> {
    state.db.delete_user(payload).await?;
    Ok((
        StatusCode::OK,
        Json(json!({"message": "User deleted successfully"})),
    ))
}

async fn handler_get_events(State(state): State<AppState>) -> Result<(StatusCode,Json<Value>), AppError>  {
    let events = state.db.get_events().await?;
    Ok((StatusCode::OK,
        Json(json!({"events": events}))
    ))
}

async fn handler_create_event(State(state): State<AppState>, Json(payload): Json<CreateEvent>)-> Result<(StatusCode,Json<Value>), AppError> {
    state.db.create_event(payload).await?;
    Ok((
        StatusCode::CREATED,
        Json(json!({"message": "Event created successfully"})),
    ))
}

async fn handler_update_event(State(state): State<AppState>, Json(payload): Json<UpdateEvent>)-> Result<(StatusCode,Json<Value>), AppError> {
    state.db.update_event(payload).await?;
    Ok((
        StatusCode::OK,
        Json(json!({"message": "Event updated successfully"})),
    ))
}
async fn handler_delete_event(State(state): State<AppState>, Json(payload): Json<uuid::Uuid>)-> Result<(StatusCode,Json<Value>), AppError> {
    state.db.delete_event(payload).await?;
    Ok((
        StatusCode::OK,
        Json(json!({"message": "Event deleted successfully"})),
    ))
}
