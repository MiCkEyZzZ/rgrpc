# Пример простого сервиса на Rust используя gRPC

## Запуск локально:

```zsh
git clone git@github.com:MiCkEyZzZ/rgrpc.git
# запуск сервера:
cargo run --bin logo-server
# запуск клиента:
cargo run --bin echo-client -- "Привет gRPC"
```