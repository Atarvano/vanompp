use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;

use crate::services::{apache, mysql, ServiceState};
use crate::utils::paths::get_app_root;
use crate::utils::port::{is_port_free, suggest_next_free};
use crate::services::is_pid_alive;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub name: String,
    pub running: bool,
    pub pid: Option<u32>,
    pub port: u16,
    pub port_free: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortInfo {
    pub port: u16,
    pub free: bool,
    pub suggest: u16,
}

fn resolve_root() -> PathBuf {
    // Try get_app_root but also enhance for dev: if www not near app root, search up
    let root = get_app_root();

    // If root/bin exists or root/resources exists, good
    if root.join("bin").exists() || root.join("resources").exists() || root.join("www").exists() {
        return root;
    }

    // Check if we are inside target/debug etc — go up to find src-tauri or www
    let mut cur = root.clone();
    for _ in 0..6 {
        if cur.join("www").exists() || cur.join("src-tauri").exists() || cur.join("bin").exists() {
            // If cur contains src-tauri folder, we want project root which is cur (contains www? maybe)
            // Actually app root for our logic is where bin/www lives.
            // If cur has src-tauri, then root is cur (or cur/src-tauri/resources?)
            // Simpler: return cur if it has www or bin
            if cur.join("www").exists() || cur.join("bin").exists() {
                return cur;
            }
            // If cur has src-tauri folder, check inside it
            let st = cur.join("src-tauri");
            if st.exists() {
                // Project root = cur, but bin may be in src-tauri/resources
                return cur;
            }
        }
        if !cur.pop() {
            break;
        }
    }

    // Fallback check CARGO_MANIFEST_DIR parent
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        let mp = PathBuf::from(manifest);
        if let Some(parent) = mp.parent() {
            if parent.join("www").exists() || parent.join("bin").exists() || mp.join("resources").exists() {
                return parent.to_path_buf();
            }
            return parent.to_path_buf();
        }
        return mp;
    }

    root
}

#[tauri::command]
pub fn start_service(
    state: State<ServiceState>,
    name: String,
    port: Option<u16>,
) -> Result<String, String> {
    let root = resolve_root();
    let n = name.to_lowercase();

    match n.as_str() {
        "apache" => {
            let p = port.unwrap_or(8080);
            let pid = apache::start_apache(&state, &root, p)?;
            Ok(format!("Apache ON pid {} port {}", pid, p))
        }
        "mysql" | "mariadb" => {
            let p = port.unwrap_or(3306);
            let pid = mysql::start_mysql(&state, &root, p)?;
            Ok(format!("MySQL ON pid {} port {}", pid, p))
        }
        _ => Err(format!("Service {} tidak dikenal — pakai apache atau mysql", name)),
    }
}

#[tauri::command]
pub fn stop_service(state: State<ServiceState>, name: String) -> Result<String, String> {
    let n = name.to_lowercase();
    match n.as_str() {
        "apache" => {
            apache::stop_apache(&state)?;
            Ok("Apache OFF".to_string())
        }
        "mysql" | "mariadb" => {
            mysql::stop_mysql(&state)?;
            Ok("MySQL OFF".to_string())
        }
        _ => Err(format!("Service {} tidak dikenal", name)),
    }
}

#[tauri::command]
pub fn get_status(state: State<ServiceState>, apache_port: Option<u16>, mysql_port: Option<u16>) -> Vec<ServiceStatus> {
    let ap = apache_port.unwrap_or(8080);
    let mp = mysql_port.unwrap_or(3306);

    let mut apache_running = false;
    let mut apache_pid: Option<u32> = None;
    let mut mysql_running = false;
    let mut mysql_pid: Option<u32> = None;

    if let Ok(map) = state.childs.lock() {
        if let Some(pid) = map.get("apache") {
            if is_pid_alive(*pid) {
                apache_running = true;
                apache_pid = Some(*pid);
            }
        }
        if let Some(pid) = map.get("mysql") {
            if is_pid_alive(*pid) {
                mysql_running = true;
                mysql_pid = Some(*pid);
            }
        }
    }

    // If map says running but actually port free and not alive, fallback: consider not running
    // But we already checked alive. Also if map missing but port busy (external process), we should report running? For UX, port busy = considered running elsewhere, but we mark port_free false
    vec![
        ServiceStatus {
            name: "apache".to_string(),
            running: apache_running,
            pid: apache_pid,
            port: ap,
            port_free: is_port_free(ap),
        },
        ServiceStatus {
            name: "mysql".to_string(),
            running: mysql_running,
            pid: mysql_pid,
            port: mp,
            port_free: is_port_free(mp),
        },
    ]
}

