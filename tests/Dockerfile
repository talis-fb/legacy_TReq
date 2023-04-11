FROM rust:1.68-bullseye

RUN apt-get update && apt-get install -y vim

WORKDIR /app
COPY . .

RUN useradd -ms /bin/bash appuser
RUN chown -R appuser:appuser /app

USER appuser

ENV EDITOR=vim

# CMD ["cargo", "test", "integration ","--release", "--", "--test-threads=1", "--ignored"]
CMD ["/bin/bash"]
