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
    if let Some(p) = sys.process(Pid::from_u32(pid)) {
        p.kill()
    } else {
        true
    }
}
pub fn kill_all(state: &ServiceState) {
    if let Ok(map) = state.childs.lock() {
        for (_, pid) in map.iter() {
            let _ = kill_pid(*pid);
            #[cfg(target_os = "windows")]
            {
                let _ = std::process::Command::new("taskkill")
                    .args(["/PID", &pid.to_string(), "/F", "/T"])
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
        let start = all.len().saturating_sub(lines);
        all[start..].join("\n")
    } else {
        String::new()
    }
}

// ponytail-shrink: was 80 lines walking 20+ candidates. Prod reality: 2 layouts.
// 1) Portable: C:/Vanompp/bin/apache/bin/httpd.exe  (root/bin/...)
// 2) Dev: D:/Vanompp/src-tauri/resources/bin/...    (exe parent walk 1-2 levels)
// 3) Cur exe parent for cargo run.
pub fn resolve_bin(root: &std::path::PathBuf, rel: &str) -> Option<std::path::PathBuf> {
    let rel_p = std::path::PathBuf::from(rel);
    let mut cand = Vec::with_capacity(8);
    cand.push(root.join("bin").join(&rel_p));
    cand.push(root.join("resources").join("bin").join(&rel_p));
    cand.push(root.join("src-tauri").join("resources").join("bin").join(&rel_p));
    if let Ok(exe) = std::env::current_exe() {
        if let Some(p) = exe.parent() {
            cand.push(p.join("bin").join(&rel_p));
            cand.push(p.join("resources").join("bin").join(&rel_p));
            if let Some(pp) = p.parent() {
                cand.push(pp.join("resources").join("bin").join(&rel_p));
            }
        }
    }
    if let Ok(m) = std::env::var("CARGO_MANIFEST_DIR") {
        cand.push(std::path::PathBuf::from(m).join("resources").join("bin").join(&rel_p));
    }
    for c in cand {
        if c.exists() {
            return Some(c);
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
