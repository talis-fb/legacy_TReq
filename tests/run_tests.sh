docker build -t treq_minha_imagem -f tests/integration/Dockerfile .
docker run -d -it --name nome_container treq_minha_imagem:latest
docker exec -it nome_container cargo run

docker exec -it nome_container cargo test integration --release -- --test-threads=1 --ignored

# docker exec -it nome_container [COMANDO DESEJADO]
