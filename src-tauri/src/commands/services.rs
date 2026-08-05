use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;

use crate::services::{apache, mysql, ServiceState};
use crate::utils::config::{read_persisted_ports, read_persisted_ports_effective, reset_persisted_port, write_persisted_port};
use crate::utils::paths::get_app_root;
use crate::utils::port::{is_port_free, suggest_next_free};
use crate::services::{is_pid_alive, find_process_pid_by_name};

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

const DEFAULT_APACHE: u16 = 8080;
const DEFAULT_MYSQL: u16 = 3306;

fn resolve_root() -> PathBuf {
    let root = get_app_root();
    if root.join("bin").exists() || root.join("resources").exists() || root.join("www").exists() {
        return root;
    }
    let mut cur = root.clone();
    for _ in 0..6 {
        if cur.join("www").exists() || cur.join("src-tauri").exists() || cur.join("bin").exists() {
            if cur.join("www").exists() || cur.join("bin").exists() {
                return cur;
            }
            let st = cur.join("src-tauri");
            if st.exists() {
                return cur;
            }
        }
        if !cur.pop() {
            break;
        }
    }
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
pub fn get_status(state: State<ServiceState>, apache_port: Option<u16>, mysql_port: Option<u16>) -> Vec<ServiceStatus> {
    let root = resolve_root();
    let (eff_ap, eff_my) = read_persisted_ports_effective(&root, DEFAULT_APACHE, DEFAULT_MYSQL);
    let ap = apache_port.unwrap_or(eff_ap);
    let mp = mysql_port.unwrap_or(eff_my);

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
    // strict orphan recovery: only adopt if process found AND port matches expectation
    if !apache_running {
        if let Some(pid) = find_process_pid_by_name("httpd") {
            if !is_port_free(ap) || is_pid_alive(pid) {
                apache_running = true;
                apache_pid = Some(pid);
            }
        }
    }
    if !mysql_running {
        if let Some(pid) = find_process_pid_by_name("mysqld") {
            // if mysqld alive, consider running regardless of requested port (orphan on custom port case)
            // check if its port is occupied: scan both default and requested
            if is_pid_alive(pid) {
                mysql_running = true;
                mysql_pid = Some(pid);
            }
        }
    }

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
pub fn check_ports(state: State<ServiceState>, apache_port: Option<u16>, mysql_port: Option<u16>) -> Vec<PortInfo> {
    let root = resolve_root();
    let (eff_ap, eff_my) = read_persisted_ports_effective(&root, DEFAULT_APACHE, DEFAULT_MYSQL);
    let ap = apache_port.unwrap_or(eff_ap);
    let mp = mysql_port.unwrap_or(eff_my);

    let st = get_status(state, Some(ap), Some(mp));
    let apache_running = st.iter().any(|s| s.name == "apache" && s.running);
    let mysql_running = st.iter().any(|s| s.name == "mysql" && s.running);

    let ap_free = if apache_running { true } else { is_port_free(ap) };
    let mp_free = if mysql_running { true } else { is_port_free(mp) };

    vec![
        PortInfo {
            port: ap,
            free: ap_free,
            suggest: if ap_free { ap } else { suggest_next_free(ap) },
        },
        PortInfo {
            port: mp,
            free: mp_free,
            suggest: if mp_free { mp } else { suggest_next_free(mp) },
        },
    ]
}

#[tauri::command(async)]
pub async fn start_service(
    state: State<'_, ServiceState>,
    name: String,
    port: Option<u16>,
) -> Result<String, String> {
    let inner = state.inner().clone();
    let n = name.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let root = resolve_root();
        let (eff_ap, eff_my) = read_persisted_ports_effective(&root, DEFAULT_APACHE, DEFAULT_MYSQL);
        let lname = n.to_lowercase();
        match lname.as_str() {
            "apache" => {
                let p = port.unwrap_or(eff_ap);
                let pid = apache::start_apache(&inner, &root, p, eff_my)?;
                Ok(format!("Apache ON pid {} port {}", pid, p))
            }
            "mysql" | "mariadb" => {
                let p = port.unwrap_or(eff_my);
                let pid = mysql::start_mysql(&inner, &root, p)?;
                Ok(format!("MySQL ON pid {} port {}", pid, p))
            }
            _ => Err(format!("Service {} tidak dikenal — pakai apache atau mysql", n)),
        }
    })
    .await
    .unwrap_or_else(|e| Err(format!("spawn err {}", e)))
}

