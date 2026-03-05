# op-cache.sh - direnv library for caching 1Password CLI reads
# Source this from .envrc to avoid repeated `op read` calls on every direnv reload.
# Requires: op CLI (https://developer.1password.com/docs/cli)

_OP_CACHE_DIR="${XDG_RUNTIME_DIR:-/tmp}/op-cache-$(id -u)"
_OP_CACHE_TTL=14400 # 4 hours in seconds

_op_cache_init() {
  if [[ ! -d "$_OP_CACHE_DIR" ]]; then
    mkdir -p "$_OP_CACHE_DIR"
    chmod 700 "$_OP_CACHE_DIR"
  fi
}

_op_cache_key() {
  printf '%s' "$1" | shasum -a 256 | cut -d' ' -f1
}

_op_cache_is_valid() {
  local cache_file="$1"
  [[ -f "$cache_file" ]] || return 1
  local now file_mtime age
  now=$(date +%s)
  file_mtime=$(stat -f%m "$cache_file" 2>/dev/null || stat -c%Y "$cache_file")
  age=$((now - file_mtime))
  (( age < _OP_CACHE_TTL ))
}

# cached_op_read <op-ref>
# Drop-in replacement for $(op read <op-ref>)
cached_op_read() {
  local ref="$1"
  _op_cache_init
  local key cache_file
  key=$(_op_cache_key "$ref")
  cache_file="$_OP_CACHE_DIR/$key"

  if _op_cache_is_valid "$cache_file"; then
    cat "$cache_file"
    return 0
  fi

  local value
  value=$(op read "$ref") || return $?
  printf '%s' "$value" > "$cache_file"
  chmod 600 "$cache_file"
  cat "$cache_file"
}

# cached_op_read_file <op-ref> <dest-path>
# Drop-in replacement for: op read <op-ref> > <dest-path>
cached_op_read_file() {
  local ref="$1"
  local dest="$2"
  _op_cache_init
  local key cache_file
  key=$(_op_cache_key "$ref")
  cache_file="$_OP_CACHE_DIR/$key"

  if _op_cache_is_valid "$cache_file"; then
    cp "$cache_file" "$dest"
    chmod 600 "$dest"
    return 0
  fi

  local value
  value=$(op read "$ref") || return $?
  printf '%s' "$value" > "$cache_file"
  chmod 600 "$cache_file"
  cp "$cache_file" "$dest"
  chmod 600 "$dest"
}

# op_cache_clear - Remove all cached secrets
op_cache_clear() {
  rm -rf "$_OP_CACHE_DIR"
  log_status "op-cache cleared"
}
