use crate::web::prelude::*;

//Функция для проверки наличия сессии:
// pub async fn check_session(session: Session) -> Result<Option<i64>, Box<dyn std::error::Error + Send + Sync>> {
pub async fn check_session(session: Session) -> Result<Option<i64>, tower_sessions::session::Error> {
    let value = session.get::<i64>(SESSION_KEY).await?;
    Ok(value)
}
