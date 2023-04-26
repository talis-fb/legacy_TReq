echo " ========================"
echo " ===== CREATE IMAGE ====="
echo " ========================"
docker build -t treq -f tests/Dockerfile .


echo " Tests ............................................"
echo " ========================"
echo " ===== FILE HANDLER ====="
echo " ========================"
docker run -d -it --name container_app treq
docker exec -it container_app cargo test integration --release -- --test-threads=1 --ignored

echo " ========================"
echo " ===== e2e ====="
echo " ========================"
docker run -d -it --name container_app2 treq
docker exec -it container_app2 cargo test integration --release -- --test-threads=1 --ignored

# docker exec -it nome_container [COMANDO DESEJADO]
