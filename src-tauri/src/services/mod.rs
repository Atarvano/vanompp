pub mod apache;
pub mod db;
pub mod mysql;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
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

// ---------- Task 18: shared layout helpers (ponytail) ----------
// Dedup of apache.rs / mysql.rs pattern:
// exe = .../bin/<service>/bin/<binary.exe>
// bin_dir      = exe.parent()               = .../bin/<service>/bin
// service_root = bin_dir.parent()           = .../bin/<service>
// bin_root     = service_root.parent()      = .../bin or .../resources/bin
// app_root     = bin_root.parent()          = C:/Vanompp or D:/Vanompp or .../resources
// root_forward = app_root forward slashes for template {{ROOT}}

/// Given resolved exe path, return (service_root, bin_root, app_root)
/// service_root = .../bin/<service>, bin_root = .../bin, app_root = parent of bin_root
/// All unwraps safe-guarded — fallback to exe ancestor if parent missing.
pub(crate) fn layout_from_exe(exe: &Path) -> (PathBuf, PathBuf, PathBuf) {
    let bin_dir = exe.parent().unwrap_or(exe).to_path_buf();
    let svc_root = bin_dir.parent().unwrap_or(bin_dir.as_path()).to_path_buf();
    let bin_root = svc_root.parent().unwrap_or(svc_root.as_path()).to_path_buf();
    let app_root = bin_root.parent().unwrap_or(bin_root.as_path()).to_path_buf();
    (svc_root, bin_root, app_root)
}

// ponytail: sugar helpers — no extra abstraction, just unwrap tuple to call sites
pub(crate) fn service_root_from_exe(exe: &Path) -> PathBuf {
    layout_from_exe(exe).0
}

pub(crate) fn app_root_from_exe(exe: &Path) -> PathBuf {
    layout_from_exe(exe).2
}

/// Forward-slash string for template {{ROOT}} from an app_root path.
pub(crate) fn fwd_from_app_root(app_root: &Path) -> String {
    crate::conf::root_forward(&app_root.to_path_buf())
}

/// Compute template fwd from optional exe. If exe present, use its app_root,
/// else fallback to root param (original behavior preserved).
pub(crate) fn template_root_forward(root: &PathBuf, exe_opt: Option<&PathBuf>) -> String {
    if let Some(exe) = exe_opt {
        let (_, _, app_root) = layout_from_exe(exe);
        fwd_from_app_root(&app_root)
    } else {
        crate::conf::root_forward(root)
    }
}

/// Required dir — error with label preserved (emoji messages kept in callers via label).
pub(crate) fn ensure_dir_required(p: &Path, label: &str) -> Result<(), String> {
    std::fs::create_dir_all(p).map_err(|e| format!("{}: {}", label, e))
}

/// Best-effort batch — ignore errors (original ensure_dirs behavior).
pub(crate) fn ensure_dirs_best_effort(paths: &[PathBuf]) {
    for p in paths {
        let _ = std::fs::create_dir_all(p);
    }
}
