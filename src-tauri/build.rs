use std::process::Command;

fn main() {
    // Inject Git Hash
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output();
    let git_hash = match output {
        Ok(o) => String::from_utf8(o.stdout).unwrap_or("unknown".to_string()),
        Err(_) => "unknown".to_string(),
    };
    println!("cargo:rustc-env=APP_GIT_HASH={}", git_hash.trim());

    // Inject Build Time (UTC)
    // Using simple approach without external deps if possible, or fallback
    // Since we can't easily execute `date` on all platforms in build.rs without deps,
    // we'll rely on the commit date which is more stable.
    let output_date = Command::new("git")
        .args(&["show", "-s", "--format=%ci", "HEAD"])
        .output();
    let commit_date = match output_date {
        Ok(o) => String::from_utf8(o.stdout).unwrap_or("unknown".to_string()),
        Err(_) => "unknown".to_string(),
    };
    println!("cargo:rustc-env=APP_COMMIT_DATE={}", commit_date.trim());

    tauri_build::build()
}
