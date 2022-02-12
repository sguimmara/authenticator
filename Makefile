DOCKER_IMAGE_NAME = authenticator
DOCKER_REPO       = sguimmara
DOCKER_TAG        = latest
DOCKER_IMAGE      = $(DOCKER_REPO)/$(DOCKER_IMAGE_NAME):$(DOCKER_TAG)
CARGO_PROFILE     = release

all: build

build:
	cargo build --profile $(CARGO_PROFILE)

clean:
	cargo clean

docker:
	echo building Docker image $(DOCKER_IMAGE)
	docker build -f Dockerfile -t $(DOCKER_IMAGE) .