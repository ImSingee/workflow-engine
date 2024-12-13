set dotenv-load

default:
    @just --list --justfile {{justfile()}}

test:
    cargo nextest run

bench:
    cargo bench

build-napi:
    cd crates/workflow-engine-napi && pnpm build

test-napi:
    cd crates/workflow-engine-napi && pnpm test

format:
    cargo +nightly fmt