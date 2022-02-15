#!/bin/bash
docker-compose down
docker-compose up -d
exec docker exec -it vegan-buddies_userindex_1 sh
