use clap::{Parser, Subcommand};
use sha2::{Digest, Sha256};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::{Command, ExitCode};
use std::time::{Duration, SystemTime};

const CACHE_TTL: Duration = Duration::from_secs(4 * 60 * 60); // 4 hours

#[derive(Parser)]
#[command(name = "op-cache", about = "Fast cache for 1Password CLI reads")]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Read a secret (cached)
    Read {
        /// 1Password reference (e.g. op://vault/item/field)
        reference: String,
    },
    /// Read a secret and write it to a file (cached)
    ReadFile {
        /// 1Password reference
        reference: String,
        /// Destination file path
        dest: PathBuf,
    },
    /// Clear the cache
    Clear,
}

fn cache_dir() -> PathBuf {
    let uid = std::process::id();
    PathBuf::from(format!("/tmp/op-cache-{uid}"))
}

fn cache_key(reference: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(reference.as_bytes());
    hex::encode(hasher.finalize())
}

fn ensure_cache_dir(dir: &PathBuf) -> std::io::Result<()> {
    if !dir.exists() {
        fs::create_dir_all(dir)?;
        fs::set_permissions(dir, fs::Permissions::from_mode(0o700))?;
    }
    Ok(())
}

fn is_cache_valid(path: &PathBuf) -> bool {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .map(|mtime| {
            SystemTime::now()
                .duration_since(mtime)
                .unwrap_or(Duration::MAX)
                < CACHE_TTL
        })
        .unwrap_or(false)
}

fn op_read(reference: &str) -> Result<String, String> {
    let output = Command::new("op")
        .args(["read", reference])
        .output()
        .map_err(|e| format!("failed to run op: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("op read failed: {stderr}"));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn cached_read(reference: &str) -> Result<String, String> {
    let dir = cache_dir();
    ensure_cache_dir(&dir).map_err(|e| format!("cache dir: {e}"))?;

    let cache_file = dir.join(cache_key(reference));

    if is_cache_valid(&cache_file) {
        return fs::read_to_string(&cache_file).map_err(|e| format!("cache read: {e}"));
    }

    let value = op_read(reference)?;
    fs::write(&cache_file, &value).map_err(|e| format!("cache write: {e}"))?;
    fs::set_permissions(&cache_file, fs::Permissions::from_mode(0o600))
        .map_err(|e| format!("cache permissions: {e}"))?;

    Ok(value)
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Cmd::Read { reference } => match cached_read(&reference) {
            Ok(value) => {
                print!("{value}");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("op-cache: {e}");
                ExitCode::FAILURE
            }
        },
        Cmd::ReadFile { reference, dest } => match cached_read(&reference) {
            Ok(value) => {
                if let Err(e) = fs::write(&dest, &value) {
                    eprintln!("op-cache: write {}: {e}", dest.display());
                    return ExitCode::FAILURE;
                }
                let _ = fs::set_permissions(&dest, fs::Permissions::from_mode(0o600));
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("op-cache: {e}");
                ExitCode::FAILURE
            }
        },
        Cmd::Clear => {
            let dir = cache_dir();
            if dir.exists() {
                if let Err(e) = fs::remove_dir_all(&dir) {
                    eprintln!("op-cache: {e}");
                    return ExitCode::FAILURE;
                }
            }
            eprintln!("op-cache: cache cleared");
            ExitCode::SUCCESS
        }
    }
}
