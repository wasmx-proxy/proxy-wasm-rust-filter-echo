.PHONY: raze
raze:
	cargo raze --generate-lockfile

.PHONY: bazel
bazel:
	bazel build --platforms=@rules_rust//rust/platform:wasm //...

.PHONY: cargo
cargo:
	cargo build --target=wasm32-unknown-unknown
