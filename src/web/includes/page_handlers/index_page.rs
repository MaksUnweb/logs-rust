use crate::web::prelude::*;
use crate::web::includes::session::check_session;
use crate::web::includes::api::problems::past_problems;
use crate::web::includes::api::get_admin_name;


//Метод для загрузки шаблона:
pub async fn index_handler(State(state): State<AppState>, session: Session) -> impl IntoResponse {

    let pool = state.pool;
    //Проверяю наличие сессии:
    let admin_id = match check_session(session).await {
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


    //Получаю последние ошибки для шаблона:
    let problems = match past_problems::select(pool.clone()).await {
        Ok(data) => {data}
        Err(e) => {
            LogErrors::DatabaseError(e);
            None
        }
    };
        
    //Получаем имя пользователя:
    let admin_name = match get_admin_name::get(admin_id, pool.clone()).await {
        Ok(name) => {name},
        Err(e) => {
            LogErrors::DatabaseError(e);
            None
        }
    };

    //Загружаем шаблон:
    if let Ok(template) = state.template.get_template("home") {
        let html = template.render(context! {
            admin_name => admin_name,
            problems => problems
        }).unwrap();
        return Html(html).into_response()
    }

    (   
        StatusCode::NOT_FOUND,
        [("Content-type", "text/html")],
        include_str!("../../templates/404.html")
    ).into_response()
}
