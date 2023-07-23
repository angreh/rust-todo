FROM rust:1.69.0
WORKDIR /api
COPY api .
EXPOSE 8080
RUN cargo install cargo-watch
CMD cargo watch -w Cargo.lock -w ./src -x run