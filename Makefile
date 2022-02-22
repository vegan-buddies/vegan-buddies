docker-compose: Dockerfile
	mkdir -p db
	mkdir -p docker-home
	mkdir -p cargo-registry
	sudo docker-compose build --build-arg UID=$(shell id -u)
