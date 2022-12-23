FROM rust:latest

WORKDIR /usr/myapp
COPY . .

RUN cargo install --path .

CMD ["hermes"]
