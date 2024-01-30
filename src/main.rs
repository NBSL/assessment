use ::entities::{devices, devices::Entity as Device, questions, questions::Entity as Question};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use dotenvy::dotenv;
use listenfd::ListenFd;
use sea_orm::{ActiveValue, Database, DatabaseConnection, EntityTrait};
use serde_json::json;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let conn = Database::connect(
        std::env::var("DATABASE_URL")
            .unwrap_or("postgres://postgres:dev_password@localhost:5432/postgres".to_string()),
    )
    .await
    .unwrap();
    let state = AppState { db: conn };
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/devices", get(devices_handler))
        .route(
            "/devices/:id",
            get(device_handler).post(questionnair_handler),
        )
        .route("/questions", get(questions_handler))
        .with_state(state);
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => TcpListener::from_std(listener).unwrap(),
        None => TcpListener::bind("127.0.0.1:3000").await.unwrap(),
    };
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

pub(crate) async fn root_handler() -> impl IntoResponse {
    "Hello, world!".to_string()
}

pub(crate) async fn devices_handler(State(state): State<AppState>) -> impl IntoResponse {
    let db = &state.db;
}

pub(crate) async fn device_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let db = &state.db;
}

pub(crate) async fn questions_handler(State(state): State<AppState>) -> impl IntoResponse {
    let db = &state.db;
    let questions = Question::find().all(db).await;
    Json(json!(questions))
}

pub(crate) async fn questionnair_handler(
    State(state): State<AppState>,
    Json(payload): Json<QuestionPayload>,
) -> impl IntoResponse {
    println!("{:?}", payload);
    let data = entities::devices::ActiveModel {
        id: ActiveValue::NotSet,
        description: ActiveValue::Set(payload.description.clone()),
        grade: ActiveValue::Set(payload.grade.clone()),
    };
    let db = &state.db;
    let res = entities::devices::Entity::insert(data)
        .exec(db)
        .await
        .unwrap();
    dbg!(res);
    Json(json!("{msg: Hello, World}"))
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct QuestionPayload {
    pub description: String,
    pub grade: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, StatusCode},
        routing::post,
        Json, Router,
    };
    use hyper::Request;
    use serde_json::json;
    use tower::{Service, ServiceExt};

    #[tokio::test]
    async fn test_post() {
        dotenv().ok();

        let conn = Database::connect(
            std::env::var("DATABASE_URL")
                .unwrap_or("postgres://postgres:dev_password@localhost:5432/postgres".to_string()),
        )
        .await
        .unwrap();
        let state = AppState { db: conn };
        let app = Router::new()
            .route("/", post(questionnair_handler))
            .with_state(state);
        let res = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .uri("/")
                    .body(Body::from(
                        serde_json::to_vec(
                            &json!({"description": "A device description", "grade": "A"}),
                        )
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
    }
}
