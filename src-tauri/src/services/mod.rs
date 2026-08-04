pub mod apache;
pub mod db;
pub mod mysql;

use std::collections::HashMap;
use std::sync::Mutex;
use sysinfo::{Pid, System};

#[derive(Debug, Default)]
pub struct ServiceState {
    pub childs: Mutex<HashMap<String, u32>>,
}

impl ServiceState {
    pub fn new() -> Self {
        Self {
            childs: Mutex::new(HashMap::new()),
        }
    }
}

pub fn is_pid_alive(pid: u32) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.process(Pid::from_u32(pid)).is_some()
}

pub fn kill_pid(pid: u32) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();
    if let Some(proc_) = sys.process(Pid::from_u32(pid)) {
        // sysinfo kill attempts graceful then kill
        proc_.kill()
    } else {
        // Already dead = considered killed
        true
    }
}

/// Kill all pids in state map, clear map
pub fn kill_all(state: &ServiceState) {
    if let Ok(map) = state.childs.lock() {
        for (_, pid) in map.iter() {
            let _ = kill_pid(*pid);
            // Fallback taskkill for stubborn apache/mysql on Windows
            #[cfg(target_os = "windows")]
            {
                let _ = std::process::Command::new("taskkill")
                    .arg("/PID")
                    .arg(pid.to_string())
                    .arg("/F")
                    .arg("/T")
                    .output();
            }
        }
    }
    if let Ok(mut map) = state.childs.lock() {
        map.clear();
    }
}

pub fn read_tail(path: &std::path::Path, lines: usize) -> String {
    if let Ok(content) = std::fs::read_to_string(path) {
        let all: Vec<&str> = content.lines().collect();
        let start = if all.len() > lines { all.len() - lines } else { 0 };
        all[start..].join("\n")
    } else {
        String::new()
    }
}

/// Resolve bin path trying several candidates near app root
pub fn resolve_bin(root: &std::path::PathBuf, rel_inside_bin: &str) -> Option<std::path::PathBuf> {
    let rel = std::path::PathBuf::from(rel_inside_bin);
    let mut candidates: Vec<std::path::PathBuf> = Vec::new();

    // Direct from given root
    candidates.push(root.join("bin").join(&rel));
    candidates.push(root.join("resources").join("bin").join(&rel));
    candidates.push(root.join("src-tauri").join("resources").join("bin").join(&rel));

    // CARGO_MANIFEST_DIR for dev
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        let mp = std::path::PathBuf::from(manifest);
        candidates.push(mp.join("resources").join("bin").join(&rel));
        candidates.push(mp.join("resources/bin").join(&rel));
        // Also parent of manifest is project root
        if let Some(parent) = mp.parent() {
            candidates.push(parent.join("src-tauri").join("resources").join("bin").join(&rel));
        }
    }

    // exe parent walk
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            candidates.push(parent.join("bin").join(&rel));
            candidates.push(parent.join("resources").join("bin").join(&rel));
            let mut cur = parent.to_path_buf();
            for _ in 0..5 {
                candidates.push(cur.join("resources").join("bin").join(&rel));
                candidates.push(cur.join("src-tauri").join("resources").join("bin").join(&rel));
                candidates.push(cur.join("bin").join(&rel));
                // also directly bin/<rel> from cur's parent layouts
                candidates.push(cur.join("bin").join(&rel));
                if !cur.pop() {
                    break;
                }
            }
        }
    }

    // current dir walk
    if let Ok(cwd) = std::env::current_dir() {
        let mut cur = cwd.clone();
        for _ in 0..4 {
            candidates.push(cur.join("bin").join(&rel));
            candidates.push(cur.join("resources").join("bin").join(&rel));
            candidates.push(cur.join("src-tauri").join("resources").join("bin").join(&rel));
            if !cur.pop() {
                break;
            }
        }
    }

    // Deduplicate and check exists
    for p in candidates {
        if p.exists() {
            return Some(p);
        }
    }
    None
}

pub fn resolve_apache_bin(root: &std::path::PathBuf) -> Result<std::path::PathBuf, String> {
    resolve_bin(root, "apache/bin/httpd.exe")
        .or_else(|| resolve_bin(root, "apache/bin/httpd"))
        .ok_or_else(|| "httpd.exe tidak ketemu 😅 pastikan bin/apache ada".to_string())
}

pub fn resolve_mysql_bin(root: &std::path::PathBuf) -> Result<std::path::PathBuf, String> {
    resolve_bin(root, "mysql/bin/mysqld.exe")
        .or_else(|| resolve_bin(root, "mysql/bin/mysqld"))
        .ok_or_else(|| "mysqld.exe tidak ketemu 😅 pastikan bin/mysql ada".to_string())
}

pub fn resolve_mysql_client_bin(root: &std::path::PathBuf) -> Result<std::path::PathBuf, String> {
    resolve_bin(root, "mysql/bin/mysql.exe")
        .or_else(|| resolve_bin(root, "mysql/bin/mysql"))
        .ok_or_else(|| "mysql.exe tidak ketemu 😅 pastikan bin/mysql ada".to_string())
}
