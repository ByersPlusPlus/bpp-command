build:
	rustup component add rustfmt
	cargo build --release

docker-build:
	docker run --rm -v "$(shell pwd)":/usr/src/{{project-name}} rust:1.54.0-slim-buster /bin/bash -c "cd /usr/src/{{project-name}} && apt update && apt install make -y && make"