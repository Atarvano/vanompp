use std::path::PathBuf;
use std::time::Duration;

use crate::conf::{render_myini, root_forward};
use crate::services::{is_pid_alive, kill_pid, read_tail, resolve_mysql_bin, ServiceState};
use crate::utils::port::{is_port_free, suggest_next_free};

fn ensure_mysql_dirs(data_dir: &PathBuf, logs_tmp: &PathBuf) -> Result<(), String> {
    std::fs::create_dir_all(data_dir).map_err(|e| format!("Gagal bikin data dir mysql: {}", e))?;
    std::fs::create_dir_all(logs_tmp).map_err(|e| format!("Gagal bikin tmp mysql: {}", e))?;
    Ok(())
}

/// Render my.ini to appropriate location, init data dir if needed
pub fn render_myini_file(root: &PathBuf, mysql_port: u16) -> Result<PathBuf, String> {
    let fwd = root_forward(root);
    let rendered = render_myini(&fwd, mysql_port);

    // Resolve actual mysql bin location
    let mysqld_path = resolve_mysql_bin(root).ok();

    let (my_ini_path, data_dir, tmp_dir): (PathBuf, PathBuf, PathBuf) = if let Some(exe_path) = mysqld_path.as_ref() {
        let bin_dir = exe_path.parent().unwrap().to_path_buf();
        let mysql_root = bin_dir.parent().unwrap().to_path_buf(); // .../bin/mysql
        let data = mysql_root.join("data");
        let tmp = data.join("tmp");
        let ini = mysql_root.join("my.ini");
        (ini, data, tmp)
    } else {
        let mysql_root = root.join("bin").join("mysql");
        (
            mysql_root.join("my.ini"),
            mysql_root.join("data"),
            mysql_root.join("data").join("tmp"),
        )
    };

    ensure_mysql_dirs(&data_dir, &tmp_dir)?;

    std::fs::write(&my_ini_path, rendered).map_err(|e| format!("Gagal tulis my.ini: {}", e))?;

    Ok(my_ini_path)
}

/// Check if data dir is initialized (has mysql subfolder or ibdata)
pub fn is_data_initialized(data_dir: &PathBuf) -> bool {
    if !data_dir.exists() {
        return false;
    }
    // Heuristic: presence of mysql folder or ibdata1 or data file
    data_dir.join("mysql").exists()
        || data_dir.join("ibdata1").exists()
        || data_dir.join("ib_buffer_pool").exists()
        || std::fs::read_dir(data_dir)
            .map(|mut rd| rd.next().is_some())
            .unwrap_or(false)
}

fn run_initialize_insecure(mysqld_path: &PathBuf, data_dir: &PathBuf, mysql_root: &PathBuf) -> Result<(), String> {
    // If already initialized, skip
    // For dev safety: if data_dir has files > 0 and mysql folder exists, skip
    if is_data_initialized(data_dir) && data_dir.join("mysql").exists() {
        return Ok(());
    }

    // Ensure dirs
    let _ = std::fs::create_dir_all(data_dir);

    #[allow(unused_mut)]
    let mut cmd = std::process::Command::new(mysqld_path);
    cmd.arg("--initialize-insecure")
        .arg(format!("--datadir={}", data_dir.display()))
        .arg(format!("--basedir={}", mysql_root.display()));

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let output = cmd.output().map_err(|e| {
        format!(
            "Gagal init data MySQL 😅 {} — cek permission folder data",
            e
        )
    })?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        // If data already initialized, mysql returns error but folder exists — treat as ok if mysql folder exists after attempt
        if is_data_initialized(data_dir) && data_dir.join("mysql").exists() {
            return Ok(());
        }
        return Err(format!(
            "Gagal init data MySQL 😅\nstdout: {}\nstderr: {}",
            stdout.chars().take(500).collect::<String>(),
            stderr.chars().take(1000).collect::<String>()
        ));
    }
    Ok(())
}

pub fn start_mysql(state: &ServiceState, root: &PathBuf, port: u16) -> Result<u32, String> {
    if let Ok(map) = state.childs.lock() {
        if let Some(pid) = map.get("mysql") {
            if is_pid_alive(*pid) {
                return Ok(*pid);
            }
        }
    }

    if !is_port_free(port) {
        let sugg = suggest_next_free(port + 1);
        return Err(format!("Port {} udah dipakai 😅 Coba {}", port, sugg));
    }

    let my_ini_path = render_myini_file(root, port)?;
    let mysqld_path = resolve_mysql_bin(root)?;

    let mysql_root = mysqld_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let data_dir = mysql_root.join("data");

    // Initialize if empty
    if !is_data_initialized(&data_dir) || !data_dir.join("mysql").exists() {
        // Attempt init
        if let Err(e) = run_initialize_insecure(&mysqld_path, &data_dir, &mysql_root) {
            return Err(e);
        }
    }

    // error log path from template
    let error_log_path = data_dir.join("mysql_error.log");

    #[allow(unused_mut)]
    let mut cmd = std::process::Command::new(&mysqld_path);
    // Use defaults-file - this is important for portable
    cmd.arg(format!("--defaults-file={}", my_ini_path.display()));

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    cmd.current_dir(&mysql_root);

    let child = cmd.spawn().map_err(|e| {
        format!(
            "Gagal start MySQL 😅 {} — cek apakah data folder corrupt? coba hapus data/* kecuali my.ini lalu restart",
            e
        )
    })?;

    let pid = child.id();

    if let Ok(mut map) = state.childs.lock() {
        map.insert("mysql".to_string(), pid);
    }

    // Poll
    let mut started = false;
    for _ in 0..20 {
        std::thread::sleep(Duration::from_millis(500));

        if !is_pid_alive(pid) {
            let tail = read_tail(&error_log_path, 30);
            if let Ok(mut map) = state.childs.lock() {
                map.remove("mysql");
            }
            if tail.is_empty() {
                return Err("MySQL gagal start 😅 Cek mysql_error.log — mysqld keluar".to_string());
            } else {
                return Err(format!("MySQL gagal start 😅 Cek mysql_error.log:\n{}", tail));
            }
        }

        if !is_port_free(port) {
            started = true;
            break;
        }

        if std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok() {
            started = true;
            break;
        }
    }

    if !started {
        let tail = read_tail(&error_log_path, 30);
        let _ = kill_pid(pid);
        if let Ok(mut map) = state.childs.lock() {
            map.remove("mysql");
        }
        if tail.is_empty() {
            return Err(format!(
                "MySQL gagal start 😅 Port {} masih free setelah 10 detik",
                port
            ));
        } else {
            return Err(format!("MySQL gagal start 😅 Cek mysql_error.log:\n{}", tail));
        }
    }

    Ok(pid)
}

pub fn stop_mysql(state: &ServiceState) -> Result<(), String> {
    let pid_opt = if let Ok(map) = state.childs.lock() {
        map.get("mysql").cloned()
    } else {
        None
    };

    if let Some(pid) = pid_opt {
        let _ = kill_pid(pid);
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("taskkill")
                .arg("/PID")
                .arg(pid.to_string())
                .arg("/F")
                .arg("/T")
                .output();
        }
        for _ in 0..10 {
            if !is_pid_alive(pid) {
                break;
            }
            std::thread::sleep(Duration::from_millis(300));
        }
        if let Ok(mut map) = state.childs.lock() {
            map.remove("mysql");
        }
        if is_pid_alive(pid) {
            return Err(format!("Gagal stop MySQL pid {} — coba taskkill manual", pid));
        }
        Ok(())
    } else {
        Ok(())
    }
}
