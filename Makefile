build:
	docker build -t katac .

test: build-test
	docker run --rm katac_tests

build-test:
	docker build -f Dockerfile.tests -t katac_tests . 
