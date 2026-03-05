# Changelog

## [2.0.0](https://github.com/breadctl/direnv-libs/compare/v1.1.0...v2.0.0) (2026-03-05)


### ⚠ BREAKING CHANGES

* remove shell script in favor of Rust CLI binary

### Features

* remove shell script in favor of Rust CLI binary ([04a16d1](https://github.com/breadctl/direnv-libs/commit/04a16d1bc6f7e428f0fae9b8be6086e9e199b140))


### Bug Fixes

* use checkout action for homebrew-tap push ([6502814](https://github.com/breadctl/direnv-libs/commit/6502814d37347ab91858d29811f5239caa1f16f5))
* use libc::getuid() instead of process::id() for cache directory ([2b6ac2a](https://github.com/breadctl/direnv-libs/commit/2b6ac2ada7eb2857e625a80fd67d580fe24f1140))

## [1.1.0](https://github.com/breadctl/direnv-libs/compare/v1.0.0...v1.1.0) (2026-03-05)


### Features

* add Rust CLI binary with cross-platform build and Homebrew support ([f2746d0](https://github.com/breadctl/direnv-libs/commit/f2746d0a8ed8d4294299b9a1684f256d850f27f3))

## 1.0.0 (2026-03-05)


### Features

* initial release with op-cache library ([4ba1092](https://github.com/breadctl/direnv-libs/commit/4ba109203a7b00fa5527e578c8835de614dbb59e))
