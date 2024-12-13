set dotenv-load

default:
    @just --list --justfile {{justfile()}}

test:
    cargo test

bench:
    cargo bench

build-napi:
    cd crates/workflow-engine-napi && pnpm build

test-napi:
    cd crates/workflow-engine-napi && pnpm test

format:
    cargo +nightly fmt