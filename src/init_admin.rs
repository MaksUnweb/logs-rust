use sqlx::{Row, postgres::{
    PgPool}};
use std::io;
use std::io::Write;
use std::collections::HashMap;
use argon2::Argon2;
use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::password_hash::rand_core::OsRng;

async fn start_check(pool: PgPool) -> Result<i64, sqlx::error::Error> {
    let rows = sqlx::query("SELECT COUNT(*) FROM admins")
        .fetch_one(&pool)
        .await?;

    let count: i64 = rows.get("count");
    Ok(count)
}

fn get_admin_data() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut login = String::new();
    let mut password = String::new();

    print!("Login: "); io::stdout().flush()?;
    io::stdin().read_line(&mut login)?;

    print!("Password: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut password)?;

    let login = login.trim_end();
    let password = password.trim_end();
    let mut admin: HashMap<String, String> = HashMap::new();
    admin.insert("login".to_string(), login.to_string());
    admin.insert("password".to_string(), password.to_string());
    Ok(admin)
}

fn argon_hash(password: String) -> String  {
   let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
    
    password_hash
}

async fn insert_admin_into_db(pool: PgPool, login: String, password: String) -> Result<(), sqlx::error::Error> {
    sqlx::query("INSERT INTO admins (login, password) VALUES ($1, $2)")
        .bind(login)
        .bind(password)
        .execute(&pool)
        .await?;
    Ok(())
}




pub async fn init(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    //Подключаемся к базе данных:    
    //Получаем список админов и проверяем, больше ли он 0;
    //если админов нету, то запускаем процесс создания админов:
    let rows = start_check(pool.clone()).await?;
    let admin: HashMap<String, String>;

    if rows > 0 {
        return Ok(());
    }else {
        admin = get_admin_data()?;
    }

    //Хэшируем пароль:
    if let (None, None) = (&admin.get("login"), &admin.get("password")) {
        panic!("Ошибка получения данных из HashMap или внемения их туда после ввода пользователя!");
    }
    let login = admin.get("login").unwrap().to_string();
        
    let hash_password = argon_hash(admin.get("password").unwrap().to_string());
    //Загружаем данные в бд:
    match insert_admin_into_db(pool.clone(), login, hash_password).await {
        Ok(()) => {
            println!("Администратор успешно зарегистрирован! Запуск остальных служб...");
            return Ok(());
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}
