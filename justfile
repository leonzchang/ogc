set dotenv-load

local-mono-ogc:
    RUST_BACKTRACE=1 RUST_LOG=info,sqlx=error cargo run --bin ogc mono