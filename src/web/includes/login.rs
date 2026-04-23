use crate::web::prelude::*;
use axum::extract::{Form, Query};
use minijinja::context;
use sqlx::FromRow;
use argon2::password_hash::PasswordHash;
use argon2::{Argon2, PasswordVerifier};
use validator::Validate;
use std::sync::LazyLock;
use regex::Regex;


static RE_ADMIN_DATA: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[a-zA-Z0-9_@-]").unwrap()
});

const NOT_VALID: &'static str = "The login or password is not valid!";
const NO_ADMIN: &'static str = "The login or password is incorrect!";
const SERVER_ERROR: &'static str = "The service is temporarily unavailable, please try again later!";


#[derive(Debug, Validate, Deserialize)]
#[allow(dead_code)]
struct ValidateData {
    #[validate(regex(path = *RE_ADMIN_DATA))]
    login: String,
    password: String
}

#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    pub login: String,
    pub password: String
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Admin {
    id: i64,
    password: String
}

#[derive(Deserialize)]
pub struct PageError {
    pub message: Option<String> 
}

pub async fn login_template(Query(params): Query<PageError>, State(state): State<AppState>) -> impl IntoResponse {
    //Проверяем есть ли сообщение об ошибке, если есть, то передаём его на шаблон:
    let mut message = String::new();
    if params.message.is_some() {
        message = params.message.clone().unwrap_or_default();
    }

    let html = state.template.get_template("login").unwrap()
        .render(context! {
            message => message
        }).unwrap();
    Html(html)
}

pub async fn login_handler(State(state): State<AppState>, session: Session, Form(form): Form<LoginForm>) -> impl IntoResponse {
    let pool = state.pool;
    let login = form.login;
    let password = form.password;

    if let Err(_) = validate_data(login.clone(), password.clone()).await {
        let path = format!("/login?message={}", NOT_VALID);
        return Redirect::to(&path).into_response();
    };
    
    //Получаем админа по логину, если нету, возвращаем редирект с ошибкой:
    let admin = match check_in_db(pool, login).await {
        Ok(admin) => {
           admin 
        }
        Err(sqlx::Error::RowNotFound) => {
            let path = format!("/login?message={}", NO_ADMIN);
            return Redirect::to(&path).into_response();
        }
        Err(other) => {
            LogErrors::DatabaseError(other);
            let path = format!("/login?message={}", SERVER_ERROR);
            return Redirect::to(&path).into_response();
        }
    };

    match password_verify(password, admin.password).await {
        Ok(result) => {
            if result {
             session.insert(SESSION_KEY, admin.id).await.unwrap();
            return Redirect::to("/").into_response()
            }else {
            let path = format!("/login?message={}", NO_ADMIN);
            return Redirect::to(&path).into_response()
            }
        }
        Err(_) => {
            let path = format!("/login?message={}", NO_ADMIN);
            return Redirect::to(&path).into_response()
        }
    }
}


//Функция для проверки валидности данных:
async fn validate_data(login: String, password: String) -> Result<(), validator::ValidationErrors> {
    let valid = ValidateData {
        login: login,
        password: password
    };

    match valid.validate() {
        Ok(()) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e)
        }
    }
}


//Функция для получения данных в бд об администраторе (если такой найден по логинУ):
async fn check_in_db(pool: Arc<PgPool>, login: String) -> Result<Admin, sqlx::error::Error> {
    let admin: Admin = sqlx::query_as::<_, Admin>("SELECT id, password FROM admins WHERE login = $1")
        .bind(login)
        .fetch_one(&*pool)
        .await?;

    Ok(admin)
}

//WARNING!!!!
//Операция верификации CPU-зависима, обязательно должна быть в tokio::task::spawn_blocking!!!
async fn password_verify(password: String, password_hash: String) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    tokio::task::spawn_blocking(move || -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let parsed_hash = PasswordHash::new(&password_hash)?;
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    })
    .await?
}
