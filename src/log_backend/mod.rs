use bollard::{
  Docker,
    query_parameters::LogsOptions,
    container::LogOutput
};
use std::{
    env,
    pin::Pin
};
use futures::StreamExt;
use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::sync::mpsc;
use log::error;
use crate::log_backend::errors::LogErrors;
mod errors;


async fn connecting_to_container() ->  impl futures::Stream<Item = Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync + 'static>>> + Send + Unpin + 'static {

    let docker = match Docker::connect_with_defaults() {
        Ok(docker) => docker,
        Err(e) => panic!("Ошибка подключения к Docker: {}", e)
    };
    dotenv().ok();
    let container = env::var("CONTAINER_ID").expect("Ошибка получения id контейнера!");

    let options = LogsOptions {
        follow: true,
        stdout: true,
        stderr: true,
        timestamps: true,
        tail: "0".to_string(),
        since: 0,
        until: 0
    };

    let logs = docker.logs(&container, Some(options));


    logs.filter_map(move |log_result| {
        Box::pin(async move {
            match log_result {
                Ok(LogOutput::StdErr { message }) =>  {
                    let bytes: Vec<u8> = message.to_vec();
                    Some(Ok(bytes))
                }
                _ => None
         }
        }) as Pin<Box<dyn Future<Output = Option<Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync + 'static>>>> + Send>>

    })
}

fn filter_data(data: String) -> Option<String> {

    if data.contains("ERROR") || data.contains("FATAL") 
    || data.contains("error") || data.contains("fatal")
    || data.contains("PANIC") || data.contains("panic"){
        return Some(data);
    }

    None
}

async fn insert_into_db(data: String, pool: PgPool) -> Result<(), LogErrors> {

    let data_cloned = data.clone();

    let timestamp_str = match data_cloned.find(' ') {
    Some(pos) => &data_cloned[..pos],      
    None => "NULL",                 
    };

    let _sql = sqlx::query(
    r#"
    INSERT INTO logs (is_error, text_error, time_data) 
    VALUES ($1, $2, CASE WHEN $3 = 'NULL' THEN NOW() ELSE $3::timestamp END)
    ON CONFLICT (text_error) DO NOTHING
    "#,
        )
        .bind(true)
        .bind(data)
        .bind(timestamp_str)
        .execute(&pool)
        .await?;
    Ok(())
}


pub async fn start(pool: PgPool) -> Result<(), LogErrors>  {
    let (tx, mut rx) = mpsc::channel::<String>(100);

    // Создаём отдельный поток для загрузки в бд:
   let _insert_log = tokio::spawn( async move  {
        while let Some(err_log) = rx.recv().await {
           if let Err(e) = insert_into_db(err_log, pool.clone()).await {
              error!("{}", e);
            };
        }
    });

     let mut stream = connecting_to_container().await;

   while let Some(item) = stream.next().await{
       match item {
            Ok(bytes) => {
                let text_logs = String::from_utf8_lossy(&bytes);
                let filtered_data = filter_data(text_logs.to_string());
                if let Some(err_log) = filtered_data {
                    if let Err(e) = tx.send(err_log).await {
                        error!("receiver dropped: {}", e);
                    };
                }
            }
            _ => {}
       }
   }
    drop(tx);
   Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;


#[tokio::test]
    async fn test_filter() {
        let test_data = String::from("[FATAL] [ERROR]");
        let result = filter_data(test_data);

        assert!(result.is_some(), "Обязательно должно быть Some!");
        let err = result.unwrap();
        assert!(err.contains("ERROR") || err.contains("FATAL"));
    }
}



