[private]
@default:
  just --list

[group('build')]
build-debug:
  cargo build

[group('build')]
build-debug-wasm:
  cargo build --target wasm32-unknown-unknown --features "wasm"

[group('build')]
build-release:
  cargo build --target wasm32-unknown-unknown --features "wasm" --release

lint:
  cargo clippy

[group('test')]
test-debug:
  cargo test

[group('test')]
test-release:
  cargo test --release

tag:
  git tag $(cargo pkgid | sed 's/.*#//')

# vim: set ts=2 sw=2 sts=2 et: