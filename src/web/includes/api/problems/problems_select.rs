use sqlx::prelude::FromRow;

use crate::web::prelude::*;

#[derive(FromRow, Serialize)]
pub struct Problem {
    pub id: i64,
    pub is_error: bool,
    pub text_error: String,
    pub total_count: i64
}


//Метод для вывода всех записей с Limit и Offset,
//при этом здесь также есть подзапрос для вывода количества записей в бд в целом:
pub async fn select_db(pool: Arc<PgPool>, offset: i32, limit: i32) -> Result<Option<Vec<Problem>>, sqlx::error::Error> {
    let data: Vec<Problem> = sqlx::query_as::<_, Problem>("SELECT
    id, is_error, text_error,
    (SELECT COUNT(*) FROM logs) AS total_count
FROM logs
ORDER BY time_data
LIMIT $1 OFFSET $2")
        .bind(limit)
        .bind(offset)
        .fetch_all(&*pool)
        .await?;
    if data.iter().len() > 0 {
        return Ok(Some(data));
    }else {
        return Ok(None);
    }
}

