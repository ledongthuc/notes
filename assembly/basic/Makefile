CPATH = $(shell pwd)
run:
	docker build -t ubuntu_assembly:latest .
	docker run --rm -v ${CPATH}:/home/ubuntu/examples -it --name ubuntu_assembly ubuntu_assembly:latest
start:
