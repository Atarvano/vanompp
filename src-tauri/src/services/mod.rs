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

impl Clone for ServiceState {
    fn clone(&self) -> Self {
        let map = self.childs.lock().map(|g| g.clone()).unwrap_or_default();
        Self {
            childs: Mutex::new(map),
        }
    }
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

pub fn find_process_pid_by_name(substr: &str) -> Option<u32> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let low = substr.to_lowercase();
    for (pid, proc) in sys.processes() {
        let name = proc.name().to_lowercase();
        if name.contains(&low) {
            return Some(pid.as_u32());
        }
    }
    None
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
}

fn try_paths(root: &PathBuf, rels: &[&str]) -> Option<PathBuf> {
    for rel in rels {
        let p = root.join(rel);
        if p.exists() {
            return Some(p);
        }
        // also try parent variants
        for i in 0..4 {
            let mut cur = root.clone();
            for _ in 0..i {
                if let Some(par) = cur.parent() {
                    cur = par.to_path_buf();
                }
            }
            let pp = cur.join(rel);
            if pp.exists() {
                return Some(pp);
            }
        }
    }
    None
}

fn resolve_bin(root: &PathBuf, rel: &str) -> Option<PathBuf> {
    // first exact
    let primary = root.join("bin").join(rel);
    if primary.exists() {
        return Some(primary);
    }
    // try exe sibling bin
    if let Ok(exe) = std::env::current_exe() {
        if let Some(par) = exe.parent() {
            let sib = par.join("bin").join(rel);
            if sib.exists() {
                return Some(sib);
            }
            // exe parent parent bin
            if let Some(pp) = par.parent() {
                let sib2 = pp.join("bin").join(rel);
                if sib2.exists() {
                    return Some(sib2);
                }
                let res = pp.join("resources").join("bin").join(rel);
                if res.exists() {
                    return Some(res);
                }
            }
        }
    }
    // try src-tauri/resources/bin
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        let mp = PathBuf::from(manifest);
        let res = mp.join("resources").join("bin").join(rel);
        if res.exists() {
            return Some(res);
        }
        if let Some(parent) = mp.parent() {
            let p2 = parent.join("bin").join(rel);
            if p2.exists() {
                return Some(p2);
            }
            let p3 = parent.join("src-tauri").join("resources").join("bin").join(rel);
            if p3.exists() {
                return Some(p3);
            }
        }
    }
    try_paths(root, &[&format!("bin/{}", rel), rel])
        .or_else(|| try_paths(root, &[rel]))
}

pub fn resolve_apache_bin(root: &PathBuf) -> Result<PathBuf, String> {
    resolve_bin(root, "apache/bin/httpd.exe")
        .or_else(|| resolve_bin(root, "apache/bin/httpd"))
        .ok_or_else(|| "httpd.exe tidak ketemu 😅 pastikan bin/apache ada".to_string())
}

pub fn resolve_mysql_bin(root: &PathBuf) -> Result<PathBuf, String> {
    resolve_bin(root, "mysql/bin/mysqld.exe")
        .or_else(|| resolve_bin(root, "mysql/bin/mysqld"))
        .ok_or_else(|| "mysqld.exe tidak ketemu 😅 pastikan bin/mysql ada".to_string())
}

pub fn resolve_mysql_client_bin(root: &PathBuf) -> Result<PathBuf, String> {
    resolve_bin(root, "mysql/bin/mysql.exe")
        .or_else(|| resolve_bin(root, "mysql/bin/mysql"))
        .ok_or_else(|| "mysql.exe tidak ketemu 😅 pastikan bin/mysql ada".to_string())
}

// dedup apache/mysql 5x parent() chain
pub(crate) fn layout_from_exe(exe: &Path) -> (PathBuf, PathBuf, PathBuf) {
    let bin_dir = exe.parent().unwrap_or(exe).to_path_buf();
    let svc_root = bin_dir.parent().unwrap_or(bin_dir.as_path()).to_path_buf();
    let bin_root = svc_root.parent().unwrap_or(svc_root.as_path()).to_path_buf();
    let app_root = bin_root.parent().unwrap_or(bin_root.as_path()).to_path_buf();
    (svc_root, bin_root, app_root)
}

pub(crate) fn service_root_from_exe(exe: &Path) -> PathBuf {
    layout_from_exe(exe).0
}

pub fn read_tail(path: &Path, max_lines: usize) -> String {
    let Ok(content) = std::fs::read_to_string(path) else {
        return String::new();
    };
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() <= max_lines {
        return content;
    }
    lines[lines.len() - max_lines..].join("\n")
}
