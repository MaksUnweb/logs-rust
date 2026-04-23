use axum::{
    extract::Form
};
use crate::web::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormData 
{
    pub id: i64,
    pub url_redirect: String
}

//Функция-обработчик для удаления лога из бд:
pub async fn start(State(state): State<AppState>, Form(form): Form<FormData>) -> impl IntoResponse {
    let pool = state.pool;
    let mut path = format!("");
     match delete_from_db(form.id, pool).await {
        Ok(true) => {
            path = format!("{}&message=Sussecc!&is_error=false", form.url_redirect);
            return Redirect::to(&path).into_response()
        }
        Ok(false) => {
            path = format!("{}&message=The deletion did not go according to plan!&is_error=true", form.url_redirect);
            return Redirect::to(&path).into_response()
        }
        Err(_) => {
            path = format!("{}&message=The deletion did not go according to plan!&is_error=true", form.url_redirect);
            return Redirect::to(&path).into_response()
        }
     }
}

//Удаляем из бд с текущим ID:
async fn delete_from_db(id: i64, pool: Arc<PgPool>) -> Result<bool, sqlx::error::Error> {

    let rows = sqlx::query("DELETE FROM logs WHERE id = $1")
        .bind(id)
        .execute(&*pool)
        .await?;
    if rows.rows_affected() > 0 {
        return Ok(true);
    }else {
        return Ok(false);
    }
}
