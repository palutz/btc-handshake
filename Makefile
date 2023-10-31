appname = minigrep

rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

clean:
	cargo clean

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

release:
	cargo build --release

docker:
	docker build -t $(appname) .
	
docker_run:
	docker run -it -p 3113:3113 $(appname)

docker_stop:
	docker ps -q --filter ancestor="$(appname)" | xargs -r docker stop

docker_clean:
	docker image prune && docker container prune

all: format lint test run
