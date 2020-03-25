GOCMD=go
GOBUILD=$(GOCMD) build
GORUN=$(GOCMD) run 
GOTEST=$(GOCMD) test
BINARY_NAME=hello-api
DOCKER_USERNAME=ledongthuc
DOCKER_IMG_NAME=$(BINARY_NAME)
DOCKER_VERSION=local
CUSTOM_ROUTES=/custom_url:this is custom;/random.8b3ec8ff2ac96774ff4e6965e9bba307.html:this is random

all: test build
test:
	$(GOTEST) ./...
build: 
	$(GOBUILD) -o $(BINARY_NAME) -v
run:
	CUSTOM_ROUTES="$(CUSTOM_ROUTES)" $(GORUN) main.go
docker-build:
	docker build -t $(DOCKER_USERNAME)/$(DOCKER_IMG_NAME):$(DOCKER_VERSION) .
docker-run:
	docker run -p 8080:8080 $(DOCKER_USERNAME)/$(BINARY_NAME):$(DOCKER_VERSION) env CUSTOM_ROUTES="$(CUSTOM_ROUTES)" ./app
