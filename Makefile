api:
	docker-compose -p rust-web-server-compose up -d api
.PHONY: api

keycloak-database:
	docker-compose -f ./docker/keycloak/docker-compose.yaml -p rust-web-server-compose up -d keycloak_database
.PHONY: keycloak-database

keycloak-service:
	docker-compose -f ./docker/keycloak/docker-compose.yaml -p rust-web-server-compose up -d keycloak
.PHONY: keycloak-service

setup-env:
	cp .env.example .env
.PHONY: setup-env

stop:
	docker-compose stop
.PHONY: stop

down:
	docker-compose -p rust-web-server-compose rm -f -s api
	docker-compose -p rust-web-server-compose stop keycloak_database
.PHONY: down

all: keycloak-database keycloak-service api
.PHONY: all