test: build
	docker run --rm katac_tests

build:
	docker build -f Dockerfile.tests -t katac_tests . 

build-app:
	docker build -t katac .
