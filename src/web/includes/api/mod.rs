pub mod benchmark_db;
pub mod problems;
pub mod get_admin_name;


pub mod api_routers {
    use axum::{Router, routing::{get, post}};
    use crate::web::{AppState, 
        includes::api::benchmark_db,
        includes::api::problems::correction_problem
    };

    pub fn api_router() -> Router<AppState> {
        Router::new()
           .route("/test_db", get(benchmark_db::test_speed))
           .route("/select_test_db", get(benchmark_db::get_past_test))
           .route("/correction_problem", post(correction_problem::start))
    }
}
