# op-cache

Fast cache for [1Password CLI](https://developer.1password.com/docs/cli) reads. Avoids repeated `op read` calls on every `direnv reload` or shell init. Cache TTL is 4 hours.

## Install

### Homebrew

```bash
brew install simonlansing/tap/op-cache
```

### Binary

Download from [Releases](https://github.com/simonlansing/direnv-libs/releases) and place in your `$PATH`.

## Usage

```bash
# Read a secret (cached)
op-cache read "op://vault/item/password"

# Read a secret to a file (cached)
op-cache read-file "op://vault/item/key" ./keyfile

# Clear the cache
op-cache clear
```

### In .envrc

```bash
export DB_PASSWORD=$(op-cache read "op://vault/item/password")
op-cache read-file "op://vault/item/key" ./keyfile
```

## Prerequisites

- [1Password CLI](https://developer.1password.com/docs/cli) (`op`)
- [direnv](https://direnv.net/) (optional, for `.envrc` usage)
