FROM rust:1.68-bullseye

WORKDIR /app
COPY . .

RUN useradd -ms /bin/bash appuser
RUN chown -R appuser:appuser /app

USER appuser

CMD ["cargo", "test", "integration ","--release", "--", "--test-threads=1", "--ignored"]
