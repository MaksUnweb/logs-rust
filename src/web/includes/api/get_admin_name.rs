use sqlx::Row;

use crate::web::prelude::*;


//Функция для получения имени админа по его ID:
pub async fn get(id: i64, pool: Arc<PgPool>) -> Result<Option<String>, sqlx::error::Error> {
   let result = sqlx::query("SELECT login FROM admins WHERE id = $1") 
       .bind(id)
       .fetch_optional(&*pool)
       .await?;
    let name = match result {
        Some(row) =>  Some(row.get("login")),
        None => None
    };
    Ok(name)
}
