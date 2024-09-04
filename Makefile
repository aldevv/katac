app: build
	docker run --rm katac sh

build:
	docker build -t katac .

test: build-test
	docker run --rm katac_test

build-test:
	docker build -f Dockerfile.tests -t katac_test . 

it: build-test
	docker run -it --rm katac_test sh

link:
	cargo build
	ln -sf $(PWD)/target/debug/katac $(HOME)/.local/bin/katac

link-release:
	cargo build --release
	ln -sf $(PWD)/target/release/katac $(HOME)/.local/bin/katac
