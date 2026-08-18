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

fn resolve_bin(root: &PathBuf, rel: &str) -> Option<PathBuf> {
    // first exact (dev/portable: root/bin/...)
    let primary = root.join("bin").join(rel);
    if primary.exists() {
        return Some(primary);
    }
    // Tauri installed: root/resources/bin (MSI/NSIS bundle.resources -> exe_parent/resources/bin)
    let res_root = root.join("resources").join("bin").join(rel);
    if res_root.exists() {
        return Some(res_root);
    }
    // try exe-relative (covers installed where exe != root)
    if let Ok(exe) = std::env::current_exe() {
        if let Some(par) = exe.parent() {
            // exe sibling: exe_dir/bin/...  and  exe_dir/resources/bin/...
            let sib = par.join("bin").join(rel);
            if sib.exists() {
                return Some(sib);
            }
            let res_sib = par.join("resources").join("bin").join(rel);
            if res_sib.exists() {
                return Some(res_sib);
            }
            // exe parent: parent/bin/...  and  parent/resources/bin/... (covers nested launchers)
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
    None
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

#[cfg(test)]
mod tests_resolve_bin {
    use super::*;
    use std::fs;

    #[test]
    fn test_resolve_bin_installed_resources_layout() {
        // Simulate Tauri MSI/NSIS installed layout:
        //   <install>/resources/bin/apache/bin/httpd.exe
        // where exe is <install>/Vanompp.exe  (so root = exe.parent())
        // Before fix, resolve_bin never checked exe.parent()/resources/bin -> FAIL
        let base = std::env::temp_dir().join(format!("vano_resolve_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        // Installed structure: base/resources/bin/apache/bin/httpd.exe
        let httpd = base.join("resources").join("bin").join("apache").join("bin").join("httpd.exe");
        fs::create_dir_all(httpd.parent().unwrap()).unwrap();
        fs::write(&httpd, b"fake").unwrap();
        // Also mysql at same layout
        let mysqld = base.join("resources").join("bin").join("mysql").join("bin").join("mysqld.exe");
        fs::create_dir_all(mysqld.parent().unwrap()).unwrap();
        fs::write(&mysqld, b"fake").unwrap();

        // root = install dir (where exe lives)
        let root = base.clone();
        // primary check inside resolve_bin: root/bin/apache/bin/httpd.exe does NOT exist
        assert!(!root.join("bin").join("apache/bin/httpd.exe").exists());
        // but root/resources/bin/apache/bin/httpd.exe DOES exist
        assert!(httpd.exists());

        // root/resources/bin layout is found via the resources/bin early-return in resolve_bin
        // Call via the public wrappers
        let got_a = resolve_apache_bin(&root);
        assert!(got_a.is_ok(), "httpd should be found in installed resources layout, got err: {:?}", got_a.err());
        let got_m = resolve_mysql_bin(&root);
        assert!(got_m.is_ok(), "mysqld should be found in installed resources layout, got err: {:?}", got_m.err());

        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn test_resolve_bin_still_finds_dev_bin_layout() {
        // Dev/portable: root/bin/apache/bin/httpd.exe  should still work (no regression)
        let base = std::env::temp_dir().join(format!("vano_resolve_dev_{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let httpd = base.join("bin").join("apache").join("bin").join("httpd.exe");
        fs::create_dir_all(httpd.parent().unwrap()).unwrap();
        fs::write(&httpd, b"fake").unwrap();
        let got = resolve_apache_bin(&base);
        assert!(got.is_ok(), "dev layout still must work");
        let _ = fs::remove_dir_all(&base);
    }
}

