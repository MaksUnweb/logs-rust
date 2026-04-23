#![allow(unused)]
pub use axum::{
    response::{IntoResponse, Json, Html, Redirect},
    extract::State,
    http::StatusCode,
};
pub use tower_sessions::Session;
pub use minijinja::context;
pub use std::sync::Arc;
pub use sqlx::postgres::PgPool;
pub use crate::web::web_errors::LogErrors;
pub use crate::web::AppState;
pub use serde::{Serialize, Deserialize};



pub const SESSION_KEY: &str = "auth_session";
