-- Создание таблицы пользователей
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),                  -- Уникальный ID каждого пользователя, генерируется автоматически
    email VARCHAR(255) NOT NULL UNIQUE,                             -- Почта обязательна и должна быть уникальной
    password_hash VARCHAR(255) NOT NULL,                            -- Хэш пароля (сам пароль не хранится в открытом виде)
    name VARCHAR(100),                                              -- Имя пользователя (необязательное поле)
    avatar_url TEXT,                                                -- Ссылка на аватар (необязательное поле)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()                   -- Дата и время создания учетной записи
);

-- Создание таблицы записей пользователя
CREATE TABLE items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),                  -- Уникальный ID записи
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,   -- Связь с пользователем. Если пользователя удалят, удалится и все его записи (CASCADE)
    title VARCHAR(255) NOT NULL,                                    -- Заголовок записи обязателен
    content TEXT,                                                   -- Содержимое записи (может быть пустым)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()                   -- Дата создания записи
);

-- Индекс для быстрого поиска записей конкретного пользователя
CREATE INDEX idx_items_user_id ON items(user_id);
