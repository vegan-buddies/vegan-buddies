docker-compose-cruft:
	mkdir -p docker-compose-cruft/db
	mkdir -p docker-compose-cruft/docker-home
	mkdir -p docker-compose-cruft/cargo-registry
	mkdir -p docker-compose-cruft/synapse-data

docker-image: matrix-geographic-user-index/Dockerfile docker-compose-cruft
	sudo docker-compose build --build-arg UID=$(shell id -u) --no-cache

matrix-users: docker-image
	sudo docker-compose up -d synapse
	sudo docker exec -it vegan-buddies_synapse_1 /start.py generate
	sudo docker exec -it vegan-buddies_synapse_1 register_new_matrix_user http://localhost:8008 -c /data/homeserver.yaml -a -u bot -p test
	sudo docker exec -it vegan-buddies_synapse_1 register_new_matrix_user http://localhost:8008 -c /data/homeserver.yaml -a -u mock_client -p test
	sudo docker-compose down

docker-compose: matrix-users
	echo "Built docker image and set up users. Run ./develop.sh to get started."
