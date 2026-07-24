## Серверная часть

Количество потоков при сборке серверной части ограничено 4-мя.

<details>
<summary style="font-size: 1.25em; font-weight: 600;">🛠️ Этапы создания рабочей основы проекта</summary>

### Создание проекта (cargo new project-name). Добавление зависимостей в `Cargo.toml`.

### (если не установлен) Установка `PostgreSQL`

```bash
sudo pacman -S postgresql
```

#### Если работа выполняется на `Arch Linux` могут потребоваться следующие шаги:

Инициализация кластера базы данных:

```bash
sudo -u postgres initdb --locale=C.UTF-8 --encoding=UTF8 -D '/var/lib/postgres/data'
```

где

- `sudo -u postgres` - требование выполнить команду от имени системного пользователя `postgres`. Благодаря этому все созданные файлы базы данных будут принадлежать пользователю `postgres` и у СУБД будет к ним доступ.

- `--locale=C.UTF-8` - локаль (региональные настройки) определяющая правила сравнения, сортировки строк и форматирования дат/чисел. `C.UTF-8` работает значительно быстрее остальных локалей и гарантирует, что базы данных будут вести себя абсолютно одинаково на любом сервере в мире, независимо от того, какие языковые настройки выбраны в самой операционной системе.

- `--encoding=UTF8` - кодировка которая указывает, в каком формате символы сохраняются на диск. Кодировка `UTF8` гарантирует, что база данных будет без проблем сохранять и отображать любые языки (кириллицу, латиницу, эмодзи, иероглифы) без появления «кракозябр».

### Глобальная установка `sqlx-cli`

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### Создание файла `.env` и добавление переменной окружения `DATABASE_URL` для `SQLx`

```env
DATABASE_URL=postgres://username:password@localhost:5432/user_cabinet_db
```

Вместо `username` и `password` нужно указать действительные данные (например на postgres:postgres). После создания файла `.env` необходимо добавить его `.gitignore`.

#### Активация службы `postgresql`

Настройка старта вместе с системой:

```bash
sudo systemctl enable postgresql
```

Старт в рамках текущей сессии:

```bash
sudo systemctl start postgresql
```

### Создание базы данный через CLI

```bash
sqlx database create
```

Создание заготовки для файлов миграций (`SQL-скриптов`), с помощью которой будет происходить создание и изменение таблицы в базе данных:

```bash
sqlx migrate add init_schema
```

После выполнения команды в корне проекта должна появится папка `migrations`, а внутри неё файл с расширением `.sql`. Данный файл необходимо открыть и добавить следующее содержимое:

<details>
<summary>*.sql</summary>

```sql
-- Создание таблицы пользователей
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(100),
    avatar_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Создание таблицы записей пользователя
CREATE TABLE items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    content TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Индекс для быстрого поиска записей конкретного пользователя
CREATE INDEX idx_items_user_id ON items(user_id);
```

</details>

Запуск миграции:

```bash
sqlx migrate run
```

</details>

<details>
<summary style="font-size: 1.25em; font-weight: 600;">🗄️ Структура проекта</summary>

```bash
├── client
│   ├── .vscode/
│   ├── node_modules/
│   ├── public/
│   ├── src/
│   │   ├── assets/
│   │   ├── components/
│   │   ├── App.vue
│   │   ├── main.ts
│   │   └── style.css
│   ├── index.html
│   ├── package.json
│   ├── README.md
│   ├── tsconfig.app.json
│   ├── tsconfig.json
│   ├── tsconfig.node.json
│   ├── vite.config.ts
│   └── yarn.lock
├── server
│   ├── migrations/
│   ├── src/
│   │   ├── middleware/         # Промежуточный слой
│   │   │   ├── auth.rs
│   │   │   └── mod.rs
│   │   ├── routes/             # Маршруты и обработчики
│   │   │   ├── auth.rs         # Регистрация и вход
│   │   │   ├── items.rs        # Записи: получение и создание
│   │   │   ├── mod.rs          # Объединение маршрутов
│   │   │   └── user.rs         # Профиль: получение и обновление
│   │   ├── main.rs             # Точка входа
│   │   ├── models.rs           # Структуры данных для БД и API
│   │   └── utils.rs            # Вспомогательные функции (хеширование паролей, генерация JWT)
│   └── Cargo.toml
├── target/
├── .env
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── Justfile
└── README.md
```
</summary>
