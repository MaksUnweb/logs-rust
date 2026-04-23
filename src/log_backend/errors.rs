#![allow(dead_code)]
use thiserror::Error;
use log::error;


#[derive(Error, Debug)]
pub enum LogErrors {
    #[error("Ошибка работы базы данных")]
    DatabaseError(#[from] sqlx::error::Error)
}


impl LogErrors {
    pub fn log(&self) {
        match self {
            LogErrors::DatabaseError(e) => {
                error!("{:?}", e);
            }
        }
    }
}

