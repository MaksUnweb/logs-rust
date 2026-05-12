use crate::web::prelude::*;
use serde_json::json;
use tokio::time::Instant;
use std::time::Duration;
use std::collections::HashMap;
use sqlx::{Database, Executor, Postgres, Row};

//Функция для проверки скорости выборки из базы данных, 
//необходимо для примерного понимания скорости работы всей бд в целом
pub async fn test_speed(State(state): State<AppState>) -> impl IntoResponse {

    let pool = state.pool;
    let iterations = 1000;

    let start = Instant::now();

    for _ in 0..iterations {
        let _ = sqlx::query("SELECT * FROM logs LIMIT 1")
        .fetch_one(&*pool)
        .await.unwrap();
    }

    let duration = start.elapsed();
    //Среднее время выполнения одного запроса:
    let avg = duration / iterations as u32;
     let avg_ms = recalculation_units(avg);


    //Запускаем вставку в базу данных:
    match insert_into_db(&*pool, &avg_ms).await {
        Ok(()) => {
            (
            StatusCode::OK,
            [("Content-type", "application/json")],
            Json(json!({
                "message": "Success!",
                "avg": avg_ms,
            }))
        ).into_response()
        }
        Err(e) => {
            LogErrors::DatabaseError(e);
            (
            StatusCode::INTERNAL_SERVER_ERROR,
            [("Content-type", "application/json")],
            Json(json!({
                "message": "Error!",
            }))
            ).into_response()
        }
    }
}


//Функция для обработки GET-запроса вывода информации о последнем тестировании:
pub async fn get_past_test(State(state): State<AppState>) -> impl IntoResponse {
    let pool = state.pool;
     match select_past_test(pool).await {
        Ok(res) => {
            (
            StatusCode::OK,
            [("Content-type", "application/json")],
            Json(json!({
                "message": "Success!",
                "data": res
            }))
            ).into_response()
        }
        Err(e) => {
            let error_message = e.to_string();
            LogErrors::DatabaseError(e);
            (
            StatusCode::INTERNAL_SERVER_ERROR,
            [("Content-type", "application/json")],
            Json(json!({
                "message": error_message,
            }))
            ).into_response()
        }
    }
}

//Выводим один последний тест:
async fn select_past_test(pool: Arc<PgPool>) -> Result< Option<HashMap<String, String>>, sqlx::error::Error> {
      let row = sqlx::query("SELECT time::DOUBLE PRECISION, TO_CHAR(date, 'DD.MM.YYYY') as date_str FROM speed_test WHERE date = (SELECT MAX(date) FROM speed_test) ORDER BY id DESC LIMIT 1")
        .fetch_optional(&*pool)
        .await?;


    let data = row.map(|row| {
            let mut map: HashMap<String, String> = HashMap::new();
            map.insert("time".to_string(), row.get::<f64, _>(0).to_string());
            map.insert("date".to_string(), row.get::<String, _>(1).to_string());
            map
    });

    Ok(data)
}

//Функция для пересчёта из наносекунд в миллисекунды:
fn recalculation_units(avg: Duration) -> f64 {
     let avg_ms = avg.as_secs_f64() * 1000.0 + (avg.subsec_nanos() as f64) / 1_000_000.0;
     avg_ms
}

//Функция для внесения данных о последнем тесте в бд:
async fn insert_into_db<'e, E>(executor: E, time: &f64) -> Result<(), sqlx::error::Error>
where E: Executor<'e, Database = Postgres>
{

    let _sql = sqlx::query("INSERT INTO speed_test (time) VALUES ($1)")
        .bind(time)
        .execute(executor)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use sqlx::postgres::PgPoolOptions;

use super::*;

    #[tokio::test]
    async fn test_recalculation() {
        let avg = Duration::from_nanos(29000);
        let avg_ms = recalculation_units(avg);
        let true_value = 0.058;
        assert_eq!(avg_ms, true_value);
    }

    //Test for check working function for insert benchmark data:
    #[tokio::test]
    async fn test_insert_data() {
        let pool = connection_to_db().await.expect("Error connecting to db! Please check Database or Auth data!");
        let mut transaction = pool.begin().await.expect("Error creating transaction!");
        let test_time: f64 = 1111.11;
        //Start inserting test time:
        insert_into_db(&mut *transaction, &test_time).await.expect("Error inserting data!");
        //Check test time in db, if isset, test completed:
        match check_inserted_time(&mut *transaction, &test_time).await {
            Ok(_) => {}
            Err(e) => {
                panic!("The test failed! Check error: {}", e);
            }
        }

        transaction.rollback().await.expect("Error rollback transaction!");
    }

    async fn connection_to_db() -> Result<PgPool, sqlx::error::Error> {
        let pool = PgPoolOptions::new() 
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(3))
            .connect("postgres://Admin:1111@127.0.0.1:5432/logs_db?connection_timeout=3")
            .await?;
        Ok(pool)
    }

    async fn check_inserted_time<'e, E>(executor: E, time: &f64) -> Result<(), sqlx::error::Error>
    where E: Executor<'e, Database = Postgres>
    {
        let count: i64 = sqlx::query_scalar(r#"
            SELECT COUNT(*) FROM speed_test WHERE time = $1
            "#)
            .bind(time)
            .fetch_one(executor)
            .await?;

        if count > 0 {
            return Ok(());
        }else {
        dbg!(count);
             return Err(sqlx::Error::RowNotFound);
        }
    }
}