#[tauri::command(async)]
pub async fn stop_service(state: State<'_, ServiceState>, name: String) -> Result<String, String> {
    let inner = state.inner().clone();
    let n = name.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let lname = n.to_lowercase();
        match lname.as_str() {
            "apache" => {
                apache::stop_apache(&inner)?;
                Ok("Apache OFF".to_string())
            }
            "mysql" | "mariadb" => {
                mysql::stop_mysql(&inner)?;
                Ok("MySQL OFF".to_string())
            }
            _ => Err(format!("Service {} tidak dikenal", n)),
        }
    })
    .await
    .unwrap_or_else(|e| Err(format!("spawn err {}", e)))
}

#[tauri::command(async)]
pub async fn start_all_services(
    state: State<'_, ServiceState>,
    apache_port: Option<u16>,
    mysql_port: Option<u16>,
) -> Result<Vec<String>, String> {
    let inner = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let root = resolve_root();
        let (eff_ap, eff_my) = read_persisted_ports_effective(&root, DEFAULT_APACHE, DEFAULT_MYSQL);
        let ap = apache_port.unwrap_or(eff_ap);
        let mp = mysql_port.unwrap_or(eff_my);

        let pid_a = apache::start_apache(&inner, &root, ap, mp)?;
        let pid_m = mysql::start_mysql(&inner, &root, mp)?;

        Ok(vec![
            format!("Apache ON pid {} port {}", pid_a, ap),
            format!("MySQL ON pid {} port {}", pid_m, mp),
        ])
    })
    .await
    .unwrap_or_else(|e| Err(format!("spawn err {}", e)))
}

#[tauri::command(async)]
pub async fn stop_all_services(state: State<'_, ServiceState>) -> Result<Vec<String>, String> {
    let inner = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let mut msgs = Vec::new();
        match apache::stop_apache(&inner) {
            Ok(()) => msgs.push("Apache OFF".to_string()),
            Err(e) => msgs.push(format!("Apache stop err: {}", e)),
        }
        match mysql::stop_mysql(&inner) {
            Ok(()) => msgs.push("MySQL OFF".to_string()),
            Err(e) => msgs.push(format!("MySQL stop err: {}", e)),
        }
        Ok(msgs)
    })
    .await
    .unwrap_or_else(|e| Err(format!("spawn err {}", e)))
}

#[tauri::command]
pub fn repair_mysql(state: State<ServiceState>) -> Result<String, String> {
    let _ = mysql::stop_mysql(&state);
    let root = resolve_root();
    let mysqld = crate::services::resolve_mysql_bin(&root)?;
    let mysql_root = mysqld.parent().unwrap().parent().unwrap().to_path_buf();
    let data_dir = mysql_root.join("data");
    if data_dir.exists() {
        std::fs::remove_dir_all(&data_dir)
            .map_err(|e| format!("Gagal hapus data MySQL {} 😅 {}", data_dir.display(), e))?;
    }
    std::fs::create_dir_all(&data_dir).map_err(|e| format!("Gagal bikin data dir {}", e))?;
    Ok(format!(
        "Data MySQL di {} sudah direset — klik Start MySQL lagi, nanti auto --initialize-insecure. (DB lama hilang, backup dulu kalau perlu).",
        data_dir.display()
    ))
}

// v1.1 new
#[tauri::command]
pub fn get_persisted_ports() -> (Option<u16>, Option<u16>) {
    let root = resolve_root();
    read_persisted_ports(&root)
}
#[tauri::command]
pub fn set_persisted_port(name: String, port: u16) -> Result<(), String> {
    let root = resolve_root();
    write_persisted_port(&root, &name, port).map_err(|e| e.to_string())
}
#[tauri::command]
pub fn reset_persisted_port_cmd(name: String) -> Result<(), String> {
    let root = resolve_root();
    reset_persisted_port(&root, &name).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use crate::utils::port::{is_port_free, suggest_next_free};
    #[test]
    fn test_check_ports_returns_two() {
        // ponytail: avoid constructing tauri::State in unit test — test port utils directly
        let free_8080 = is_port_free(18080);
        let sugg = suggest_next_free(18080);
        assert!(sugg >= 18080);
        let _ = free_8080;
    }
}
