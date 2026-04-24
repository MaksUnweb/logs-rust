//Этот модуль необходим для веб-интерфейса. 
//Этот файл является точкой входа веба
mod prelude;
mod web_errors;
mod includes;
use axum::{
    routing::{get, post},
    Router,
};
use minijinja::Environment;
use tower_sessions_sqlx_store::{sqlx::PgPool,PostgresStore};
use tower_sessions::{SessionManagerLayer, Expiry};
use crate::web::prelude::*;
use crate::web::includes::login;
use crate::web::includes::page_handlers::index_page;
use crate::web::includes::page_handlers::problems_page;
use crate::web::includes::page_handlers::information_page;
use crate::web::includes::page_handlers::politics_page;
use crate::web::includes::api::api_routers;
use log::error;
use time::Duration;
use tower_http::services::ServeDir;

type StartResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    template: Arc<Environment<'static>>,
    pool: Arc<PgPool>
}

trait CriticalErrors<T> {
    fn session_error(self, msg: &str) -> T;
}

impl<T, E: std::fmt::Display> CriticalErrors<T> for Result<T, E> {
    fn session_error(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(e) => {
                error!("{}: {}", msg, e);
                panic!("{}: {}", msg, e);
            }
        }
    }
}

//Функция для запуска веб-сервера:
pub async fn start(pool: PgPool, addr: String) -> StartResult {
   let mut env = Environment::new(); 
   env.add_template("header", include_str!("./templates/header.html"))?;
   env.add_template("footer", include_str!("./templates/footer.html"))?;
   env.add_template("login", include_str!("./templates/login.html"))?;
   env.add_template("home", include_str!("./templates/home.html"))?;
   env.add_template("problems", include_str!("./templates/problems.html"))?;
   env.add_template("information", include_str!("./templates/information.html"))?;
   env.add_template("politics", include_str!("./templates/politics.html"))?;
   env.add_template("error_page", include_str!("./templates/404.html"))?;

   
    let pool_for_store = pool.clone();

    // Создаём хранилище сессий:
    let session_store = PostgresStore::new(pool_for_store)
        .with_schema_name("public").session_error("Error adding to the public schema")
        .with_table_name("sessions").session_error("Error adding sessions table name");

    session_store.migrate().await?;

    let session_layer = SessionManagerLayer::new(session_store)
    .with_secure(false) 
    .with_expiry(Expiry::OnInactivity(Duration::hours(1)));

    //Запускаем миграции, если таблиц в базе данных нету, то они будут созданы:
    sqlx::migrate!("./migrations").run(&pool).await?;

   let state = AppState {
        template: Arc::new(env),
        pool: Arc::new(pool)
   };

    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/home") }),)
        .route("/home", get(index_page::index_handler))
        .route("/problems", get(problems_page::problems_handler))
        .route("/information", get(information_page::information_handler))
        .route("/politics", get(politics_page::politics_handler))
        .route("/login", get(login::login_template))
        .route("/login", post(login::login_handler))
        .nest("/api", api_routers::api_router())
        .fallback(not_found)
        .with_state(state)
        .layer(session_layer)
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();

   Ok(())
}


//Функция для отображения fallback (шаблона ошибки)
async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        [("Content-type", "text/html")],
        include_str!("./templates/404.html")
    )
}
