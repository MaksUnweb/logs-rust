mod web;
mod log_backend;
mod init_admin;
use std::env;

use sqlx::postgres::{PgPool, PgPoolOptions};
use dotenv::dotenv;

async fn connect_db(url: String) -> Result<PgPool, sqlx::error::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&url).await?;

    Ok(pool)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();
  let url = env::var("DB_URL").expect("Ошибка получения URL базы данных для подключения!");
  let host_web = env::var("CONFIG_HOST").expect("Ошибка получения HOST для старта веб-сервера!");
  let port_web = env::var("CONFIG_PORT").expect("Ошибка получения PORT для старта веб-сервера!");
  let addr = format!("{}:{}", host_web, port_web);

  
    //Подключаемся к базе данных:
    let pool = connect_db(url).await?;
    let next_pool = pool.clone();

    //Проверяем, есть ли админ в бд, если нету, то запускается скрипт регистрации прямиков в
    //командной строке:
    init_admin::init(pool.clone()).await?;


    let logs_handler = tokio::spawn(async move {
        if let Err(e) = log_backend::start(pool.clone()).await {
            eprintln!("Logs crashed: {}", e);
        }
    });

    let web_handler = tokio::spawn(async move{
        if let Err(e) = web::start(next_pool, addr).await {
            eprintln!("Web crashed: {}", e);
         }
    });


    let _ = tokio::join!(logs_handler, web_handler);
    Ok(())
}
