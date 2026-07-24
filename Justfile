# Настройки Just
set dotenv-load := true

# Ограничиваем количество потоков сборки Cargo для всего Justfile
export CARGO_BUILD_JOBS := "4"

# Вывод списка доступных команд
default:
    @just --list

# --- КОМАНДЫ ДЛЯ СОВМЕСТНОГО ЗАПУСКА ---

# Запуск всего проекта (серверной и клиентской части одновременно в одном терминале)
dev:
    @echo "🚀 Запуск личного кабинета (Axum + Vue)..."
    just run & (cd client && yarn dev)

# --- КОМАНДЫ ДЛЯ СЕРВЕРНОЙ ЧАСТИ ---

# Запуск проверки кода серверной части
check:
    cargo check --manifest-path server/Cargo.toml

# Запуск серверной части в режиме разработки
run:
    cargo run --manifest-path server/Cargo.toml

# Сборка серверной части в релизный бинарник
build:
    cargo build --release --manifest-path server/Cargo.toml

# --- КОМАНДЫ ДЛЯ КЛИЕНТСКОЙ ЧАСТИ ---

# Установка зависимостей клиентской части
client-install:
    @echo "📦 Установка зависимостей клиентской части..."
    cd client && yarn install 

# Запуск клиентской части в режиме разработки
client-dev:
    cd client && yarn dev

# Сборка клиентской части для боя (в папку client/dist)
client-build:
    cd client && yarn build

# Запуск линтера
client-check:
    cd client && yarn lint

# --- КОМАНДЫ ДЛЯ МИГРАЦИЙ (SQLx) ---

# Проверка статуса миграций базы данных
db-status:
    cd server && sqlx migrate status

# Запуск миграции базы данных
db-migrate:
    cd server && sqlx migrate run

# Создание новой миграции (Пример: just db-new create_orders_table)
db-new name:
    cd server && sqlx migrate add {{name}}
