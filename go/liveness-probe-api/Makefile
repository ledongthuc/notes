GOCMD=go
GOBUILD=$(GOCMD) build
GORUN=$(GOCMD) run 
GOTEST=$(GOCMD) test
BINARY_NAME=liveness-probe-api
DOCKER_IMG_NAME=$(BINARY_NAME)

all: test build
build: 
	$(GOBUILD) -o $(BINARY_NAME) -v
run:
	$(GORUN) main.go
docker-build:
	docker build -t ledongthuc/$(DOCKER_IMG_NAME):latest .
docker-push: docker-build
	docker push ledongthuc/$(DOCKER_IMG_NAME):latest
docker-run: docker-build
	docker run -p 8080:8080 ledongthuc/liveness-probe-api
