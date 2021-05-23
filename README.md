# puny2d
[![Crate Version](https://img.shields.io/crates/v/puny2d.svg?label=puny2d&logo=rust)](https://crates.io/crates/puny2d) [![Crate Documentation](https://img.shields.io/badge/puny2d-docs-blue?logo=rust)](https://docs.rs/puny2d) [![Minimal rust version](https://img.shields.io/badge/rust-v1.52%2B-blue?logo=rust)](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1521-2021-05-10)  
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/substation-beta/puny2d/Build%20project?logo=github)](https://github.com/substation-beta/puny2d/actions?query=workflow%3A%22Build+project%22) [![Code Coverage](https://img.shields.io/codecov/c/github/substation-beta/puny2d.svg?logo=Codecov)](https://codecov.io/gh/substation-beta/puny2d) [![Deps.rs dependency status for GitHub repo](https://deps.rs/repo/github/substation-beta/puny2d/status.svg)](https://deps.rs/repo/github/substation-beta/puny2d)

**puny2d** is a 2d graphics software renderer library for programming language [rust](https://www.rust-lang.org/).

## Build
* Compile: `cargo build --release`
  * Output: file **target/release/libpuny2d.rlib**
* Documentation generation: `cargo doc --no-deps`
  * Output: directory **target/doc** (file **puny2d/index.html**)

## Develop
* Compile check: `cargo check`
* Code linting: `cargo clippy`
* Code formatting: `cargo fmt`
* Unit tests: `cargo test`
* Benchmark tests: `cargo bench`