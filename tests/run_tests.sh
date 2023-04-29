#!/bin/bash

cleanup() {
  echo " ===================================="
  echo " ==== Removing opened containers ===="
  echo " ===================================="
  docker rm -f treq_container_app
  docker rm -f treq_container_app2
}

trap cleanup EXIT

echo " ========================"
echo " ===== FILE HANDLER ====="
echo " ========================"
docker run -d -it --name treq_container_app treq
docker exec -it treq_container_app cargo test integration --release -- --test-threads=1 --ignored || exit 1

echo " ========================"
echo " ====== e2e tests ======="
echo " ========================"
docker run -d -it --name treq_container_app2 treq
docker exec -it treq_container_app2 cargo test integration --release -- --test-threads=1 --ignored || exit 1

