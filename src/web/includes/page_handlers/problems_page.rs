use axum::extract::Query;

use crate::web::prelude::*;
use crate::web::includes::session::check_session;
use crate::web::includes::api::problems::problems_select::{self, Problem};

#[derive(Deserialize, Default)]
pub struct Pagination
{
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_page_size")]
    pub page_size: i32,
    message: Option<String>,
    is_error: Option<bool> 
}

fn default_page() -> i32 {
    1
}

fn default_page_size() -> i32 {
    5
}


impl Pagination {
    pub async fn start(&self, pool: Arc<PgPool>) -> Result<Option<Vec<Problem>>, sqlx::error::Error> {
        let offset = (self.page - 1) * self.page_size;
        let vec_problems = problems_select::select_db(pool, offset, self.page_size).await?;
        Ok(vec_problems)
    }
}

pub async fn problems_handler(State(state): State<AppState>, Query(pagination): Query<Pagination> ,session: Session) -> impl IntoResponse {
    //Проверяю наличие сессии:
    let _admin_id = match check_session(session).await {
        Ok(Some(admin_id)) => {
            admin_id 
        }
        Ok(None) => {
            return Redirect::to("/login").into_response();
        }
        Err(e) => {
           LogErrors::SessionStoreError(e);
           return Redirect::to("/login?message=The service is temporarily unavailable, please try again later!").into_response();
        }
    };

    //Проверяю наличие ошибки (Это необходимо для обратного редиректа после нажатия кнопки
    //correction_btn):
    let mut message = String::new();
    let mut is_error = false;
    if pagination.message.is_some() {
        message = pagination.message.clone().unwrap_or_default();
        is_error = pagination.is_error.clone().unwrap_or_default();
    }

    //Получаю список всех записей и 5 записей на вывод:
    let pool = state.pool;
    let vec_problems = match pagination.start(pool.clone()).await {
        Ok(Some(data)) => {data}
        Ok(None) => {vec![]}
        Err(e) => {
            LogErrors::DatabaseError(e);
            vec![]
        }
    };

    //Список всех элементов: 
    let elements: i64;
    if vec_problems.iter().len() > 0{
        elements = vec_problems[0].total_count;
    }else {
        elements = 0;
    }
    //Текущее "смещение", проще говоря сколько элементов пропущено 
    let current_offset = pagination.page * pagination.page_size;

    if let Ok(template) = state.template.get_template("problems") {
        if let Ok(html) = template.render(context! {
            is_error => is_error,
            message => message,
            vec_problems => vec_problems,
            current_page => &pagination.page,
            page_size => &pagination.page_size,
            element_count => elements,
            current_offset => current_offset
        }) {
            return Html(html).into_response()
        }
    }

    (   
        StatusCode::NOT_FOUND,
        [("Content-type", "text/html")],
        include_str!("../../templates/404.html")
    ).into_response()

}
