# Используем официальный образ Rust
FROM rust:1.75-slim as builder

# Устанавливаем системные зависимости
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Создаем рабочую директорию
WORKDIR /app

# Копируем файлы проекта
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Собираем проект в release режиме
RUN cargo build --release

# Финальный образ
FROM debian:bookworm-slim

# Устанавливаем runtime зависимости
RUN apt-get update && apt-get install -y \
    libssl3 \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Копируем собранный бинарник
COPY --from=builder /app/target/release/blockchain_project /usr/local/bin/blockchain_project

# Открываем порт
EXPOSE 3000

# Запускаем приложение
CMD ["blockchain_project"]
