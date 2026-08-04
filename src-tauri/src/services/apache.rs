use std::path::PathBuf;
use std::time::Duration;

use crate::conf::{render_httpd, render_phpini, root_forward};
use crate::services::{is_pid_alive, kill_pid, read_tail, resolve_apache_bin, ServiceState};
use crate::utils::port::{is_port_free, suggest_next_free};

fn ensure_dirs(root: &PathBuf) -> Result<(), String> {
    let rf = root_forward(root);
    let _ = rf; // ensure root_forward used
    let dirs = [
        root.join("bin").join("apache").join("conf"),
        root.join("bin").join("apache").join("logs"),
        root.join("bin").join("php").join("tmp"),
        root.join("bin").join("php").join("logs"),
        root.join("bin").join("mysql").join("data").join("tmp"),
    ];
    // Also try resources/bin variants if primary doesn't exist — but we ensure inside resolved locations?
    // Simpler: ensure via resolved apache_root / php_root detection if needed.
    for d in dirs.iter() {
        if let Err(e) = std::fs::create_dir_all(d) {
            // Only error if path is expected primary and parent exists-ish, else ignore for now
            // But keep error for troubleshooting
            // We'll try best-effort: if root/bin/apache doesn't exist we will later resolve actual location
            // So don't fail hard here unless it's about php/tmp inside resources/bin fallback
            let _ = e;
        }
    }
    Ok(())
}

pub fn render_conf(root: &PathBuf, apache_port: u16) -> Result<PathBuf, String> {
    let fwd = root_forward(root);
    let rendered = render_httpd(&fwd, apache_port);
    let php_rendered = render_phpini(&fwd);

    // Resolve real bin dirs (might be resources/bin layout)
    // Try to find actual apache conf directory — prefer resolved location if exists
    let apache_conf_dir_primary = root.join("bin").join("apache").join("conf");
    let php_dir_primary = root.join("bin").join("php");

    // Discover actual base bin root:
    // If primary exists, use it; else try to locate via resolve_bin using httpd.exe location
    let apache_exe = resolve_apache_bin(root).ok();
    let (apache_conf_path, php_ini_path, apache_logs_dir, php_tmp, php_logs): (PathBuf, PathBuf, PathBuf, PathBuf, PathBuf) =
        if let Some(exe_path) = apache_exe.as_ref() {
            // exe_path = .../bin/apache/bin/httpd.exe -> apache root = .../bin/apache
            let bin_dir = exe_path
                .parent()
                .unwrap_or(exe_path.as_path())
                .to_path_buf();
            let apache_root = bin_dir.parent().unwrap_or(bin_dir.as_path()).to_path_buf();
            let conf_dir = apache_root.join("conf");
            let logs_dir = apache_root.join("logs");
            // php is sibling of apache root's parent: .../bin/php
            let bin_root = apache_root.parent().unwrap_or(apache_root.as_path()).to_path_buf();
            let php_dir = bin_root.join("php");
            (
                conf_dir.join("httpd-vano.conf"),
                php_dir.join("php.ini"),
                logs_dir,
                php_dir.join("tmp"),
                php_dir.join("logs"),
            )
        } else {
            // Fallback primary paths
            (
                apache_conf_dir_primary.join("httpd-vano.conf"),
                php_dir_primary.join("php.ini"),
                root.join("bin").join("apache").join("logs"),
                root.join("bin").join("php").join("tmp"),
                root.join("bin").join("php").join("logs"),
            )
        };

    // Also ensure alternative dirs exist
    // Ensure parent directories
    if let Some(parent) = apache_conf_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Gagal bikin folder conf apache: {}", e))?;
    }
    std::fs::create_dir_all(&apache_logs_dir)
        .map_err(|e| format!("Gagal bikin folder logs apache: {}", e))?;

    // php.ini: ensure php parent exists, plus tmp/logs
    if let Some(parent) = php_ini_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Gagal bikin folder php: {}", e))?;
    }
    std::fs::create_dir_all(&php_tmp).map_err(|e| format!("Gagal bikin tmp php: {}", e))?;
    std::fs::create_dir_all(&php_logs).map_err(|e| format!("Gagal bikin logs php: {}", e))?;

    std::fs::write(&apache_conf_path, rendered)
        .map_err(|e| format!("Gagal tulis httpd-vano.conf: {}", e))?;
    std::fs::write(&php_ini_path, php_rendered)
        .map_err(|e| format!("Gagal tulis php.ini: {}", e))?;

    // Ensure www exists
    let www = root.join("www");
    if !www.exists() {
        // Try also locate via alternate resolution? best effort
        // If root doesn't contain www, try fallback for dev: walk up for www folder
        // Don't fail — just try to create
        let _ = std::fs::create_dir_all(&www);
    }

    // Also ensure parent resource copy for httpd-vano if running from src-tauri/resources layout:
    // If we wrote to resources/bin/... that's fine already; if we wrote to root/bin/..., also try to keep resources version in sync?
    // YAGNI — skip duplication

    ensure_dirs(root)?;

    Ok(apache_conf_path)
}

