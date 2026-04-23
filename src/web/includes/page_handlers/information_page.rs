use crate::web::prelude::*;
use crate::web::includes::session::check_session;


pub async fn information_handler(State(state): State<AppState>, session: Session) -> impl IntoResponse {
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
  
    if let Ok(template) = state.template.get_template("information") {
        let html = template.render(context! {
        }).unwrap();
        return Html(html).into_response()
    }
  
    (   
        StatusCode::NOT_FOUND,
        [("Content-type", "text/html")],
        include_str!("../../templates/404.html")
    ).into_response()
}
