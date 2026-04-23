
-- Таблица для администраторов:
CREATE TABLE IF NOT EXISTS admins (
  id BIGSERIAL PRIMARY KEY,
  login VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL
);

-- Таблица для логов:

CREATE TABLE IF NOT EXISTS logs (
  id BIGSERIAL PRIMARY KEY,
  is_error BOOLEAN NOT NULL,
  text_error TEXT NOT NULL,
  time_data timestamp 
);

-- Таблица для хранения результатов тестирования базы данных:
CREATE TABLE IF NOT EXISTS speed_test (
  id BIGSERIAL PRIMARY KEY,
  time NUMERIC(10,4) NOT NULL,
  date DATE NOT NULL DEFAULT CURRENT_DATE
);


-- Уникальный индекс для поля text_error:
CREATE UNIQUE INDEX IF NOT EXISTS idx_logs_text_error ON logs (text_error);
