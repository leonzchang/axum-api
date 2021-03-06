# Path and Variables
SHELL := /bin/bash
ORG := Alpha-Carbon
PROJECT := vrf-oracle-service
REPO := github.com/${ORG}/${PROJECT}
ROOT_DIR := $(CURDIR)
SEM_VER := $(shell awk -F' = ' '$$1=="version"{print $$2;exit;}' ./Cargo.toml)

.PHONY: deps deps-rust semver docker

semver:
	@echo ${SEM_VER}

###########################################################
### Database

migrate:
	source ${ROOT_DIR}/env/.env ; \
	export DATABASE_URL=$$DATABASE_URL; \
	sqlx db create ; \
	sqlx migrate --source migrations run

###########################################################
### Local Deployment

local: local-pg migrate
	source ${ROOT_DIR}/env/.env ; \
	RUST_LOG=info RUST_BACKTRACE=1 cargo run --release --bin axum-api ; \

local-pg:
	source ${ROOT_DIR}/env/.env ; \
	docker-compose -f ${ROOT_DIR}/deployment/docker-compose.yaml up -d postgres adminer ; \
	sleep 3