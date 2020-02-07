GOCMD=go
GOBUILD=$(GOCMD) build
GORUN=$(GOCMD) run 
GOTEST=$(GOCMD) test
BINARY_NAME=authenticate_with_cookie
DOCKER_USERNAME=ledongthuc
DOCKER_IMG_NAME=$(BINARY_NAME)
DOCKER_VERSION=latest

all: build
build: 
	$(GOBUILD) -o $(BINARY_NAME) -v
run:
	$(GORUN) main.go
docker-build:
	docker build -t $(DOCKER_USERNAME)/$(DOCKER_IMG_NAME):$(DOCKER_VERSION) .
docker-push: docker-build
	docker push $(DOCKER_USERNAME)/$(DOCKER_IMG_NAME):$(DOCKER_VERSION)
docker-run: docker-build
	docker run -p 8080:8080 $(DOCKER_USERNAME)/$(BINARY_NAME):$(DOCKER_VERSION)
