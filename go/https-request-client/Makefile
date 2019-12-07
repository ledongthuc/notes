GOCMD=go
GOBUILD=$(GOCMD) build
GORUN=$(GOCMD) run 
GOTEST=$(GOCMD) test
BINARY_NAME=http-request-client
DOCKER_USERNAME=ledongthuc
DOCKER_IMG_NAME=$(BINARY_NAME)
DOCKER_VERSION=latest

ENV_URL=https://postman-echo.com/get

all: test build
build: 
	$(GOBUILD) -o $(BINARY_NAME) -v
run:
	URL="$(ENV_URL)" $(GORUN) main.go
docker-build:
	docker build -t $(DOCKER_USERNAME)/$(DOCKER_IMG_NAME):$(DOCKER_VERSION) .
docker-push: docker-build
	docker push $(DOCKER_USERNAME)/$(DOCKER_IMG_NAME):$(DOCKER_VERSION)
docker-run: docker-build
	docker run $(DOCKER_USERNAME)/$(BINARY_NAME):$(DOCKER_VERSION) env URL="$(ENV_URL)" ./app
