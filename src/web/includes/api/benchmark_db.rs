use crate::web::prelude::*;
use serde_json::json;
use tokio::time::Instant;
use std::time::Duration;
use std::collections::HashMap;
use sqlx::Row;

//Функция для проверки скорости выборки из базы данных, 
//необходимо для примерного понимания скорости работы всей бд в целом
pub async fn test_speed(State(state): State<AppState>) -> impl IntoResponse {

    let pool = state.pool;
    let cloned_pool = pool.clone();
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
    match insert_into_db(cloned_pool, avg_ms).await {
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
async fn insert_into_db(pool: Arc<PgPool>, time: f64) -> Result<(), sqlx::error::Error> {

    let _sql = sqlx::query("INSERT INTO speed_test (time) VALUES ($1)")
        .bind(time)
        .execute(&*pool)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_recalculation() {
        let avg = Duration::from_nanos(29000);
        let avg_ms = recalculation_units(avg);
        let true_value = 0.058;
        assert_eq!(avg_ms, true_value);
    }
}