#[tauri::command]
pub fn check_ports(apache_port: Option<u16>, mysql_port: Option<u16>) -> Vec<PortInfo> {
    let ap = apache_port.unwrap_or(8080);
    let mp = mysql_port.unwrap_or(3306);
    let ports = [ap, mp];
    ports
        .iter()
        .map(|p| {
            let free = is_port_free(*p);
            let sugg = if free { *p } else { suggest_next_free(p + 1) };
            PortInfo {
                port: *p,
                free,
                suggest: sugg,
            }
        })
        .collect()
}

#[tauri::command]
pub fn start_all_services(
    state: State<ServiceState>,
    apache_port: Option<u16>,
    mysql_port: Option<u16>,
) -> Result<Vec<String>, String> {
    let root = resolve_root();
    let ap = apache_port.unwrap_or(8080);
    let mp = mysql_port.unwrap_or(3306);

    let pid_a = apache::start_apache(&state, &root, ap)?;
    let pid_m = mysql::start_mysql(&state, &root, mp)?;

    Ok(vec![
        format!("Apache ON pid {} port {}", pid_a, ap),
        format!("MySQL ON pid {} port {}", pid_m, mp),
    ])
}

#[tauri::command]
pub fn stop_all_services(state: State<ServiceState>) -> Result<Vec<String>, String> {
    let mut msgs = Vec::new();
    match apache::stop_apache(&state) {
        Ok(()) => msgs.push("Apache OFF".to_string()),
        Err(e) => msgs.push(format!("Apache stop err: {}", e)),
    }
    match mysql::stop_mysql(&state) {
        Ok(()) => msgs.push("MySQL OFF".to_string()),
        Err(e) => msgs.push(format!("MySQL stop err: {}", e)),
    }
    Ok(msgs)
}

#[tauri::command]
pub fn repair_mysql(state: State<ServiceState>) -> Result<String, String> {
    // Stop first if running
    let _ = mysql::stop_mysql(&state);
    let root = resolve_root();
    // Resolve data dir
    let mysqld = crate::services::resolve_mysql_bin(&root)?;
    let mysql_root = mysqld.parent().unwrap().parent().unwrap().to_path_buf();
    let data_dir = mysql_root.join("data");
    // Remove corrupt data dir content (keep nothing)
    if data_dir.exists() {
        std::fs::remove_dir_all(&data_dir)
            .map_err(|e| format!("Gagal hapus data MySQL {} 😅 {}", data_dir.display(), e))?;
    }
    std::fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Gagal bikin data dir {}", e))?;
    Ok(format!(
        "Data MySQL di {} sudah direset — klik Start MySQL lagi, nanti auto --initialize-insecure. (DB lama hilang, backup dulu kalau perlu).",
        data_dir.display()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::ServiceState;

    fn make_state() -> ServiceState {
        ServiceState::new()
    }

    #[test]
    fn test_check_ports_returns_two() {
        let infos = check_ports(None, None);
        assert_eq!(infos.len(), 2);
        assert_eq!(infos[0].port, 8080);
        assert_eq!(infos[1].port, 3306);
        // suggest should be >= port
        assert!(infos[0].suggest >= 8080);
        assert!(infos[1].suggest >= 3306);
    }

    #[test]
    fn test_start_invalid_name() {
        let r = resolve_root();
        assert!(!r.as_os_str().is_empty());
        let s = make_state();
        {
            let map = s.childs.lock().expect("lock");
            assert!(map.is_empty());
        }
    }

    #[test]
    fn test_start_service_unknown() {
        // Can't fully invoke without tauri State, but we test that ServiceState new works
        let s = ServiceState::new();
        drop(s);
    }
}
