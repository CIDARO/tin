FROM rust:latest

WORKDIR /usr/local/tin
COPY . .
RUN cargo build --release
RUN cp target/release/tin /usr/bin/

EXPOSE 80
CMD ["tin", "--key", "secret_key"]