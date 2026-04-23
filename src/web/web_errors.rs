#![allow(dead_code)]


use thiserror::Error;
use log::error;
use minijinja::Error as MinijinjaError;

#[derive(Error, Debug)]
pub enum LogErrors {
    #[error("Database working error!")] 
    DatabaseError(#[from] sqlx::Error),
    #[error("Error getting the template")]
    MinijinjaTemplateError(#[from] MinijinjaError),
    #[error("Session store error!")]
    SessionStoreError(#[from] tower_sessions::session::Error),
    #[error("Error making migrations")]
    MigrationsError(#[from] sqlx::migrate::MigrateError),
    #[error("Background task failed!")]
    JoinError(#[from] tokio::task::JoinError),
}

trait Logger {
    fn log(&self) {}
}

impl Logger for LogErrors {
    fn log(&self) {
       match &self {
        LogErrors::DatabaseError(e) => {
            error!("{}: {}", self.to_string(), e);
        }
        LogErrors::MinijinjaTemplateError(e) => {
            error!("{}: {}", self.to_string(), e);
        }
        LogErrors::SessionStoreError(e) => {
            error!("{}: {}", self.to_string(), e);
        }
        LogErrors::MigrationsError(e) => {
            error!("{}: {}", self.to_string(), e);
        }
        LogErrors::JoinError(e) => {
            error!("{}: {}", self.to_string(), e);
        }
       } 
    }
}