/// Start Apache, return pid or Err with Indonesian message
pub fn start_apache(state: &ServiceState, root: &PathBuf, port: u16) -> Result<u32, String> {
    // If already running according to state, check if still alive
    if let Ok(map) = state.childs.lock() {
        if let Some(pid) = map.get("apache") {
            if is_pid_alive(*pid) {
                return Ok(*pid);
            }
        }
    }

    if !is_port_free(port) {
        let sugg = suggest_next_free(port + 1);
        return Err(format!(
            "Port {} udah dipakai 😅 Coba {}",
            port, sugg
        ));
    }

    let conf_path = render_conf(root, port)?;

    let httpd_path = resolve_apache_bin(root)?;

    // Ensure logs files exist so tail works even before first run
    let logs_dir = httpd_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("logs");
    let _ = std::fs::create_dir_all(&logs_dir);
    let error_log_path = logs_dir.join("error.log");

    // Spawn httpd
    // ApacheLounge typical: httpd.exe -f "<confPath>"  (no -k start needed if using -f alone as service?
    // Spec says -f confPath -d root. Using -f is enough, it will run foreground if no -k.
    // We'll use -f with absolute path. Also pass -d if needed.
    #[allow(unused_mut)]
    let mut cmd = std::process::Command::new(&httpd_path);
    cmd.arg("-f").arg(&conf_path);

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    // Working dir = apache root
    if let Some(apache_root) = httpd_path.parent().and_then(|p| p.parent()) {
        cmd.current_dir(apache_root);
    }

    let child = cmd
        .spawn()
        .map_err(|e| format!("Gagal start Apache 😅 {} — cek apakah VC++ redist terinstall?", e))?;

    let pid = child.id();

    // Store pid
    if let Ok(mut map) = state.childs.lock() {
        map.insert("apache".to_string(), pid);
    }

    // Do NOT wait on child handle — we stored pid and let it run detached
    // Poll for port becoming busy or child exit
    let mut started = false;
    for _ in 0..20 {
        std::thread::sleep(Duration::from_millis(500));

        // Check if pid still alive
        if !is_pid_alive(pid) {
            // Read error log tail
            let tail = read_tail(&error_log_path, 20);
            // Clean map
            if let Ok(mut map) = state.childs.lock() {
                map.remove("apache");
            }
            if tail.is_empty() {
                return Err(
                    "Apache gagal start 😅 Cek error.log — httpd.exe keluar tanpa log".to_string(),
                );
            } else {
                return Err(format!(
                    "Apache gagal start 😅 Cek error.log:\n{}",
                    tail
                ));
            }
        }

        // If port is now occupied, apache bound successfully
        if !is_port_free(port) {
            started = true;
            break;
        }

        // Also try tcp connect optimistic
        if std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok() {
            started = true;
            break;
        }
    }

    if !started {
        // Try kill dangling?
        // Don't auto-kill yet, but read log
        let tail = read_tail(&error_log_path, 20);
        // If pid still alive but port free after 10 sec → likely config error
        // Kill it to avoid orphan
        let _ = kill_pid(pid);
        if let Ok(mut map) = state.childs.lock() {
            map.remove("apache");
        }
        if tail.is_empty() {
            return Err(format!(
                "Apache gagal start 😅 Port {} masih free setelah 10 detik, mungkin config error. Cek {}",
                port,
                error_log_path.display()
            ));
        } else {
            return Err(format!(
                "Apache gagal start 😅 Cek error.log\n{}",
                tail
            ));
        }
    }

    Ok(pid)
}

pub fn stop_apache(state: &ServiceState) -> Result<(), String> {
    let pid_opt = {
        if let Ok(map) = state.childs.lock() {
            map.get("apache").cloned()
        } else {
            None
        }
    };

    if let Some(pid) = pid_opt {
        // First try graceful kill via sysinfo
        let killed = kill_pid(pid);
        // Fallback taskkill /F /T on Windows
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("taskkill")
                .arg("/PID")
                .arg(pid.to_string())
                .arg("/F")
                .arg("/T")
                .output();
        }

        // Wait a bit for process to die
        for _ in 0..10 {
            if !is_pid_alive(pid) {
                break;
            }
            std::thread::sleep(Duration::from_millis(300));
        }

        if let Ok(mut map) = state.childs.lock() {
            map.remove("apache");
        }

        if !killed {
            // If kill returned false but map cleared, still ok? Check liveness
            if is_pid_alive(pid) {
                return Err(format!("Gagal stop Apache pid {} — coba manual taskkill", pid));
            }
        }
        Ok(())
    } else {
        // No pid in map — try to find httpd by name and kill? YAGNI — just return ok
        Ok(())
    }
}
