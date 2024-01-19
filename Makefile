target=aarch64-unknown-linux-musl
ip=`ifconfig en0 | grep inet | cut -d " " -f 2`

.PHONY: clean
clean:
	docker images -f "dangling=true" | grep -v kindest | awk 'NR!=1{print $$3}' | xargs docker rmi

.PHONY: build
build:
	cargo zigbuild --release --target=${target} -p hot_update_server
hot_update_server
.PHONY: image
image: build
	docker build --no-cache -f backend.dockerfile  --build-arg target=${target} --build-arg ip=${ip} -t yuexclusive/hot_update_server:latest .
	make clean

.PHONY: run
run: image
	docker run --rm -p 8881:8881 -it yuexclusive/hot_update_server:latest

.PHONY: image_nginx
image_nginx: build
	docker build --no-cache -f backend_nginx.dockerfile  --build-arg target=${target} --build-arg ip=${ip} -t yuexclusive/hot_update_server_nginx:latest .
	make clean

.PHONY: run_nginx
run_nginx: image_nginx
	docker run --rm -p 8881:80 -it yuexclusive/hot_update_server_nginx:latest


MODULE_NAME:="user"
LNGUAGE:="rust"
.PHONY: openapi
openapi:
	openapi-generator generate -i http://localhost:8881/api-doc/${MODULE_NAME}.json -g ${LNGUAGE} --package-name ${MODULE_NAME}_cli -o ./openapi_cli/${MODULE_NAME}_cli