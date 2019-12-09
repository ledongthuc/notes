GOCMD=go
GOBUILD=$(GOCMD) build
GORUN=$(GOCMD) run 
GOTEST=$(GOCMD) test
BINARY_NAME=gocui-docker-gotty
DOCKER_USERNAME=ledongthuc
DOCKER_IMG_NAME=$(BINARY_NAME)
DOCKER_VERSION=latest

all: test build
build: 
	$(GOBUILD) -o $(BINARY_NAME) -v
run:
	go run main.go
run-gotty:
	gotty --close-timeout=180 go run main.go
docker-build:
	docker build -t $(DOCKER_USERNAME)/$(DOCKER_IMG_NAME):$(DOCKER_VERSION) .
docker-push: docker-build
	docker push $(DOCKER_USERNAME)/$(DOCKER_IMG_NAME):$(DOCKER_VERSION)
docker-run: docker-build
	docker run -it --rm $(DOCKER_USERNAME)/$(BINARY_NAME):$(DOCKER_VERSION)
docker-run-gotty: docker-build
	gotty -w --close-timeout=180 docker run -it --rm $(DOCKER_USERNAME)/$(BINARY_NAME):$(DOCKER_VERSION)
