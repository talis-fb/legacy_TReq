FROM rust:1.68-bullseye

RUN apt-get update && apt-get install -y vim

WORKDIR /app
COPY . .

RUN cargo build --release

RUN useradd -ms /bin/bash appuser
USER appuser

ENV EDITOR=vim

CMD ["./target/release/treq"]
