use std::path::PathBuf;
use std::time::Duration;

use crate::conf::{render_httpd, render_phpini, render_phpmyadmin, root_forward};
use crate::services::{is_pid_alive, kill_pid, read_tail, resolve_apache_bin, ServiceState};
use crate::utils::port::{is_port_free, is_port_occupied_tcp, suggest_next_free};

pub fn render_conf(root: &PathBuf, apache_port: u16, mysql_port: u16) -> Result<PathBuf, String> {
    // Resolve real bin first to compute correct ROOT for template (dev vs portable)
    let apache_exe = resolve_apache_bin(root).ok();

    // ponytail: reuse layout_from_exe instead of manual parent chain dupe
    let (_apache_root_for_fwd, bin_root_for_fwd): (Option<PathBuf>, Option<PathBuf>) = if let Some(exe) = apache_exe.as_ref() {
        let (svc_root, bin_root, _app_root) = crate::services::layout_from_exe(exe.as_path());
        (Some(svc_root), Some(bin_root))
    } else {
        (None, None)
    };

    // ROOT for template = bin_root parent (C:/Vanompp or .../resources)
    let (fwd, www_fwd) = {
        let root_for_template = if let Some(br) = bin_root_for_fwd.as_ref() {
            br.parent().unwrap_or(br.as_path()).to_path_buf()
        } else {
            root.clone()
        };
        let f = root_forward(&root_for_template);
        // www is always actual project www, not resources/www which doesn't exist in dev
        // Try root/www first (project root), then resources/www, then root_for_template/www fallback
        let www_candidate = if root.join("www").exists() {
            root.join("www")
        } else if root_for_template.join("www").exists() {
            root_for_template.join("www")
        } else {
            // For dev: project root is parent of src-tauri; www at D:/Vanompp/www
            // root param is D:/Vanompp, so root/www exists — already handled. Fallback create.
            root_for_template.join("www")
        };
        let wf = root_forward(&www_candidate);
        (f, wf)
    };

    let rendered = render_httpd(&fwd, apache_port, &www_fwd);
    let php_rendered = render_phpini(&fwd);
    let pma_rendered = render_phpmyadmin(&fwd, mysql_port);

    // Resolve real bin dirs (might be resources/bin layout)
    let apache_conf_dir_primary = root.join("bin").join("apache").join("conf");
    let php_dir_primary = root.join("bin").join("php");

    let (apache_conf_path, php_ini_path, apache_logs_dir, php_tmp, php_logs, phpmyadmin_conf_path): (
        PathBuf,
        PathBuf,
        PathBuf,
        PathBuf,
        PathBuf,
        PathBuf,
    // ponytail: same layout_from_exe — removes 3x parent chain dupe
    ) = if let Some(exe_path) = apache_exe.as_ref() {
        let (apache_root, bin_root, _app_root) = crate::services::layout_from_exe(exe_path.as_path());
        let conf_dir = apache_root.join("conf");
        let logs_dir = apache_root.join("logs");
        let php_dir = bin_root.join("php");
        let pma_dir = bin_root.join("phpmyadmin");
        (
            conf_dir.join("httpd-vano.conf"),
            php_dir.join("php.ini"),
            logs_dir,
            php_dir.join("tmp"),
            php_dir.join("logs"),
            pma_dir.join("config.inc.php"),
        )
    } else {
        (
            apache_conf_dir_primary.join("httpd-vano.conf"),
            php_dir_primary.join("php.ini"),
            root.join("bin").join("apache").join("logs"),
            root.join("bin").join("php").join("tmp"),
            root.join("bin").join("php").join("logs"),
            root.join("bin").join("phpmyadmin").join("config.inc.php"),
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
    // phpmyadmin config — best effort, don't fail if can't write
    if let Some(parent) = phpmyadmin_conf_path.parent() {
        if std::fs::create_dir_all(parent).is_ok() {
            let _ = std::fs::write(&phpmyadmin_conf_path, pma_rendered);
            // also ensure tmp dir for pma
            let _ = std::fs::create_dir_all(parent.join("tmp"));
        }
    }

    // Ensure www exists
    let www = root.join("www");
    if !www.exists() {
        // Try also locate via alternate resolution? best effort
        // If root doesn't contain www, try fallback for dev: walk up for www folder
        // Don't fail — just try to create
        let _ = std::fs::create_dir_all(&www);
    }

    Ok(apache_conf_path)
}

/// Start Apache, return pid or Err with Indonesian message
/// mysql_port used for phpMyAdmin config generation (default 3306 if 0 passed)
pub fn start_apache(
    state: &ServiceState,
    root: &PathBuf,
    port: u16,
    mysql_port: u16,
) -> Result<u32, String> {
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

    let mp = if mysql_port == 0 { 3306 } else { mysql_port };
    let conf_path = render_conf(root, port, mp)?;

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

    // Config syntax check first — gives better error than empty error.log
    // httpd.exe often writes syntax errors to stderr, not error.log, if ServerRoot invalid.
    {
        let mut check_cmd = std::process::Command::new(&httpd_path);
        check_cmd.arg("-t").arg("-f").arg(&conf_path);
        if let Some(apache_root) = httpd_path.parent().and_then(|p| p.parent()) {
            check_cmd.current_dir(apache_root);
        }
        if let Ok(out) = check_cmd.output() {
            let stderr = String::from_utf8_lossy(&out.stderr);
            let stdout = String::from_utf8_lossy(&out.stdout);
            let combined = format!("{}{}", stdout, stderr);
            if !out.status.success() {
                // Show config error directly — this is why error.log empty
                return Err(format!(
                    "Apache config error 😅 Syntax check gagal:\n{}\nConf: {}\nCoba klik Repair atau cek VC++ redist (php8apache2_4.dll butuh).",
                    combined.chars().take(800).collect::<String>(),
                    conf_path.display()
                ));
            }
        }
    }

    // Spawn httpd
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
            let tail = read_tail(&error_log_path, 30);
            let access_exists = logs_dir.join("access.log").exists();
            if let Ok(mut map) = state.childs.lock() {
                map.remove("apache");
            }
            if tail.is_empty() {
                // Try to get config check again for better msg
                let mut check_cmd = std::process::Command::new(&httpd_path);
                check_cmd.arg("-t").arg("-f").arg(&conf_path);
                if let Some(ar) = httpd_path.parent().and_then(|p| p.parent()) {
                    check_cmd.current_dir(ar);
                }
                let cfg_msg = if let Ok(out) = check_cmd.output() {
                    let s = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
                    if !s.trim().is_empty() { s.chars().take(600).collect::<String>() } else { String::new() }
                } else { String::new() };
                if !cfg_msg.is_empty() {
                    return Err(format!("Apache gagal start 😅 Syntax:\n{}\nConf: {}\nLog empty (access log exists? {}). Cek VC++ redist & php dll.", cfg_msg, conf_path.display(), access_exists));
                }
                return Err(format!(
                    "Apache gagal start 😅 httpd.exe keluar tanpa log (log empty). Conf: {}\nKemungkinan: VC++ redist belum install (butuh untuk php8apache2_4.dll), atau port {} dipakai, atau www folder hilang. Coba: 1) Install VC++ 2015-2022 x64, 2) Klik Repair, 3) Cek Task Manager httpd.",
                    conf_path.display(), port
                ));
            } else {
                return Err(format!("Apache gagal start 😅 Cek error.log:\n{}", tail));
            }
        }

        // If port is now occupied, apache bound successfully
        if !is_port_free(port) {
            started = true;
            break;
        }

        // Also try tcp connect optimistic
        if is_port_occupied_tcp(port) {
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
