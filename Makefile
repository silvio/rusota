
TARGET ?= x86_64-unknown-linux-gnu
RELEASE ?= true
REGISTRY ?= docker.io
REGISTRY_USERNAME ?= $(shell id -un)
GITREV := $(shell git rev-parse HEAD)
GITREV := $(GITREV)$(shell git diff --quiet || echo ".dirty")
BUILDDATE := $(shell date --iso-8601=s)

ifeq ($(RELEASE),)
	BUILD_STRING :=
	BUILD_TARGET := debug
else
	BUILD_STRING := --release
	BUILD_TARGET := release
endif

bin-build:
	cargo build --target=$(TARGET) $(BUILD_STRING)

bin-build-docker: TARGET = x86_64-unknown-linux-musl
bin-build-docker: bin-build strip-build

strip-build:
	strip "target/$(TARGET)/release/rusota"

docker-build: bin-build-docker
	docker build \
		--progress plain \
		--build-arg="VERSION=$(BUILD_TARGET)" \
		--label="GITREV=$(GITREV)" \
		--label="BUILDDATE=$(BUILDDATE)" \
		--tag="${REGISTRY}/$(REGISTRY_USERNAME)/rusota:latest" \
		--tag="${REGISTRY}/$(REGISTRY_USERNAME)/rusota:$(GITREV)" \
		.
