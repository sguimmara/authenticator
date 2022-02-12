DOCKER_IMAGE_NAME = authenticator
DOCKER_REPO       = sguimmara
DOCKER_TAG        = latest
DOCKER_IMAGE      = $(DOCKER_REPO)/$(DOCKER_IMAGE_NAME):$(DOCKER_TAG)
CARGO_PROFILE     = release
DOCKER_BUILDER    = builder

all: build

build:
	cargo build --profile $(CARGO_PROFILE)

test:
	cargo test

clean:
	cargo clean

fmt:
	cargo fmt --check

lint:
	cargo clippy -- -D warnings

docker:
	echo building Docker image $(DOCKER_IMAGE)
	docker build -f Dockerfile -t $(DOCKER_IMAGE) .

docker-build:
	echo building Docker image $(DOCKER_IMAGE) with '$(DOCKER_BUILDER)' target
	docker build -f Dockerfile --target $(DOCKER_BUILDER) -t $(DOCKER_IMAGE) .
