export HTTP_HOST=0.0.0.0
export HTTP_PORT=8888
export DATABASE_URL=postgres://postgres:mysecretpassword@localhost/cms
export RUST_LOG=info

cargo run
