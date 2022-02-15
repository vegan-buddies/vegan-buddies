docker-compose: Dockerfile
	mkdir -p db
	mkdir -p docker-home
	sudo docker-compose build --build-arg UID=$(shell id -u)
