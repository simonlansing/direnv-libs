# op-cache

Fast cache for [1Password CLI](https://developer.1password.com/docs/cli) reads. Avoids repeated `op read` calls on every `direnv reload` or shell init. Cache TTL is 4 hours.

## Install

### Homebrew

```bash
brew install simonlansing/tap/op-cache
```

### Binary

Download from [Releases](https://github.com/simonlansing/direnv-libs/releases) and place in your `$PATH`.

### Shell script (no binary needed)

You can also use the shell-only version by adding to your `.envrc`:

```bash
source_url "https://raw.githubusercontent.com/simonlansing/direnv-libs/vX.Y.Z/op-cache.sh" "sha256-<hash>"
```

Generate the SHA256 hash:

```bash
curl -sL "https://raw.githubusercontent.com/simonlansing/direnv-libs/vX.Y.Z/op-cache.sh" | openssl dgst -sha256 -binary | openssl base64 -A
```

## Usage

### CLI

```bash
# Read a secret (cached)
op-cache read "op://vault/item/password"

# Read a secret to a file (cached)
op-cache read-file "op://vault/item/key" ./keyfile

# Clear the cache
op-cache clear
```

### In .envrc (with CLI)

```bash
export DB_PASSWORD=$(op-cache read "op://vault/item/password")
```

### In .envrc (with shell script)

```bash
export DB_PASSWORD=$(cached_op_read "op://vault/item/password")
cached_op_read_file "op://vault/item/key" ./keyfile
op_cache_clear
```

## Prerequisites

- [1Password CLI](https://developer.1password.com/docs/cli) (`op`)
- [direnv](https://direnv.net/) (optional, for `.envrc` usage)
