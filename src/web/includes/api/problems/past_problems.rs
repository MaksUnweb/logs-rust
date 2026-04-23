use sqlx::prelude::FromRow;

use crate::web::prelude::*;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Problem {
   text_error: String 
}

// Функция для запуска вывода последних проблем
pub async fn select(pool: Arc<PgPool>) -> Result<Option<Vec<Problem>>, sqlx::error::Error> {
     match select_from_db(pool).await {
        Ok(data) => {
            if data.iter().len() > 0 {
                return Ok(Some(data));
            }else {
               return Ok(None); 
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
}

// Делаем вывод:
async fn select_from_db(pool: Arc<PgPool>) -> Result<Vec<Problem>, sqlx::error::Error> {
    let data: Vec<Problem> = sqlx::query_as::<_, Problem>("SELECT text_error FROM logs ORDER BY time_data DESC LIMIT 5")
        .fetch_all(&*pool)
        .await?;
    Ok(data)
} 
