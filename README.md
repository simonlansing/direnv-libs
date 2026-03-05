# direnv-libs

Shared [direnv](https://direnv.net/) libraries for sipgate projects.

## Available Libraries

### op-cache.sh

Caches [1Password CLI](https://developer.1password.com/docs/cli) reads to avoid repeated `op read` calls on every `direnv reload`. Cache TTL is 4 hours.

**Functions:**

- `cached_op_read <op-ref>` - drop-in replacement for `$(op read <op-ref>)`
- `cached_op_read_file <op-ref> <dest-path>` - reads a secret and writes it to a file
- `op_cache_clear` - removes all cached secrets

## Usage

Add to your `.envrc`:

```bash
source_url "https://raw.githubusercontent.com/sipgate/direnv-libs/v1.0.0/op-cache.sh" "sha256-<hash>"
```

Then use `cached_op_read` instead of `op read`:

```bash
export DB_PASSWORD=$(cached_op_read "op://vault/item/password")
```

## Updating

1. Check the [releases](https://github.com/sipgate/direnv-libs/releases) for the latest version
2. Generate the SHA256 hash for the new version:
   ```bash
   curl -sL "https://raw.githubusercontent.com/sipgate/direnv-libs/vX.Y.Z/op-cache.sh" | shasum -a 256
   # Output: d5558cd4...  -
   ```
   Prefix the hash with `sha256-` for `source_url`:
   ```bash
   source_url "https://raw.githubusercontent.com/sipgate/direnv-libs/vX.Y.Z/op-cache.sh" "sha256-d5558cd4..."
   ```
3. Update the version and hash in your `.envrc`
4. Run `direnv allow` to accept the changes

## Prerequisites

- [direnv](https://direnv.net/)
- [1Password CLI](https://developer.1password.com/docs/cli) (`op`)
