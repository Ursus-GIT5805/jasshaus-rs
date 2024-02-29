BIND="127.0.0.1"

build:
	# cargo build
	wasm-pack build --target web --dev

run:
	make build
	python3 -m http.server --bind $(BIND)
